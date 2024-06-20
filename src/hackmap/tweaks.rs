use super::common::*;
use super::HackMap;

struct Stubs {
    UI_HandleUIVars: Option<extern "stdcall" fn(PVOID)>,
}

static mut STUBS: Stubs = Stubs{
    UI_HandleUIVars: None,
};

#[allow(static_mut_refs)]
fn get_stubs() -> &'static Stubs {
    unsafe { &STUBS }
}

extern "stdcall" fn HandleUIVars(obj: PVOID) {
    HackMap::get().handle_perm_show_items(obj);
}

extern "stdcall" fn MISC_CalculateShadowRGBA(r: &mut u8, g: &mut u8, b: &mut u8, a: &mut u8) {
    *a = 0xFF;
    *r = 0xFF;
    *g = 0xFF;
    *b = 0xFF;
}

extern "stdcall" fn D2Common_Units_TestCollisionWithUnit(unit1: PVOID, unit2: PVOID, collision_mask: i32) -> BOOL {
    let (success, hide) = HackMap::get().should_hide_unit(unit2);

    if success == false {
        return D2Common::Units::TestCollisionWithUnit(unit1, unit2, collision_mask);
    }

    if hide { FALSE } else { TRUE }
}

impl HackMap {
    fn should_hide_unit(&self, _unit: PVOID) -> (bool, bool) {
        let success = true;
        let hide = false;

        (success, hide)
    }

    fn handle_perm_show_items(&self, obj: PVOID) {
        let UI_HandleUIVars = get_stubs().UI_HandleUIVars.unwrap();

        if self.options.perm_show_items_toggle == false || D2Client::UI::GetUIVar(D2UIvars::HoldAlt) != 0 {
            UI_HandleUIVars(obj);
            return;
        }

        D2Client::UI::SetUIVar(D2UIvars::HoldAlt, 0, 0);
        UI_HandleUIVars(obj);
        D2Client::UI::SetUIVar(D2UIvars::HoldAlt, 1, 0);
    }
}

pub fn init(modules: &D2Modules) -> Result<(), HookError> {
    HackMap::get().on_key_down(|vk| -> bool {
        if vk == 'Y' as u16 {
            let hm = HackMap::get();
            hm.options.perm_show_items_toggle = !hm.options.perm_show_items_toggle;
        }

        false
    });

    unsafe {
        // 永久显示地面物品
        let glide3x = &*RtlImageNtHeader(modules.glide3x.unwrap() as PVOID);

        inline_hook_call(0, D2Client::AddressTable.UI.HandleUIVars, HandleUIVars as usize, Some(&mut STUBS.UI_HandleUIVars), None)?;
        patch_memory_value(modules.D2Client.unwrap(), D2RVA::D2Client(0x6FB0948B), 0xEB, 1)?;

        // HDText_drawFramedText_is_alt_clicked
        match glide3x.FileHeader.TimeDateStamp {
            0x6606E04D => {
                patch_memory_value(modules.glide3x.unwrap(), 0x55F2E, 0x80, 1)?;
            },

            _ => {},
        }

        // 去除阴影
        inline_hook_jmp::<()>(modules.D2Client.unwrap(), D2RVA::D2Client(0x6FB59A20), MISC_CalculateShadowRGBA as usize, None, None)?;

        // 透视
        inline_hook_call::<()>(modules.D2Client.unwrap(), D2RVA::D2Client(0x6FB16695), D2Common_Units_TestCollisionWithUnit as usize, None, None)?;
    }

    Ok(())
}
