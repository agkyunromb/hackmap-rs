use std::cell::RefCell;
use std::rc::Rc;
use std::sync::OnceLock;

use D2Common::D2Unit;

use crate::d2api::d2ex::common::*;
use crate::d2api::d2consts::*;

#[repr(C, packed(1))]
struct FormatItemPropertiesContext {
    pub buf1                    : [u8; 0x100],          // 0x0000
    pub text                    : [u16; 0x4000],        // 0x0100
    pub text2                   : [u16; 0x4000],        // 0x8100
    pub text3                   : [[u16; 0x400]; 3],    // 0xC100
    pub client_unit_type_table  : PVOID,                // 0xD900
    pub unit                    : *mut D2Unit,          // 0xD904
    pub owner                   : *mut D2Unit,          // 0xD908
}

struct D2SigmaEx {
    is_getting_item_properties  : bool,
    item_properties         : String,
    DrawFramedText          : Option<extern "fastcall" fn(PCWSTR, i32, i32, i32, i32)>,
}

impl D2SigmaEx {
    const fn new() -> Self {
        Self {
            is_getting_item_properties  : false,
            item_properties         : String::new(),
            DrawFramedText          : None,
        }
    }

    #[allow(static_mut_refs)]
    pub fn get() -> &'static mut Self {
        static mut OBJ: D2SigmaEx = D2SigmaEx::new();

        unsafe {
            &mut OBJ
        }
    }

    fn get_item_properties(&mut self, unit: &D2Common::D2Unit) -> String {
        self.is_getting_item_properties = true;

        D2Sigma::Units::DisplayItemProperties(D2Client::Units::GetClientUnitTypeTable(), unit);

        self.is_getting_item_properties = false;

        std::mem::take(&mut self.item_properties)
    }

    fn on_get_item_properties(&mut self, text: &str) {
        let chars: Vec<char> = text.chars().collect();
        let mut new_text = String::new();
        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];
            if ch == 'ÿ' && i + 1 < chars.len() && chars[i + 1] == 'c' {
                i += 3;
            } else {
                new_text.push(ch);
                i += 1;
            }
        }

        self.item_properties = new_text;
    }

    extern "fastcall" fn draw_framed_text(text: PCWSTR, x: i32, y: i32, color: i32, align: i32) {
        let sigma = D2SigmaEx::get();

        if sigma.is_getting_item_properties {
            sigma.on_get_item_properties(&text.to_string());
            return;
        }

        sigma.DrawFramedText.unwrap()(text, x, y, color, align)
    }
}

pub mod Items {
    use super::*;

    pub fn get_item_name(unit: &D2Unit) -> String {
        let buffer: [u16; 0x1000] = [0; 0x1000];

        D2Sigma::Items::GetItemName(unit, buffer.as_ptr(), 0);

        String::from_utf16_lossy(buffer.as_slice())
    }

    pub fn get_item_properties(unit: &D2Unit) -> String {
        D2SigmaEx::get().get_item_properties(unit)
    }

    pub fn is_getting_item_properties() -> bool {
        D2SigmaEx::get().is_getting_item_properties
    }
}

pub(super) fn init(_modules: &D2Modules) -> Result<(), HookError> {
    let sigma = D2SigmaEx::get();

    inline_hook_jmp(0, D2Win::AddressTable.Text.DrawFramedText, D2SigmaEx::draw_framed_text as usize, Some(&mut sigma.DrawFramedText), None)?;

    Ok(())
}
