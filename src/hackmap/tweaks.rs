use super::common::*;
use windows_sys::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS32;

::windows_targets::link!("ntdll.dll" "system" fn RtlImageNtHeader(Base: PVOID) -> *const IMAGE_NT_HEADERS32);

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

extern "stdcall" fn HandleUIVars(this: PVOID) {
    D2Client::UI::SetUIVar(13, 0, 0);
    get_stubs().UI_HandleUIVars.unwrap()(this);
    D2Client::UI::SetUIVar(13, 1, 0);
}

extern "stdcall" fn MISC_CalculateShadowRGBA(r: &mut u8, g: &mut u8, b: &mut u8, a: &mut u8) {
    *a = 0xFF;
    *r = 0xFF;
    *g = 0xFF;
    *b = 0xFF;
}

extern "stdcall" fn D2Common_Units_TestCollisionWithUnit(_unit1: PVOID, _unit2: PVOID, _collision_mask: i32) -> BOOL {
    FALSE
}

pub fn init(modules: &D2Modules) -> Result<(), HookError> {
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
