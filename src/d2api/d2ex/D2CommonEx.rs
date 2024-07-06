use crate::d2api::d2ex::common::*;
use crate::d2api::d2consts::*;
use D2Common::D2Unit;

pub mod Inventory {
    use super::*;

    pub fn get_player_cursor_item() -> Option<&'static mut D2Unit> {
        let player = D2Client::Units::GetClientPlayer()?;
        let inv = ptr_to_ref_mut(player.pInventory)?;
        D2Common::Inventory::GetCursorItem(inv)
    }
}

pub(super) fn init(_modules: &D2Modules) -> Result<(), HookError> {
    Ok(())
}
