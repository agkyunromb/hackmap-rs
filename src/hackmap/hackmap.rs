use super::common::*;
use super::{
    unit_color,
    tweaks,
};

pub fn init(modules: &D2Modules) {
    unit_color::init(&modules).unwrap();
    tweaks::init(&modules).unwrap();
}
