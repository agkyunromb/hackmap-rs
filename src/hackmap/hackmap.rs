use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OKCANCEL};

use super::common::*;
use super::{
    config,
    image_loader,
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
    pub image_loader            : image_loader::ImageLoader,
    pub input                   : input::Input,
    pub automap                 : automap::AutoMap,
    pub quick_next_game         : quick_next::QuickNextGame,
    pub tweaks                  : tweaks::Tweaks,
    pub unit_color              : unit_color::UnitColor,
}

impl HackMap {
    fn new() -> Self {
        let config = Rc::new(RefCell::new(config::Config::new()));

        Self{
            config              : Rc::clone(&config),
            image_loader        : image_loader::ImageLoader::new(Rc::clone(&config)),
            input               : input::Input::new(Rc::clone(&config)),
            automap             : automap::AutoMap::new(),
            quick_next_game     : quick_next::QuickNextGame::new(),
            tweaks              : tweaks::Tweaks::new(Rc::clone(&config)),
            unit_color          : unit_color::UnitColor::new(Rc::clone(&config)),
        }
    }

    fn init(&mut self) -> anyhow::Result<()> {
        self.config.borrow_mut().load("hackmap\\hackmap.cfg.toml")?;

        Ok(())
    }

    pub fn get() -> &'static mut Self {
        static mut HACKMAP: Option<HackMap> = None;

        unsafe {
            if HACKMAP.is_none() {
                HACKMAP = Some(HackMap::new());
            }

            HACKMAP.as_mut().unwrap()
        }
    }

    pub fn config() -> config::ConfigRef {
        Rc::clone(&Self::get().config)
    }

    pub fn image_loader() -> &'static mut image_loader::ImageLoader {
        &mut Self::get().image_loader
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

    pub fn unit_color() -> &'static mut unit_color::UnitColor {
        &mut Self::get().unit_color
    }
}

pub fn init(modules: &D2Modules) {
    if let Err(err) = HackMap::get().init() {
        println!("{}", err);
        unsafe {
            MessageBoxW(0, format!("{err}").to_utf16().as_ptr(), null(), MB_OK);
        }
    }

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
