use super::common::*;
use super::config_deserializer::VirtualKeyCode;
use super::HackMap;
use super::config::ConfigRef;
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
    HackMap::input().in_game_key_down(msg);
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

pub(super) type OnKeyDownCallback = fn(vk: u16) -> bool;

pub(super) struct Input {
    cfg                     : ConfigRef,
    on_keydown_callbacks    : Vec<Box<dyn FnMut(u16) -> bool>>,
    toggles                 : Vec<(&'static str, Box<dyn FnMut(u16) -> (bool, bool)>)>,
}

impl Input {
    pub fn new(cfg: ConfigRef) -> Self{
        Self{
            cfg,
            on_keydown_callbacks: vec![],
            toggles: vec![],
        }
    }

    pub fn reg_toggle<F: FnMut(u16) -> (bool, bool) + 'static>(&mut self, name: &'static str, cb: F) {
        self.toggles.push((name, Box::new(cb)));
    }

    pub fn on_key_down(&mut self, f: OnKeyDownCallback) {
        self.on_keydown_callbacks.push(Box::new(f));
    }

    fn in_game_key_down(&mut self, msg: &mut StormMsgHandlerParams) {
        if msg.returned != FALSE || msg.message != WM_KEYDOWN || msg.key_pressed() {
            return;
        }

        for var in [D2UIvars::ChatBox, D2UIvars::EscMenu, D2UIvars::HoldAlt] {
            if D2Client::UI::GetUIVar(var) != 0 {
                return;
            }
        }

        let vk = msg.virtual_key();

        for (name, cb) in self.toggles.iter_mut() {
            let (handled, toggle_enabled) = cb(vk);
            if handled == false {
                continue;
            }

            let toggle_state = if toggle_enabled { "ON" } else { "OFF" };
            let toggle_color = if toggle_enabled { D2StringColorCodes::LightGreen } else { D2StringColorCodes::Red };

            D2Client::UI::DisplayQuickMessage(
                &format!("{name} -> {}{toggle_state}", toggle_color.to_str_code()),
                D2StringColorCodes::Orange,
            );
        }

        for cb in self.on_keydown_callbacks.iter_mut() {
            if cb(vk) {
                break;
            }
        }
    }

    pub fn init(&mut self, _modules: &D2Modules) -> Result<(), HookError> {
        unsafe {
            inline_hook_jmp(0, D2Win::AddressTable.MsgHandler.RegisterMsgHandler, RegisterMsgHandler as usize, Some(&mut STUBS.RegisterMsgHandler), None)?;
        }

        Ok(())
    }
}
