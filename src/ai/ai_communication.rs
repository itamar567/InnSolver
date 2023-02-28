use crate::ai::ai_thread::AIThread;
use crate::ai::types::skill_eval::SkillEval;
use crate::game::game_manager::GameManager;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::thread;

pub struct AICommunicationPacket {
    new_game: GameManager,
    skill_name: String,
}

pub struct AICommunicator {
    pub ai: AI,
    channel_recv: mpsc::Receiver<AICommunicationPacket>,
    channel_send: mpsc::Sender<AICommunicationPacket>,
}

impl AICommunicator {
    pub fn new(ai: AI) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            ai,
            channel_recv: rx,
            channel_send: tx,
        }
    }

    pub fn do_turn(&self) {
        let mut ai = self.ai.clone();
        let channel = self.channel_send.clone();
        thread::spawn(move || ai.do_best_skill(channel));
    }

    pub fn try_get_skill(&mut self) -> Result<String, TryRecvError> {
        let raw_packet = self.channel_recv.try_recv();

        if let Ok(packet) = raw_packet {
            self.ai.game = packet.new_game;
            Ok(packet.skill_name)
        } else {
            Err(raw_packet.err().unwrap())
        }
    }
}

#[derive(Clone)]
pub struct AI {
    pub game: GameManager,

    depth: u8,
}

impl AI {
    pub fn new(game: GameManager, depth: u8) -> Self {
        AI { game, depth }
    }

    fn get_best_skill(&self) -> SkillEval {
        let (tx, rx) = mpsc::channel();

        let available_skills = self
            .game
            .player
            .get_base_type()
            .as_player()
            .get_available_skills();

        // Spawn all threads
        for skill in available_skills.clone() {
            let current_tx = tx.clone();
            let mut current_game = self.game.clone();

            current_game
                .player
                .get_base_type_mut()
                .as_player()
                .set_current_skill(skill);
            current_game.do_turn();

            let ai_thread = AIThread::new(current_game, self.depth - 1);

            thread::spawn(move || current_tx.send(SkillEval::new(Some(skill), ai_thread.eval())));
        }

        // Gather results
        let mut result = SkillEval::lost();
        for _ in 0..available_skills.len() {
            let val = rx.recv().unwrap();

            if val > result {
                result = val;
            }
        }

        result
    }

    pub fn do_best_skill(&mut self, tx: mpsc::Sender<AICommunicationPacket>) {
        let best_skill = self.get_best_skill();

        let mut _player_ref = self.game.player.get_base_type_mut();
        let player = _player_ref.as_player();
        let skills = player.skills.clone();

        player.set_current_skill(best_skill.index.unwrap());
        self.game.do_turn();

        tx.send(AICommunicationPacket {
            new_game: self.game.clone(),
            skill_name: skills[best_skill.index.unwrap()].name.clone(),
        })
        .unwrap();
    }
}
