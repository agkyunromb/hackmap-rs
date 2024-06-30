mod common;
pub mod D2GfxEx;
pub mod D2WinEx;
pub mod D2ClientEx;

use crate::d2api::types::*;

pub fn d2ex_init(modules: &D2Modules) -> Result<(), common::HookError> {
    D2GfxEx::init(modules)?;
    D2WinEx::init(modules)?;
    D2ClientEx::init(modules)?;
    Ok(())
}
