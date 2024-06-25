use super::common::*;
use super::{
    config,
    automap,
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

pub(super) struct HackMap {
    pub config                  : config::ConfigRef,
    pub input                   : input::Input,
    pub automap                 : automap::AutoMap,
    pub quick_next_game         : quick_next::QuickNextGame,
    pub tweaks                  : tweaks::Tweaks,
}

impl HackMap {
    fn new() -> Self {
        let config = Rc::new(RefCell::new(config::Config::new()));

        Self{
            config              : Rc::clone(&config),
            input               : input::Input::new(Rc::clone(&config)),
            automap             : automap::AutoMap::new(),
            quick_next_game     : quick_next::QuickNextGame::new(),
            tweaks              : tweaks::Tweaks::new(Rc::clone(&config)),
        }
    }

    pub fn get() -> &'static mut Self {
        static mut HACKMAP: Option<HackMap> = None;

        unsafe {
            if HACKMAP.is_none() {
                HACKMAP = Some(HackMap::new());
            }

            HACKMAP.as_mut().unwrap()
            // &mut *std::ptr::addr_of_mut!(HACKMAP.unwrap())
        }
    }

    pub fn config() -> config::ConfigRef {
        Rc::clone(&Self::get().config)
        // &mut Self::get().config
    }

    pub fn input() -> &'static mut input::Input {
        &mut Self::get().input
    }

    pub fn automap() -> &'static mut automap::AutoMap {
        &mut Self::get().automap
    }

    pub fn quick_next() -> &'static mut quick_next::QuickNextGame {
        &mut Self::get().quick_next_game
    }

    pub fn tweaks() -> &'static mut tweaks::Tweaks {
        &mut Self::get().tweaks
    }
}

pub fn init(modules: &D2Modules) {
    let initializer: &[(&str, fn(&D2Modules) -> Result<(), HookError>)] = &[
        ("auto_map",    automap::init),
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
