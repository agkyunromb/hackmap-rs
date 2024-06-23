use super::common::*;
use super::{
    auto_map,
    unit_color,
    tweaks,
    input,
    quick_next,
    helper_bot,
};

pub(super) struct HackMapConfig {
    pub(super) perm_show_items_toggle: bool,
}

impl HackMapConfig {
    const fn new() -> Self {
        Self {
            perm_show_items_toggle: true,
        }
    }
}

type OnKeyDownCallback = fn(vk: u16) -> bool;

pub(super) struct QuickNextGameInfo {
    pub auto_create_game                    : bool,
    pub auto_game_name                      : String,
    pub auto_game_password                  : String,
    pub auto_game_index                     : Option<i32>,
    pub create_game_button                  : Option<PVOID>,
    pub on_create_game_tab_button_clicked   : Option<D2Win::Control::PerformFnType>,
    pub on_create_game_button_clicked       : Option<D2Win::Control::PerformFnType>,
}

impl QuickNextGameInfo {
    const fn new() -> Self {
        Self {
            auto_create_game                    : false,
            auto_game_name                      : String::new(),
            auto_game_password                  : String::new(),
            auto_game_index                     : None,
            create_game_button                  : None,
            on_create_game_tab_button_clicked   : None,
            on_create_game_button_clicked       : None,
        }
    }
}

pub(super) struct HackMap {
    pub options                 : HackMapConfig,
    pub quick_next_game         : QuickNextGameInfo,
    pub on_keydown_callbacks    : Vec<OnKeyDownCallback>,
    pub current_monster_name    : Vec<u16>,
    pub automap_cells_for_layers: Option<std::collections::HashMap<u32, Vec<auto_map::D2AutoMapCellDataEx>>>,
}

impl HackMap {
    const fn new() -> Self {
        Self{
            options                 : HackMapConfig::new(),
            quick_next_game         : QuickNextGameInfo::new(),
            on_keydown_callbacks    : vec![],
            current_monster_name    : vec![],
            automap_cells_for_layers: None,
        }
    }

    pub fn on_key_down(&mut self, f: OnKeyDownCallback) {
        self.on_keydown_callbacks.push(f);
    }

    pub fn get() -> &'static mut HackMap {
        unsafe {
            &mut *std::ptr::addr_of_mut!(HACKMAP)
        }
    }
}

static mut HACKMAP: HackMap = HackMap::new();

pub fn init(modules: &D2Modules) {
    let initializer: &[(&str, fn(&D2Modules) -> Result<(), HookError>)] = &[
        ("auto_map",    auto_map::init),
        ("input",       input::init),
        ("unit_color",  unit_color::init),
        ("tweaks",      tweaks::init),
        ("quick_next",  quick_next::init),
        ("helper_bot",  helper_bot::init),
    ];

    for m in initializer {
        Fog::Trace(format!("init {}", m.0).as_str());
        m.1(&modules).expect(m.0);
    }
}
