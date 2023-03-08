use egui::{Color32, Response, Ui};

#[derive(Debug)]
pub enum OptionalBool {
    Some(bool),
    None(bool),
}

impl Default for OptionalBool {
    fn default() -> Self {
        Self::None(false)
    }
}

impl OptionalBool {
    pub fn as_option(&self) -> Option<bool> {
        match self {
            OptionalBool::Some(x) => Some(*x),
            OptionalBool::None(_) => None,
        }
    }

    fn switch(&mut self) {
        *self = match self {
            OptionalBool::Some(x) => OptionalBool::None(!*x),
            OptionalBool::None(x) => OptionalBool::Some(*x),
        };
    }

    fn as_value(&self) -> f32 {
        if let Some(val) = self.as_option() {
            if val {
                return 1.0;
            }
            return 0.0;
        }

        0.5
    }
}

pub(crate) fn switch(ui: &mut Ui, value: &mut OptionalBool) -> Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.5, 1.25);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if response.clicked() {
        value.switch();
        response.mark_changed();
    }

    if ui.is_rect_visible(rect) {
        let animation_pos = ui
            .ctx()
            .animate_value_with_time(response.id, value.as_value(), 0.2);

        let mut visuals = *ui.style().interact(&response);
        if let Some(val) = value.as_option() {
            if val {
                visuals.bg_fill = Color32::DARK_GREEN;
            } else {
                visuals.bg_fill = Color32::DARK_RED;
            }
        }

        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp(
            (rect.left() + radius)..=(rect.right() - radius),
            animation_pos,
        );
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}
