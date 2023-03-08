use crate::game::types::gear::{Item, Slot};
use crate::gui::widgets::optional_switch::OptionalBool;
use egui::Ui;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, PartialEq, Default)]
pub struct ItemTags {
    pub dragon_amulet: bool,
    pub dragon_coins: bool,
    pub temporary: bool,
    pub rare: bool,
    pub war: bool,
    pub special_offer: bool,
}

impl ToString for ItemTags {
    fn to_string(&self) -> String {
        let mut result = Vec::new();

        if self.dragon_amulet {
            result.push("Dragon Amulet");
        }
        if self.dragon_coins {
            result.push("Dragon Coins");
        }
        if self.temporary {
            result.push("Temporary");
        }
        if self.rare {
            result.push("Rare");
        }
        if self.war {
            result.push("War");
        }
        if self.special_offer {
            result.push("Special Offer");
        }

        result.join(", ")
    }
}

#[derive(Default)]
pub struct ItemFilters {
    pub dragon_coins: OptionalBool,
    pub dragon_amulet: OptionalBool,
    pub temporary: OptionalBool,
    pub rare: OptionalBool,
    pub war: OptionalBool,
    pub special_offer: OptionalBool,
}

impl ItemFilters {
    pub fn allow(&self, tags: &ItemTags) -> bool {
        if let Some(val) = self.dragon_coins.as_option() {
            if tags.dragon_coins != val {
                return false;
            }
        }
        if let Some(val) = self.dragon_amulet.as_option() {
            if tags.dragon_amulet != val {
                return false;
            }
        }
        if let Some(val) = self.temporary.as_option() {
            if tags.temporary != val {
                return false;
            }
        }
        if let Some(val) = self.rare.as_option() {
            if tags.rare != val {
                return false;
            }
        }
        if let Some(val) = self.war.as_option() {
            if tags.war != val {
                return false;
            }
        }
        if let Some(val) = self.special_offer.as_option() {
            if tags.special_offer != val {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GuiItem {
    pub base: Item,

    pub name: String,
    pub pedia_page_id: u32,

    pub weapon_type: Option<String>,
    pub tags: ItemTags,
}

#[derive(Debug, Clone)]
pub struct GearSet {
    pub set: HashMap<Slot, GuiItem>,
    pub name: String,
}

impl PartialEq for GearSet {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl GearSet {
    pub fn new(name: String) -> Self {
        Self {
            set: HashMap::new(),
            name,
        }
    }

    pub fn draw(&self, ui: &mut Ui) {
        egui::Grid::new("gear_set").num_columns(2).show(ui, |ui| {
            for slot in Slot::vec() {
                ui.label(slot.to_string());

                let item_name = if let Some(item) = self.set.get(&slot) {
                    item.name.clone()
                } else {
                    "None".to_string()
                };

                ui.label(item_name);

                ui.end_row();
            }
        });
    }
}
