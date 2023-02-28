use egui::{Response, Ui};

pub fn unsigned_number_input(ui: &mut Ui, text: &mut String, max: u32, min: u32) -> Response {
    let text_edit = ui.text_edit_singleline(text);
    if text_edit.lost_focus() {
        let mut new_text = text.clone();

        // Remove all chars that are not digits
        new_text = new_text.chars().filter(|c| c.is_ascii_digit()).collect();

        // Make sure we won't crash when we convert to u32
        if new_text.is_empty() {
            new_text.push('0');
        }
        new_text = new_text[0..new_text.len().min(9)].to_string();

        let mut num: u32 = new_text.parse().unwrap();
        if num > max {
            num = max;
        }
        if num < min {
            num = min;
        }

        text.clear();
        text.push_str(&num.to_string());
    }

    text_edit
}
