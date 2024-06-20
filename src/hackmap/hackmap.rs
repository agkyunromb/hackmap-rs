use super::common::*;
use super::{
    unit_color,
    tweaks,
    input,
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

impl Default for HackMapConfig {
    fn default() -> Self {
        Self::new()
    }
}

type OnKeyDownCallback = fn(vk: u16) -> bool;

pub(super) struct HackMap {
    pub options                 : HackMapConfig,
    pub on_keydown_callbacks    : Vec<OnKeyDownCallback>,
}

impl HackMap {
    const fn new() -> Self {
        Self{
            options                 : HackMapConfig::new(),
            on_keydown_callbacks    : vec![],
        }
    }

    pub fn get() -> &'static mut HackMap {
        unsafe {
            &mut *std::ptr::addr_of_mut!(HACKMAP)
        }
    }

    pub fn on_key_down(&mut self, f: OnKeyDownCallback) {
        self.on_keydown_callbacks.push(f);
    }
}

static mut HACKMAP: HackMap = HackMap::new();

pub fn init(modules: &D2Modules) {
    // let mut hm = HACKMAP.lock();

    // hm.options.perm_show_items_toggle = false;

    unit_color::init(&modules).unwrap();
    tweaks::init(&modules).unwrap();
    input::init(&modules).unwrap();
}
