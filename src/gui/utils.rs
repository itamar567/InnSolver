use crate::game::types::dict::Dict;
use egui::Ui;

pub fn draw_dict(ui: &mut Ui, dict: &Dict, id_str: String) {
    let size_of_column = ui.available_width() / 2.0;

    egui::Grid::new(id_str)
        .num_columns(2)
        .min_col_width(size_of_column)
        .max_col_width(size_of_column)
        .striped(true)
        .show(ui, |ui| {
            for (index, (key, value)) in dict.iter_sorted().iter().enumerate() {
                if index != 0 {
                    ui.end_row();
                }
                ui.label(format!("{}:", to_title_case(key)));
                ui.label(value.to_string());
            }
        });
}

fn to_title_case(key: &str) -> String {
    let chars = key.chars();
    let mut result = String::new();
    let mut start_of_word = true;

    for mut char in chars {
        if start_of_word {
            char = char.to_uppercase().next().unwrap();
        }

        if char == '_' {
            char = ' ';
        }
        start_of_word = char == ' ';

        result.push(char);
    }

    result
}
