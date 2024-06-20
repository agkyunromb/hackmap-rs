use super::common::*;
use super::HackMap;
use D2Win::MsgHandler::{StormMsgHandler, StormMsgHandlerParams};

struct Stubs {
    RegisterMsgHandler: Option<extern "fastcall" fn(hwnd: HWND, msg_type: u32, msg:u32, handler: StormMsgHandler)>,
    InGame_OnKeyDown: Option<extern "stdcall" fn(&mut StormMsgHandlerParams)>,
}

static mut STUBS: Stubs = Stubs{
    RegisterMsgHandler    : None,
    InGame_OnKeyDown      : None,
};

#[allow(static_mut_refs)]
fn get_stubs() -> &'static Stubs {
    unsafe { &STUBS }
}

extern "stdcall" fn InGame_OnKeyDown(msg: &mut StormMsgHandlerParams) {
    HackMap::get().in_game_on_key_down(msg);
    get_stubs().InGame_OnKeyDown.unwrap()(msg)
}

extern "fastcall" fn RegisterMsgHandler(hwnd: HWND, msg_type: u32, msg: u32, handler: StormMsgHandler) {
    if msg_type == 0 && msg == WM_KEYDOWN {
        if get_stubs().InGame_OnKeyDown.is_none() {
            unsafe {
                inline_hook_jmp(0, handler as usize, InGame_OnKeyDown as usize, Some(&mut STUBS.InGame_OnKeyDown), None).unwrap();
            }
        }
    }

    get_stubs().RegisterMsgHandler.unwrap()(hwnd, msg_type, msg, handler);
}

impl HackMap {
    fn in_game_on_key_down(&mut self, msg: &mut StormMsgHandlerParams) {
        if msg.returned != FALSE || msg.message != WM_KEYDOWN || msg.key_pressed() {
            return;
        }

        for var in [D2UIvars::ChatBox, D2UIvars::EscMenu, D2UIvars::HoldAlt] {
            if D2Client::UI::GetUIVar(var) != 0 {
                return;
            }
        }

        let vk = msg.virtual_key();

        for cb in self.on_keydown_callbacks.iter() {
            if cb(vk) {
                break;
            }
        }
    }
}

pub fn init(_modules: &D2Modules) -> Result<(), HookError> {
    unsafe {
        inline_hook_jmp(0, D2Win::AddressTable.MsgHandler.RegisterMsgHandler, RegisterMsgHandler as usize, Some(&mut STUBS.RegisterMsgHandler), None)?;
    }

    Ok(())
}
