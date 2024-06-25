use super::common::*;

pub(super) type ConfigRef = Rc<RefCell<Config>>;

pub(super) struct Config {
    pub perm_show_items_toggle: bool,
}

impl Config {
    pub const fn new() -> Self {
        Self{
            perm_show_items_toggle: true,
        }
    }
}
