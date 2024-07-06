mod common;
pub mod D2CommonEx;
pub mod D2GfxEx;
pub mod D2WinEx;
pub mod D2ClientEx;
pub mod D2SigmaEx;

use crate::d2api::types::*;

pub fn d2ex_init(modules: &D2Modules) -> Result<(), common::HookError> {
    D2CommonEx::init(modules)?;
    D2GfxEx::init(modules)?;
    D2WinEx::init(modules)?;
    D2ClientEx::init(modules)?;

    D2SigmaEx::init(modules)?;
    Ok(())
}
