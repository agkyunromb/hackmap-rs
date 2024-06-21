use super::common::*;
use super::HackMap;
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
        nChatType       : u8,
        nLanguageCode   : u8,
        nUnitType       : u8,
        nUnitGUID       : i32,
        nChatColor      : D2StringColorCodes,
        nChatSubType    : u8,
        // std::string szNick;
        // std::string szMessage;
    }

    let chat = unsafe { &*(payload as *const D2GS_CHAT) };

    loop {
        if chat.nChatType != 3 {
            break;
        }

        if chat.nLanguageCode != 0 {
            break;
        }

        if chat.nUnitType != 0 {
            break;
        }

        if chat.nUnitGUID != 0 {
            break;
        }

        if chat.nChatColor != D2StringColorCodes::Red {
            break;
        }

        if chat.nChatSubType != 1 {
            break;
        }

        let ActiveMessage: &[u8] = &[0x15, 0x01, 0x09, b'1', 0x00, 0x00, 0x00];

        D2Client::Net::SendPacket(ActiveMessage.as_ptr() as PVOID, ActiveMessage.len());

        break;
    }

    get_stubs().Handle_D2GS_CHAT_26.unwrap()(payload);
}

pub fn init(_modules: &D2Modules) -> Result<(), HookError> {
    // let D2Sigma = modules.D2Sigma.unwrap();

    unsafe {
        STUBS.Handle_D2GS_CHAT_26 = Some(D2Client::Net::SwapD2GSHandler(0x04, Handle_D2GS_CHAT_26));
    }

    Ok(())
}
