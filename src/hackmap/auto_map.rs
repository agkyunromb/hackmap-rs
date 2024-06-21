use super::common::*;
use super::HackMap;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetKeyState;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_CONTROL;
use D2Win::MsgHandler::{StormMsgHandler, StormMsgHandlerParams};

struct Stubs {
    Handle_D2GS_LOADCOMPLETE_04 : Option<D2Client::Net::D2GSHandler>,
}

static mut STUBS: Stubs = Stubs{
    Handle_D2GS_LOADCOMPLETE_04 : None,
};

#[allow(static_mut_refs)]
fn get_stubs() -> &'static Stubs {
    unsafe { &STUBS }
}

extern "fastcall" fn Handle_D2GS_LOADCOMPLETE_04(payload: *const u8) {
    get_stubs().Handle_D2GS_LOADCOMPLETE_04.unwrap()(payload);
    D2Sigma::AutoMap::RevealMap();
}

pub fn init(_modules: &D2Modules) -> Result<(), HookError> {
    // let D2Sigma = modules.D2Sigma.unwrap();

    unsafe {
        STUBS.Handle_D2GS_LOADCOMPLETE_04 = Some(D2Client::Net::SwapD2GSHandler(0x04, Handle_D2GS_LOADCOMPLETE_04));
    }

    Ok(())
}
