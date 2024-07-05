use super::common::*;
use super::HackMap;
use super::config::ConfigRef;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetKeyState;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_CONTROL;
use D2Win::MsgHandler::{StormMsgHandler, StormMsgHandlerParams};

struct Stubs {
    Handle_D2GS_CHAT_26 : Option<D2Client::Net::D2GSHandler>,
}

static mut STUBS: Stubs = Stubs{
    Handle_D2GS_CHAT_26 : None,
};

#[allow(static_mut_refs)]
fn get_stubs() -> &'static Stubs {
    unsafe { &STUBS }
}

extern "fastcall" fn Handle_D2GS_CHAT_26(payload: *const u8) {
    #[repr(C, packed(1))]
    struct D2GS_CHAT {
        PacketId        : u8,
        chat_type       : u8,
        language_code   : u8,
        unit_type       : u8,
        unit_guid       : i32,
        chat_color      : D2StringColorCodes,
        chat_sub_type    : u8,
        // std::string szNick;
        // std::string szMessage;
    }

    let chat = unsafe { &*(payload as *const D2GS_CHAT) };

    loop {
        if chat.chat_type != 3 {
            break;
        }

        if chat.language_code != 0 {
            break;
        }

        if chat.unit_type != 0 {
            break;
        }

        if chat.unit_guid != 0 {
            break;
        }

        if chat.chat_color != D2StringColorCodes::Red {
            break;
        }

        if chat.chat_sub_type != 1 {
            break;
        }

        let ActiveMessage: &[u8] = &[0x15, 0x01, 0x09, b'1', 0x00, 0x00, 0x00];

        D2Client::Net::SendPacket(ActiveMessage.as_ptr(), ActiveMessage.len());

        break;
    }

    get_stubs().Handle_D2GS_CHAT_26.unwrap()(payload);
}

pub(super) struct HelperBot {
    pub cfg: super::config::ConfigRef,
}

impl HelperBot {
    pub fn new(cfg: ConfigRef) -> Self{
        Self{
            cfg,
        }
    }

    pub fn init(&mut self, _modules: &D2Modules) -> Result<(), HookError> {
        // let D2Sigma = modules.D2Sigma.unwrap();

        unsafe {
            STUBS.Handle_D2GS_CHAT_26 = Some(D2Client::Net::SwapD2GSHandler(0x26, Handle_D2GS_CHAT_26));
        }

        Ok(())
    }
}
