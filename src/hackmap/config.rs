use std::path::Path;
use std::io::Read;
use std::collections::HashSet;
use serde::Deserialize;
use super::common::*;
use D2Common::D2Unit;
use anyhow::Result;

use super::config_deserializer::*;

pub(super) type ConfigRef = Rc<RefCell<Config>>;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(i32)]
pub(super) enum DropNotify {
    None        = 0,
    Name        = 1,
    Property    = 2,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(i32)]
pub(super) enum PickupMethod {
    None        = 0,
    Inventory   = 1,
    Cube        = 2,
    AutoBelt    = 3,
}

#[derive(Debug, Deserialize)]
pub(super) struct HotKeyConfig {
    #[serde(default)]
    pub reload              : VirtualKeyCode,

    #[serde(default)]
    pub hide_items          : VirtualKeyCode,

    #[serde(default)]
    pub perm_show_items     : VirtualKeyCode,

    #[serde(default)]
    pub quick_next_game     : VirtualKeyCode,

    #[serde(default)]
    pub item_extra_info     : VirtualKeyCode,

    #[serde(default)]
    pub show_monster_id     : VirtualKeyCode,

    #[serde(default)]
    pub auto_pickup         : VirtualKeyCode,
}

#[derive(Debug, Deserialize)]
pub(super) struct TweaksConfig {
    #[serde(deserialize_with = "bool_from_int", default)]
    pub perm_show_items: bool,

    #[serde(deserialize_with = "bool_from_int", default)]
    pub show_monster_id: bool,

    #[serde(deserialize_with = "bool_from_int", default)]
    pub continue_attacking_after_target_dead: bool,

    #[serde(deserialize_with = "bool_from_int", default)]
    pub remove_shadow: bool,

    #[serde(default)]
    pub excluded_dc6: HashSet<String>,
}

#[derive(Debug, Deserialize)]
pub(super) struct UnitColorConfig {
    #[serde(deserialize_with = "bool_from_int", default)]
    pub show_socket_number          : bool,

    #[serde(deserialize_with = "bool_from_int", default)]
    pub hide_items                  : bool,

    #[serde(deserialize_with = "bool_from_int", default)]
    pub item_extra_info             : bool,

    #[serde(deserialize_with = "bool_from_int", default)]
    pub auto_pickup                 : bool,

    pub player_blob_file            : Option<String>,
    pub player_pet_blob_file        : Option<String>,
    pub monster_blob_file           : Option<String>,
    pub object_blob_file            : Option<String>,
    pub missile_blob_file           : Option<String>,
    pub item_blob_file              : Option<String>,
    pub boss_blob_file              : Option<String>,
    pub npc_blob_file               : Option<String>,
    pub my_blob_file                : Option<String>,
    pub my_pet_blob_file            : Option<String>,
    pub corpse_blob_file            : Option<String>,

    pub my_blob_color               : u8,
    pub my_pet_blob_color           : u8,
    pub party_blob_color            : u8,
    pub party_pet_blob_color        : u8,
    pub normal_monster_color        : u8,
    pub boss_monster_color          : u8,
    pub minion_monster_color        : u8,
    pub champion_monster_color      : u8,
    pub super_unique_color          : u8,

    #[serde(default)]
    pub player_missile_color        : u8,

    #[serde(default)]
    pub other_missile_color         : u8,

    pub magic_resistant_desc        : Option<String>,
    pub fire_enchanted_desc         : Option<String>,
    pub lightning_enchanted_desc    : Option<String>,
    pub cold_enchanted_desc         : Option<String>,
    pub mana_burn_desc              : Option<String>,

    pub physical_immunity_desc      : Option<String>,
    pub magic_immunity_desc         : Option<String>,
    pub fire_immunity_desc          : Option<String>,
    pub lightning_immunity_desc     : Option<String>,
    pub cold_immunity_desc          : Option<String>,
    pub poison_immunity_desc        : Option<String>,

    #[serde(deserialize_with = "deserialize_monster_color")]
    pub monster_color               : HashMap<u32, u8>,

    pub item_colors                 : Vec<ItemColor>,
}

#[derive(Debug, Deserialize)]
pub struct ItemColor {
    #[serde(rename = "id")]
    pub class_id: Option<u32>,

    #[serde(deserialize_with = "opt_bool_from_int", default)]
    pub match_name: Option<bool>,

    #[serde(rename = "prop")]
    pub property: Option<String>,

    pub regex: Option<RegexWrapper>,

    #[serde(deserialize_with = "opt_d2_str_color_code_from_int", default)]
    pub text_color: Option<D2StringColorCodes>,

    #[serde(deserialize_with = "opt_palette_from_int", default)]
    pub minimap_color: Option<u8>,

    #[serde(deserialize_with = "opt_d2_item_quality_from_str", default)]
    pub quality: Option<D2ItemQualities>,

    #[serde(deserialize_with = "opt_bool_from_int", default)]
    pub eth: Option<bool>,

    pub socks: Option<usize>,

    pub notify      : Option<DropNotify>,
    pub notify_text : Option<String>,
    pub pickup      : Option<PickupMethod>,
}

impl UnitColorConfig {
    pub fn get_color_from_unit(&self, item: &D2Unit) -> Option<&ItemColor> {
        let class_id = item.dwClassId;
        let quality = D2Common::Items::GetItemQuality(item);
        let socks_num = D2Common::StatList::GetUnitBaseStat(item, D2ItemStats::Item_NumSockets, 0);
        let is_eth = D2Common::Items::CheckItemFlag(item, D2ItemFlags::Ethereal) != FALSE;

        let item_prop = D2SigmaEx::Items::get_item_properties(item, false);
        let item_name = D2SigmaEx::Items::get_item_name(item);

        for entry in self.item_colors.iter().rev() {
            let match_name = entry.match_name.unwrap_or(false);

            if let Some(cid) = entry.class_id {
                if cid != class_id {
                    continue;
                }
            }

            if let Some(q) = entry.quality {
                if q != quality {
                    continue;
                }
            }

            if let Some(socks) = entry.socks {
                if socks != socks_num {
                    continue;
                }
            }

            if let Some(eth) = entry.eth {
                if eth != is_eth {
                    continue;
                }
            }

            if let Some(re) = entry.regex.as_ref() {
                let prop = if match_name { &item_name } else { &item_prop };
                if re.is_match(&prop) == false {
                    continue;
                }

            } else if let Some(prop) = entry.property.as_ref() {
                let t = if match_name { &item_name } else { &item_prop };
                if prop.is_empty() == false && t.contains(prop) == false {
                    continue;
                }
            }

            return Some(entry)
        }

        None
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct Config {
    pub hotkey      : HotKeyConfig,
    pub tweaks      : TweaksConfig,
    pub unit_color  : UnitColorConfig,
}

impl Config {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self{
            hotkey: HotKeyConfig{
                reload              : Default::default(),
                hide_items          : Default::default(),
                perm_show_items     : Default::default(),
                quick_next_game     : Default::default(),
                item_extra_info     : Default::default(),
                show_monster_id     : Default::default(),
                auto_pickup         : Default::default(),
            },

            tweaks: TweaksConfig{
                perm_show_items                     : true,
                show_monster_id                     : false,
                continue_attacking_after_target_dead: true,
                remove_shadow                       : true,
                excluded_dc6                        : HashSet::new(),
            },

            unit_color: UnitColorConfig{
                show_socket_number              : true,
                hide_items                      : true,
                item_extra_info                 : false,
                auto_pickup                     : false,

                player_blob_file                : None,
                player_pet_blob_file            : None,
                monster_blob_file               : None,
                object_blob_file                : None,
                missile_blob_file               : None,
                item_blob_file                  : None,
                boss_blob_file                  : None,
                npc_blob_file                   : None,
                my_blob_file                    : None,
                my_pet_blob_file                : None,
                corpse_blob_file                : None,

                magic_resistant_desc            : None,
                fire_enchanted_desc             : None,
                lightning_enchanted_desc        : None,
                cold_enchanted_desc             : None,
                mana_burn_desc                  : None,

                my_blob_color                   : 0x81,
                my_pet_blob_color               : 0x7F,
                party_blob_color                : 0x68,
                party_pet_blob_color            : 0x68,
                normal_monster_color            : 0xFF,
                boss_monster_color              : 0xFF,
                minion_monster_color            : 0xFF,
                champion_monster_color          : 0xFF,
                super_unique_color              : 0xFF,

                player_missile_color            : 0xFF,
                other_missile_color             : 0xFF,

                physical_immunity_desc          : None,
                magic_immunity_desc             : None,
                fire_immunity_desc              : None,
                lightning_immunity_desc         : None,
                cold_immunity_desc              : None,
                poison_immunity_desc            : None,

                monster_color                   : HashMap::new(),
                item_colors                     : vec![],
            }
        }))
    }

    pub fn load<P: AsRef<Path>>(&mut self, cfg_file: P) -> Result<()> {
        let mut fs = std::fs::File::open(cfg_file)?;
        let mut content = String::new();

        fs.read_to_string(&mut content)?;

        let cfg: Config = toml::from_str(&content)?;

        *self = cfg;

        Ok(())
    }
}
