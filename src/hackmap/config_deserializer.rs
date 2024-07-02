use serde::Deserialize;
use serde::de::{self, Deserializer, Unexpected};
use super::common::*;

pub fn default_option<T>() -> Option<T> {
    None
}

impl<'de> Deserialize<'de> for D2ItemQualities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        match s.as_str() {
            "inferior"  => Ok(D2ItemQualities::Inferior),
            "normal"    => Ok(D2ItemQualities::Normal),
            "superior"  => Ok(D2ItemQualities::Superior),
            "magic"     => Ok(D2ItemQualities::Magic),
            "set"       => Ok(D2ItemQualities::Set),
            "rare"      => Ok(D2ItemQualities::Rare),
            "unique"    => Ok(D2ItemQualities::Unique),
            "craft"     => Ok(D2ItemQualities::Craft),
            "tempered"  => Ok(D2ItemQualities::Tempered),
            _ => Err(serde::de::Error::custom("Unknown D2ItemQualities value")),
        }
    }
}

fn deserialize_option_qualities<'de, D>(deserializer: D) -> Result<Option<D2ItemQualities>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(D2ItemQualities::deserialize(deserializer)?))
}

pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"0 or 1",
        )),
    }
}

pub fn opt_bool_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(bool_from_int(deserializer)?))
}

pub fn d2_str_color_code_from_int<'de, D>(deserializer: D) -> Result<D2StringColorCodes, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(D2StringColorCodes::White),
        1 => Ok(D2StringColorCodes::Red),
        2 => Ok(D2StringColorCodes::LightGreen),
        3 => Ok(D2StringColorCodes::Blue),
        4 => Ok(D2StringColorCodes::DarkGold),
        5 => Ok(D2StringColorCodes::Grey),
        6 => Ok(D2StringColorCodes::Black),
        7 => Ok(D2StringColorCodes::Tan),
        8 => Ok(D2StringColorCodes::Orange),
        9 => Ok(D2StringColorCodes::Yellow),
        10 => Ok(D2StringColorCodes::DarkGreen),
        11 => Ok(D2StringColorCodes::Purple),
        12 => Ok(D2StringColorCodes::DarkGreen2),
        _ => Ok(D2StringColorCodes::Invalid),
    }
}

pub fn opt_d2_item_quality_from_str<'de, D>(deserializer: D) -> Result<Option<D2ItemQualities>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(D2ItemQualities::deserialize(deserializer)?))
}

pub fn deserialize_monster_color<'de, D>(deserializer: D) -> Result<HashMap<u32, u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let intermediate: HashMap<String, u8> = HashMap::deserialize(deserializer)?;
    let mut result = HashMap::new();

    for (key, color) in intermediate {
        let id = u32::from_str_radix(&key.trim_start_matches("0x"), if key.starts_with("0x") { 16 } else { 10 }).map_err(serde::de::Error::custom)?;
        result.insert(id, color);
    }

    Ok(result)
}
