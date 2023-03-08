use crate::game::types::gear::Slot;
use crate::gui::gear::{GuiItem, ItemFilters};
use crate::gui::options::GearOptions;
use crate::gui::utils;
use crate::gui::widgets::optional_switch::switch;
use egui::{Align, Layout, TextStyle, Ui, Vec2};
use egui_extras::Column;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct GearView {
    max_items: usize,
    gear_options: Rc<RefCell<GearOptions>>,
    all_items: HashMap<Slot, Vec<GuiItem>>,
    item_list: Vec<GuiItem>,
    current_slot: Slot,

    current_item: Option<GuiItem>,

    pub search_string: String,
    pub sort_string: String,
    pub filters: ItemFilters,
}

impl GearView {
    pub fn new(gear: HashMap<Slot, Vec<GuiItem>>, gear_options: Rc<RefCell<GearOptions>>) -> Self {
        let slot = gear_options.borrow().slot;

        let mut result = Self {
            max_items: 100,
            current_slot: slot,
            gear_options,
            all_items: gear,
            item_list: Vec::new(),
            current_item: None,
            search_string: String::new(),
            sort_string: String::new(),
            filters: ItemFilters::default(),
        };
        result.update_item_list();

        result
    }

    pub fn update_item_list(&mut self) {
        let items_for_slot = self
            .all_items
            .get(&self.gear_options.borrow().slot)
            .unwrap_or(&Vec::new())
            .clone();

        let mut item_list = Vec::new();

        for item in items_for_slot {
            if item
                .name
                .to_lowercase()
                .contains(&self.search_string.to_lowercase())
                && self.filters.allow(&item.tags)
            {
                item_list.push(item);
            }
        }

        self.item_list = item_list;
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        if self.current_slot != self.gear_options.borrow().slot {
            // The slot was changed from the side panel, we need to update the item list
            self.update_item_list();
        }

        ui.with_layout(
            Layout::left_to_right(Align::default()).with_cross_justify(true),
            |ui| {
                let small_panel_size = Vec2::new(ui.available_width() / 4.0, ui.available_height());
                let large_panel_size = Vec2::new(ui.available_width() / 3.0, ui.available_height());

                ui.allocate_ui(small_panel_size, |ui| {
                    ui.vertical(|ui| {
                        self.draw_left_panel(ui);
                    });
                });

                ui.separator();

                ui.allocate_ui(large_panel_size, |ui| {
                    ui.vertical(|ui| {
                        self.draw_middle_panel(ui);
                    });
                });

                ui.separator();

                ui.allocate_ui(small_panel_size, |ui| {
                    ui.vertical(|ui| {
                        self.draw_right_panel(ui);
                    });
                });
            },
        );
    }

    fn draw_left_panel(&mut self, ui: &mut Ui) {
        ui.heading("Search: ");
        if ui.text_edit_singleline(&mut self.search_string).changed() {
            self.update_item_list();
        }

        ui.separator();

        ui.heading("Sort by: ");
        if ui.text_edit_singleline(&mut self.sort_string).changed() {
            self.update_item_list();
        }

        ui.separator();

        ui.heading("Filter by tags:");
        egui::Grid::new("gear_view_filters_grid").show(ui, |ui| {
            ui.label("Dragon Amulet");
            if switch(ui, &mut self.filters.dragon_amulet).changed() {
                self.update_item_list();
            }
            ui.end_row();

            ui.label("Dragon Coins");
            if switch(ui, &mut self.filters.dragon_coins).changed() {
                self.update_item_list();
            }
            ui.end_row();

            ui.label("Rare");
            if switch(ui, &mut self.filters.rare).changed() {
                self.update_item_list();
            }
            ui.end_row();

            ui.label("Temporary");
            if switch(ui, &mut self.filters.temporary).changed() {
                self.update_item_list();
            }
            ui.end_row();

            ui.label("Special Offer");
            if switch(ui, &mut self.filters.special_offer).changed() {
                self.update_item_list();
            }
            ui.end_row();

            ui.label("War");
            if switch(ui, &mut self.filters.war).changed() {
                self.update_item_list();
            }
        });
    }

    fn draw_middle_panel(&mut self, ui: &mut Ui) {
        if self.item_list.is_empty() {
            ui.allocate_space(ui.available_size());

            return;
        }

        let text_size = ui
            .ctx()
            .style()
            .text_styles
            .get(&TextStyle::Body)
            .unwrap()
            .size
            + 3.0;
        let width = ui.available_width();
        egui_extras::TableBuilder::new(ui)
            .striped(true)
            .auto_shrink([true, false])
            .column(Column::exact(width * 0.7))
            .column(Column::remainder())
            .body(|mut body| {
                let mut index = 0;
                for item in &self.item_list {
                    index += 1;
                    if index > self.max_items {
                        body.row(text_size, |mut row| {
                            row.col(|ui| {
                                if ui.button("Show more").clicked() {
                                    self.max_items += 100;
                                }
                            });
                        });

                        break;
                    }

                    body.row(text_size, |mut row| {
                        row.col(|ui| {
                            if ui
                                .selectable_label(
                                    self.current_item == Some(item.clone()),
                                    &item.name,
                                )
                                .clicked()
                            {
                                self.current_item = Some(item.clone());
                            }
                        });
                        row.col(|ui| {
                            if ui
                                .add_enabled(
                                    self.gear_options.borrow().current_set_index.is_some(),
                                    egui::Button::new("Equip"),
                                )
                                .clicked()
                            {
                                let slot = self.gear_options.borrow().slot;
                                self.gear_options
                                    .borrow_mut()
                                    .get_current_set_mut()
                                    .unwrap()
                                    .set
                                    .insert(slot, item.clone());
                            }
                        });
                    });
                }
            });
    }

    fn draw_right_panel(&mut self, ui: &mut Ui) {
        if let Some(item) = &self.current_item {
            ui.heading(&item.name);
            ui.separator();

            let size_of_column = ui.available_width() / 3.0;
            egui::Grid::new("gear_view_show_item_grid")
                .num_columns(2)
                .min_col_width(size_of_column)
                .max_col_width(size_of_column)
                .show(ui, |ui| {
                    if !item.base.bonuses.is_empty() {
                        ui.label("Bonuses");
                    }

                    if !item.base.resists.is_empty() {
                        ui.label("Resists");
                    }
                    ui.end_row();

                    if !item.base.bonuses.is_empty() {
                        utils::draw_dict(ui, &item.base.bonuses, "bonuses".to_string());
                    }
                    if !item.base.resists.is_empty() {
                        utils::draw_dict(ui, &item.base.resists, "resists".to_string());
                    }
                });
            ui.separator();

            let tags_string = &item.tags.to_string();
            if !tags_string.is_empty() {
                ui.heading("Tags");
                ui.label(tags_string);
                ui.separator();
            }
        }
    }
}
