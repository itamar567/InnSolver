use crate::game::types::gear::Slot;
use crate::gui::gear::GuiItem;
use crate::gui::options::{AIOptions, GameOptions, GearOptions};
use crate::gui::views::ai_view::AIView;
use crate::gui::views::gear_view::GearView;
use crate::gui::views::interactive_view::InteractiveView;
use crate::gui::views::side_panel_view::SidePanelView;
use eframe::Frame;
use egui::{Color32, Context, FontFamily, FontId, TextStyle, Ui};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

#[derive(Copy, Clone)]
pub enum AppView {
    AI,
    Interactive,
    Gear,
}

impl Default for AppView {
    fn default() -> Self {
        Self::AI
    }
}

pub struct App {
    view: AppView,
    ai_view: AIView,
    interactive_view: InteractiveView,
    gear_view: GearView,

    side_panel: SidePanelView,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.configure_panels();

        egui::SidePanel::right("configuration_panel")
            .resizable(false)
            .show(ctx, |ui| {
                self.draw_side_panel(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_central_panel(ui);
        });
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        // Configure style
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = BTreeMap::from([
            (TextStyle::Small, FontId::new(9.0, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(15.0, FontFamily::Proportional)),
            (
                TextStyle::Monospace,
                FontId::new(12.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Button,
                FontId::new(15.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Heading,
                FontId::new(18.0, FontFamily::Proportional),
            ),
        ]);
        style.visuals.override_text_color = Some(Color32::WHITE);
        cc.egui_ctx.set_style(style);

        let game_options = Rc::new(RefCell::new(GameOptions::default()));
        let ai_options = Rc::new(RefCell::new(AIOptions::default()));
        let gear_options = Rc::new(RefCell::new(GearOptions::default()));

        Self {
            view: AppView::default(),
            ai_view: AIView::new(game_options.clone(), ai_options.clone()),
            interactive_view: InteractiveView::new(game_options.clone()),
            gear_view: GearView::new(Self::get_gear(), gear_options.clone()),
            side_panel: SidePanelView::new(game_options, ai_options, gear_options),
        }
    }

    fn draw_side_panel(&mut self, ui: &mut Ui) {
        self.side_panel.draw(ui);
        ui.separator();
    }

    fn draw_central_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui
                .selectable_label(matches!(self.view, AppView::AI), "AI")
                .clicked()
            {
                self.view = AppView::AI;
            }
            if ui
                .selectable_label(matches!(self.view, AppView::Interactive), "Interactive")
                .clicked()
            {
                self.view = AppView::Interactive;
            }
            if ui
                .selectable_label(matches!(self.view, AppView::Gear), "Gear")
                .clicked()
            {
                self.view = AppView::Gear;
            }
        });

        ui.separator();

        match self.view {
            AppView::AI => self.ai_view.draw(ui),
            AppView::Interactive => self.interactive_view.draw(ui),
            AppView::Gear => self.gear_view.draw(ui),
        };
    }

    fn configure_panels(&mut self) {
        if let Some(view) = self.side_panel.change_view {
            self.view = view;
            self.side_panel.change_view = None;
        }

        self.side_panel.set_current_view(self.view);
    }

    pub fn get_gear() -> HashMap<Slot, Vec<GuiItem>> {
        let data = include_str!("../../res/gear.json");

        serde_json::from_str(data).expect("Failed to parse items")
    }
}
