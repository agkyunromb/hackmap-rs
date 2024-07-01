use std::path::Path;
use std::io::Read;
use serde::Deserialize;
use serde::de::{self, Deserializer, Unexpected};
use super::common::*;
use anyhow::Result;

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

fn deserialize_monster_color<'de, D>(deserializer: D) -> Result<HashMap<u32, u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let intermediate: HashMap<String, u8> = HashMap::deserialize(deserializer)?;
    let mut result = HashMap::new();

    for (key, color) in intermediate {
        // let id = key.parse::<u32>().map_err(serde::de::Error::custom)?;
        let id = u32::from_str_radix(&key.trim_start_matches("0x"), if key.starts_with("0x") { 16 } else { 10 }).map_err(serde::de::Error::custom)?;
        result.insert(id, color);
    }

    Ok(result)
}

pub(super) type ConfigRef = Rc<RefCell<Config>>;

#[derive(Debug, Deserialize)]
pub(super) struct TweaksConfig {
    #[serde(deserialize_with = "bool_from_int")]
    pub perm_show_items_toggle: bool,
}

#[derive(Debug, Deserialize)]
pub(super) struct UnitColorConfig {
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
}

#[derive(Debug, Deserialize)]
pub(super) struct Config {
    pub tweaks      : TweaksConfig,
    pub unit_color  : UnitColorConfig,
}

impl Config {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self{
            tweaks: TweaksConfig{
                perm_show_items_toggle: true,
            },
            unit_color: UnitColorConfig{
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
