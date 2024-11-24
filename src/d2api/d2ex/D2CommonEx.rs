use crate::d2api::d2ex::common::*;
use crate::d2api::d2consts::*;
use D2Common::{D2Unit, D2Inventory};

pub mod Inventory {
    use std::u32;

    use super::*;

    pub fn get_player_cursor_item() -> Option<&'static mut D2Unit> {
        let player = D2Client::Units::GetClientPlayer()?;
        let inv = ptr_to_ref_mut(player.pInventory)?;
        D2Common::Inventory::GetCursorItem(inv)
    }

    pub fn iter_inventory<F: FnMut(&D2Inventory, &mut D2Unit) -> bool>(unit: &D2Unit, mut cb: F) -> Option<&mut D2Unit> {
        let mut opt_item = D2Common::Inventory::GetFirstItem(ptr_to_ref(unit.pInventory)?);

        while let Some(item) = opt_item {
            if cb(ptr_to_ref(unit.pInventory)?, item) {
                return Some(item);
            }

            opt_item = D2Common::Inventory::GetNextItem(item);
        }

        None
    }
}

pub mod Items {
    pub fn strip_all_color_codes(s: &str) -> String {
        strip_color_code(s, u32::MAX)
    }

    pub fn strip_color_code(s: &str, n: u32) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut ss = String::new();
        let mut i = 0;
        let mut n = n;

        while i < chars.len() {
            let ch = chars[i];
            if n != 0 && ch == 'ÿ' && i + 1 < chars.len() && chars[i + 1] == 'c' {
                i += 3;
                n -= 1;
            } else {
                ss.push(ch);
                i += 1;
            }
        }

        return ss;
    }
}

pub(super) fn init(_modules: &D2Modules) -> Result<(), HookError> {
    Ok(())
}
