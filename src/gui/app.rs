use crate::gui::options::{AIOptions, GameOptions};
use crate::gui::views::ai_view::AIView;
use crate::gui::views::interactive_view::InteractiveView;
use crate::gui::views::side_panel_view::SidePanelView;
use eframe::Frame;
use egui::{Color32, Context, FontFamily, FontId, TextStyle, Ui};
use std::collections::BTreeMap;

enum AppView {
    AIAppView,
    InteractiveAppView,
}

pub struct App {
    game_options: GameOptions,
    ai_options: AIOptions,

    view: AppView,
    ai_view: AIView,
    interactive_view: InteractiveView,

    side_panel: SidePanelView,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::SidePanel::right("configuration_panel").show(ctx, |ui| {
            self.draw_side_panel(ui);
        });

        self.configure_panels();

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_central_panel(ui);
        });
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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

        Self {
            game_options: GameOptions::default(),
            ai_options: AIOptions::default(),
            view: AppView::AIAppView,
            ai_view: AIView::new(),
            interactive_view: InteractiveView::new(),
            side_panel: SidePanelView::new(),
        }
    }

    fn draw_side_panel(&mut self, ui: &mut Ui) {
        self.side_panel.draw(ui);
        ui.separator();
    }

    fn draw_central_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui
                .selectable_label(matches!(self.view, AppView::AIAppView), "AI")
                .clicked()
            {
                self.view = AppView::AIAppView;
            }
            if ui
                .selectable_label(
                    matches!(self.view, AppView::InteractiveAppView),
                    "Interactive",
                )
                .clicked()
            {
                self.view = AppView::InteractiveAppView;
            }
        });

        ui.separator();

        match self.view {
            AppView::AIAppView => self.ai_view.draw(ui),
            AppView::InteractiveAppView => self.interactive_view.draw(ui),
        };
    }

    fn configure_panels(&mut self) {
        let options = self.side_panel.get_options();
        self.game_options = options.0;
        self.ai_options = options.1;

        self.ai_view
            .configure(self.game_options.clone(), self.ai_options.clone());
        self.interactive_view.configure(self.game_options.clone())
    }
}
