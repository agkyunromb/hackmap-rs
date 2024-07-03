use serde::Deserialize;
use serde::de::{self, Deserializer, Unexpected};
use super::common::*;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref VK_MAP: HashMap<&'static str, u16> = [
        ("VK_NULL",                     0x00),
        (" K_LBUTTON",                  0x01),        // Left mouse button
        ("VK_RBUTTON",                  0x02),        // Right mouse button
        ("VK_CANCEL",                   0x03),        // Control-break processing
        ("VK_MBUTTON",                  0x04),        // Middle mouse button
        ("VK_XBUTTON1",                 0x05),        // X1 mouse button
        ("VK_XBUTTON2",                 0x06),        // X2 mouse button
        ("VK_BACK",                     0x08),        // BACKSPACE key
        ("VK_TAB",                      0x09),        // TAB key
        ("VK_CLEAR",                    0x0C),        // CLEAR key
        ("VK_RETURN",                   0x0D),        // ENTER key
        ("VK_SHIFT",                    0x10),        // SHIFT key
        ("VK_CONTROL",                  0x11),        // CTRL key
        ("VK_MENU",                     0x12),        // ALT key
        ("VK_PAUSE",                    0x13),        // PAUSE key
        ("VK_CAPITAL",                  0x14),        // CAPS LOCK key
        ("VK_KANA",                     0x15),        // IME Kana mode
        ("VK_HANGUL",                   0x15),        // IME Hangul mode
        ("VK_IME_ON",                   0x16),        // IME On
        ("VK_JUNJA",                    0x17),        // IME Junja mode
        ("VK_FINAL",                    0x18),        // IME final mode
        ("VK_HANJA",                    0x19),        // IME Hanja mode
        ("VK_KANJI",                    0x19),        // IME Kanji mode
        ("VK_IME_OFF",                  0x1A),        // IME Off
        ("VK_ESCAPE",                   0x1B),        // ESC key
        ("VK_CONVERT",                  0x1C),        // IME convert
        ("VK_NONCONVERT",               0x1D),        // IME nonconvert
        ("VK_ACCEPT",                   0x1E),        // IME accept
        ("VK_MODECHANGE",               0x1F),        // IME mode change request
        ("VK_SPACE",                    0x20),        // SPACEBAR
        ("VK_PRIOR",                    0x21),        // PAGE UP key
        ("VK_NEXT",                     0x22),        // PAGE DOWN key
        ("VK_END",                      0x23),        // END key
        ("VK_HOME",                     0x24),        // HOME key
        ("VK_LEFT",                     0x25),        // LEFT ARROW key
        ("VK_UP",                       0x26),        // UP ARROW key
        ("VK_RIGHT",                    0x27),        // RIGHT ARROW key
        ("VK_DOWN",                     0x28),        // DOWN ARROW key
        ("VK_SELECT",                   0x29),        // SELECT key
        ("VK_PRINT",                    0x2A),        // PRINT key
        ("VK_EXECUTE",                  0x2B),        // EXECUTE key
        ("VK_SNAPSHOT",                 0x2C),        // PRINT SCREEN key
        ("VK_INSERT",                   0x2D),        // INS key
        ("VK_DELETE",                   0x2E),        // DEL key
        ("VK_HELP",                     0x2F),        // HELP key
        ("VK_0",                        0x30),
        ("VK_1",                        0x31),
        ("VK_2",                        0x32),
        ("VK_3",                        0x33),
        ("VK_4",                        0x34),
        ("VK_5",                        0x35),
        ("VK_6",                        0x36),
        ("VK_7",                        0x37),
        ("VK_8",                        0x38),
        ("VK_9",                        0x39),
        ("VK_A",                        0x41),
        ("VK_B",                        0x42),
        ("VK_C",                        0x43),
        ("VK_D",                        0x44),
        ("VK_E",                        0x45),
        ("VK_F",                        0x46),
        ("VK_G",                        0x47),
        ("VK_H",                        0x48),
        ("VK_I",                        0x49),
        ("VK_J",                        0x4A),
        ("VK_K",                        0x4B),
        ("VK_L",                        0x4C),
        ("VK_M",                        0x4D),
        ("VK_N",                        0x4E),
        ("VK_O",                        0x4F),
        ("VK_P",                        0x50),
        ("VK_Q",                        0x51),
        ("VK_R",                        0x52),
        ("VK_S",                        0x53),
        ("VK_T",                        0x54),
        ("VK_U",                        0x55),
        ("VK_V",                        0x56),
        ("VK_W",                        0x57),
        ("VK_X",                        0x58),
        ("VK_Y",                        0x59),
        ("VK_Z",                        0x5A),
        ("VK_LWIN",                     0x5B),        // Left Windows key
        ("VK_RWIN",                     0x5C),        // Right Windows key
        ("VK_APPS",                     0x5D),        // Applications key
        ("VK_SLEEP",                    0x5F),        // Computer Sleep key
        ("VK_NUMPAD0",                  0x60),        // Numeric keypad 0 key
        ("VK_NUMPAD1",                  0x61),        // Numeric keypad 1 key
        ("VK_NUMPAD2",                  0x62),        // Numeric keypad 2 key
        ("VK_NUMPAD3",                  0x63),        // Numeric keypad 3 key
        ("VK_NUMPAD4",                  0x64),        // Numeric keypad 4 key
        ("VK_NUMPAD5",                  0x65),        // Numeric keypad 5 key
        ("VK_NUMPAD6",                  0x66),        // Numeric keypad 6 key
        ("VK_NUMPAD7",                  0x67),        // Numeric keypad 7 key
        ("VK_NUMPAD8",                  0x68),        // Numeric keypad 8 key
        ("VK_NUMPAD9",                  0x69),        // Numeric keypad 9 key
        ("VK_MULTIPLY",                 0x6A),        // Multiply key
        ("VK_ADD",                      0x6B),        // Add key
        ("VK_SEPARATOR",                0x6C),        // Separator key
        ("VK_SUBTRACT",                 0x6D),        // Subtract key
        ("VK_DECIMAL",                  0x6E),        // Decimal key
        ("VK_DIVIDE",                   0x6F),        // Divide key
        ("VK_F1",                       0x70),        // F1 key
        ("VK_F2",                       0x71),        // F2 key
        ("VK_F3",                       0x72),        // F3 key
        ("VK_F4",                       0x73),        // F4 key
        ("VK_F5",                       0x74),        // F5 key
        ("VK_F6",                       0x75),        // F6 key
        ("VK_F7",                       0x76),        // F7 key
        ("VK_F8",                       0x77),        // F8 key
        ("VK_F9",                       0x78),        // F9 key
        ("VK_F10",                      0x79),        // F10 key
        ("VK_F11",                      0x7A),        // F11 key
        ("VK_F12",                      0x7B),        // F12 key
        ("VK_F13",                      0x7C),        // F13 key
        ("VK_F14",                      0x7D),        // F14 key
        ("VK_F15",                      0x7E),        // F15 key
        ("VK_F16",                      0x7F),        // F16 key
        ("VK_F17",                      0x80),        // F17 key
        ("VK_F18",                      0x81),        // F18 key
        ("VK_F19",                      0x82),        // F19 key
        ("VK_F20",                      0x83),        // F20 key
        ("VK_F21",                      0x84),        // F21 key
        ("VK_F22",                      0x85),        // F22 key
        ("VK_F23",                      0x86),        // F23 key
        ("VK_F24",                      0x87),        // F24 key
        ("VK_NUMLOCK",                  0x90),        // NUM LOCK key
        ("VK_SCROLL",                   0x91),        // SCROLL LOCK key
        ("VK_LSHIFT",                   0xA0),        // Left SHIFT key
        ("VK_RSHIFT",                   0xA1),        // Right SHIFT key
        ("VK_LCONTROL",                 0xA2),        // Left CONTROL key
        ("VK_RCONTROL",                 0xA3),        // Right CONTROL key
        ("VK_LMENU",                    0xA4),        // Left ALT key
        ("VK_RMENU",                    0xA5),        // Right ALT key
        ("VK_BROWSER_BACK",             0xA6),        // Browser Back key
        ("VK_BROWSER_FORWARD",          0xA7),        // Browser Forward key
        ("VK_BROWSER_REFRESH",          0xA8),        // Browser Refresh key
        ("VK_BROWSER_STOP",             0xA9),        // Browser Stop key
        ("VK_BROWSER_SEARCH",           0xAA),        // Browser Search key
        ("VK_BROWSER_FAVORITES",        0xAB),        // Browser Favorites key
        ("VK_BROWSER_HOME",             0xAC),        // Browser Start and Home key
        ("VK_VOLUME_MUTE",              0xAD),        // Volume Mute key
        ("VK_VOLUME_DOWN",              0xAE),        // Volume Down key
        ("VK_VOLUME_UP",                0xAF),        // Volume Up key
        ("VK_MEDIA_NEXT_TRACK",         0xB0),        // Next Track key
        ("VK_MEDIA_PREV_TRACK",         0xB1),        // Previous Track key
        ("VK_MEDIA_STOP",               0xB2),        // Stop Media key
        ("VK_MEDIA_PLAY_PAUSE",         0xB3),        // Play/Pause Media key
        ("VK_LAUNCH_MAIL",              0xB4),        // Start Mail key
        ("VK_LAUNCH_MEDIA_SELECT",      0xB5),        // Select Media key
        ("VK_LAUNCH_APP1",              0xB6),        // Start Application 1 key
        ("VK_LAUNCH_APP2",              0xB7),        // Start Application 2 key
        ("VK_OEM_1",                    0xBA),        // Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the ;: key
        ("VK_OEM_PLUS",                 0xBB),        // For any country/region, the + key
        ("VK_OEM_COMMA",                0xBC),        // For any country/region, the , key
        ("VK_OEM_MINUS",                0xBD),        // For any country/region, the - key
        ("VK_OEM_PERIOD",               0xBE),        // For any country/region, the . key
        ("VK_OEM_2",                    0xBF),        // Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the /? key
        ("VK_OEM_3",                    0xC0),        // Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the `~ key
        ("VK_OEM_4",                    0xDB),        // Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the [{ key
        ("VK_OEM_5",                    0xDC),        // Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the \\| key
        ("VK_OEM_6",                    0xDD),        // Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the ]} key
        ("VK_OEM_7",                    0xDE),        // Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '" key
        ("VK_OEM_8",                    0xDF),        // Used for miscellaneous characters; it can vary by keyboard.
        ("VK_OEM_102",                  0xE2),        // The <> keys on the US standard keyboard, or the \\| key on the non-US 102-key keyboard
        ("VK_PROCESSKEY",               0xE5),        // IME PROCESS key
        ("VK_PACKET",                   0xE7),        // Used to pass Unicode characters as if they were keystrokes. The VK_PACKET key is the low word of a 32-bit Virtual Key value used for non-keyboard input methods. For more information, see Remark in KEYBDINPUT, SendInput, WM_KEYDOWN, and WM_KEYUP
        ("VK_ATTN",                     0xF6),        // Attn key
        ("VK_CRSEL",                    0xF7),        // CrSel key
        ("VK_EXSEL",                    0xF8),        // ExSel key
        ("VK_EREOF",                    0xF9),        // Erase EOF key
        ("VK_PLAY",                     0xFA),        // Play key
        ("VK_ZOOM",                     0xFB),        // Zoom key
        ("VK_NONAME",                   0xFC),        // Reserved
        ("VK_PA1",                      0xFD),        // PA1 key
        ("VK_OEM_CLEAR",                0xFE),        // Clear key
    ].iter().cloned().collect();
}

#[derive(Debug)]
pub(super) struct RegexWrapper(Regex);

impl std::ops::Deref for RegexWrapper {
    type Target = Regex;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for RegexWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Regex::new(&s)
            .map(RegexWrapper)
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Copy, Clone)]
pub(super) struct VirtualKeyCode(u16);

impl VirtualKeyCode {
    fn from_str(vk: &str) -> Self {
        VK_MAP.get(vk).map_or(Self::default(), |&v| {
            v.into()
        })
    }
}

impl From<u16> for VirtualKeyCode {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl Into<u16> for VirtualKeyCode {
    fn into(self) -> u16 {
        self.0
    }
}

impl PartialEq<u16> for VirtualKeyCode {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl PartialEq<VirtualKeyCode> for u16 {
    fn eq(&self, other: &VirtualKeyCode) -> bool {
        *self == other.0
    }
}

impl Default for VirtualKeyCode {
    fn default() -> Self {
        Self(0)
    }
}

impl<'de> Deserialize<'de> for VirtualKeyCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_uppercase();
        Ok(Self::from_str(&s))
    }
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

impl<'de> Deserialize<'de> for D2StringColorCodes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u8::deserialize(deserializer)? {
            0  => Ok(D2StringColorCodes::White),
            1  => Ok(D2StringColorCodes::Red),
            2  => Ok(D2StringColorCodes::LightGreen),
            3  => Ok(D2StringColorCodes::Blue),
            4  => Ok(D2StringColorCodes::DarkGold),
            5  => Ok(D2StringColorCodes::Grey),
            6  => Ok(D2StringColorCodes::Black),
            7  => Ok(D2StringColorCodes::Tan),
            8  => Ok(D2StringColorCodes::Orange),
            9  => Ok(D2StringColorCodes::Yellow),
            10 => Ok(D2StringColorCodes::DarkGreen),
            11 => Ok(D2StringColorCodes::Purple),
            12 => Ok(D2StringColorCodes::DarkGreen2),
            _  => Ok(D2StringColorCodes::Invalid),
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

pub fn opt_d2_str_color_code_from_int<'de, D>(deserializer: D) -> Result<Option<D2StringColorCodes>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(D2StringColorCodes::deserialize(deserializer)?))
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
