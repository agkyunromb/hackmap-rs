use std::path::Path;
use std::io::Read;
use serde::Deserialize;
use super::common::*;
use D2Common::D2Unit;
use anyhow::Result;

use super::config_deserializer::*;

pub(super) type ConfigRef = Rc<RefCell<Config>>;

#[derive(Debug, Deserialize)]
pub(super) struct HotKeyConfig {
    pub reload                  : VirtualKeyCode,
    pub hide_items              : VirtualKeyCode,
    pub perm_show_items  : VirtualKeyCode,
    pub quick_next_game         : VirtualKeyCode,
}

#[derive(Debug, Deserialize)]
pub(super) struct TweaksConfig {
    #[serde(deserialize_with = "bool_from_int")]
    pub perm_show_items: bool,
}

#[derive(Debug, Deserialize)]
pub(super) struct UnitColorConfig {
    #[serde(deserialize_with = "bool_from_int", default)]
    pub show_socket_number          : bool,

    #[serde(deserialize_with = "bool_from_int", default)]
    pub hide_items                  : bool,

    pub player_blob_file            : Option<String>,
    pub monster_blob_file           : Option<String>,
    pub object_blob_file            : Option<String>,
    pub missile_blob_file           : Option<String>,
    pub item_blob_file              : Option<String>,
    pub boss_blob_file              : Option<String>,
    pub npc_blob_file               : Option<String>,
    pub my_blob_file                : Option<String>,
    pub corpse_blob_file            : Option<String>,

    pub normal_monster_color        : u8,
    pub boss_monster_color          : u8,
    pub minion_monster_color        : u8,
    pub champion_monster_color      : u8,
    pub super_unique_color          : u8,

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

    #[serde(deserialize_with = "opt_d2_str_color_code_from_int", default)]
    pub text_color: Option<D2StringColorCodes>,

    pub minimap_color: Option<u8>,

    #[serde(deserialize_with = "opt_d2_item_quality_from_str", default)]
    pub quality: Option<D2ItemQualities>,

    #[serde(deserialize_with = "opt_bool_from_int", default)]
    pub eth: Option<bool>,

    pub socks: Option<usize>,
}

impl UnitColorConfig {
    pub fn get_color_from_unit(&self, item: &D2Unit) -> Option<&ItemColor> {
        let class_id = item.dwClassId;
        let quality = D2Common::Items::GetItemQuality(item);
        let socks_num = D2Common::StatList::GetUnitBaseStat(item, D2ItemStats::Item_NumSockets, 0);
        let is_eth = D2Common::Items::CheckItemFlag(item, D2ItemFlags::Ethereal) != FALSE;

        for entry in self.item_colors.iter().rev() {
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
                reload                  : Default::default(),
                hide_items              : Default::default(),
                perm_show_items  : Default::default(),
                quick_next_game         : Default::default(),
            },

            tweaks: TweaksConfig{
                perm_show_items: true,
            },

            unit_color: UnitColorConfig{
                show_socket_number              : true,
                hide_items                      : true,

                player_blob_file                : None,
                monster_blob_file               : None,
                object_blob_file                : None,
                missile_blob_file               : None,
                item_blob_file                  : None,
                boss_blob_file                  : None,
                npc_blob_file                   : None,
                my_blob_file                    : None,
                corpse_blob_file                : None,

                magic_resistant_desc            : None,
                fire_enchanted_desc             : None,
                lightning_enchanted_desc        : None,
                cold_enchanted_desc             : None,
                mana_burn_desc                  : None,

                normal_monster_color            : 0xFF,
                boss_monster_color              : 0xFF,
                minion_monster_color            : 0xFF,
                champion_monster_color          : 0xFF,
                super_unique_color              : 0xFF,

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
