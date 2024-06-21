use super::common::*;
use super::HackMap;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetKeyState;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_CONTROL;
use D2Win::MsgHandler::{StormMsgHandler, StormMsgHandlerParams};

struct Stubs {
    SaveAndExitGame : Option<extern "fastcall" fn(i32, &HWND)>,
    CreateControl   : Option<extern "stdcall" fn(*const D2Win::Control::D2WinControlInitStrc) -> PVOID>,
    EnterBNLobby    : Option<extern "stdcall" fn() -> BOOL>,
}

static mut STUBS: Stubs = Stubs{
    SaveAndExitGame : None,
    CreateControl   : None,
    EnterBNLobby    : None,
};

#[allow(static_mut_refs)]
fn get_stubs() -> &'static Stubs {
    unsafe { &STUBS }
}

extern "fastcall" fn SaveAndExitGame(_: i32, hwnd: &HWND) {
    HackMap::get().on_save_and_exit();
    get_stubs().SaveAndExitGame.unwrap()(0, hwnd);
}

extern "stdcall" fn CreateControl(init_info: &D2Win::Control::D2WinControlInitStrc) -> PVOID {
    let ctrl = get_stubs().CreateControl.unwrap()(init_info);

    if ctrl.is_null() == false {
        HackMap::get().on_create_lobby_controls(ctrl, init_info);
    }

    ctrl
}

extern "stdcall" fn EnterBNLobby() -> BOOL {
    if get_stubs().EnterBNLobby.unwrap()() == FALSE {
        return FALSE;
    }

    let _ = HackMap::get().on_enter_lobby();

    TRUE
}

impl HackMap {
    fn on_save_and_exit(&mut self) {
        self.generate_next_game_info(0);
    }

    fn generate_next_game_info(&mut self, delta: i32) {
        let game_info = D2Client::Game::GetGameInfo();
        let name = game_info.get_name();

        if name.is_empty() {
            return;
        }

        if delta != 0 && unsafe { GetKeyState(VK_SHIFT as i32) } >= 0 {
            self.quick_next_game.auto_create_game = true;
        }

        let mut end = name.len();

        for (_, c) in name.char_indices().rev() {
            if c.is_digit(10) == false {
                break;
            }

            end -= 1;
        }

        let index = &name[end..];
        let name = &name[..end];

        self.quick_next_game.auto_game_name = name[..end].to_string();
        self.quick_next_game.auto_game_index = if index.is_empty() { None } else { Some(index.parse::<i32>().unwrap() + delta) };
        self.quick_next_game.auto_game_password = game_info.get_password();
    }

    fn on_create_lobby_controls(&mut self, ctrl: PVOID, init_info: &D2Win::Control::D2WinControlInitStrc) {
        match init_info.ctrl_type {
            D2ControlTypes::Editbox => {
                /*
                    32 EDITBOX  创建 - 名称
                    x = 548
                    y = 199
                    w = 158
                    h = 20

                    33 EDITBOX  创建 - 密码
                    x = 778
                    y = 199
                    w = 158
                    h = 20

                    64 EDITBOX  加入 - 名称
                    x = 548
                    y = 165
                    w = 155
                    h = 20
                    cb = 00000000

                    65 EDITBOX  加入 - 密码
                    x = 778
                    y = 165
                    w = 155
                    h = 20
                    cb = 00000000
                */

                if (init_info.x == 548 && init_info.y == 199 && init_info.width == 158 && init_info.height == 20) ||   // create game name
                   (init_info.x == 548 && init_info.y == 165 && init_info.width == 155 && init_info.height == 20)      // join game name
                {
                    match self.quick_next_game.auto_game_index {
                        Some(index) => {
                            D2Win::EditBox::SetTextW(ctrl, format!("{}{}", self.quick_next_game.auto_game_name, index).to_utf16().as_ptr());
                        },

                        None => {
                            D2Win::EditBox::SetTextW(ctrl, self.quick_next_game.auto_game_name.to_utf16().as_ptr());
                        },
                    }

                    D2Win::EditBox::SelectAll(ctrl);

                } else if (init_info.x == 778 && init_info.y == 199 && init_info.width == 158 && init_info.height == 20) ||     // create game password
                          (init_info.x == 778 && init_info.y == 165 && init_info.width == 155 && init_info.height == 20)        // join game password
                {
                    D2Win::EditBox::SetTextW(ctrl, self.quick_next_game.auto_game_password.to_utf16().as_ptr());
                    D2Win::EditBox::SelectAll(ctrl);
                }
            },

            D2ControlTypes::Button => {

                /*
                    23 BUTTON   创建
                    x = 583
                    y = 605
                    w = 205
                    h = 25

                    25 BUTTON   加入
                    x = 793
                    y = 605
                    w = 205
                    h = 25

                    26 BUTTON   天梯
                    x = 583
                    y = 635
                    w = 205
                    h = 25

                    27 BUTTON   退出
                    x = 793
                    y = 635
                    w = 205
                    h = 25

                    30 BUTTON   创建游戏
                    x = 713
                    y = 546
                    w = 272
                    h = 30
                    cb = 6F9E42C0
                */

                if init_info.x == 583 && init_info.y == 605 && init_info.width == 205 && init_info.height == 25 {
                    // lobby create game tab button
                    self.quick_next_game.on_create_game_tab_button_clicked = Some(init_info.perform);

                } else if init_info.x == 713 && init_info.y == 546 && init_info.width == 272 && init_info.height == 30 {
                    self.quick_next_game.create_game_button = Some(ctrl);
                    self.quick_next_game.on_create_game_button_clicked = Some(init_info.perform);
                }
            },

            _ => {},
        }
    }

    fn on_enter_lobby(&mut self) -> Result<(), ()> {
        let quick_next_game = &mut self.quick_next_game;

        if quick_next_game.auto_create_game == false {
            return Ok(());
        }

        let on_create_game_tab_button_clicked = quick_next_game.on_create_game_tab_button_clicked.ok_or(())?;

        if on_create_game_tab_button_clicked(null_mut()) == FALSE {
            return Err(());
        }

        let on_create_game_button_clicked = quick_next_game.on_create_game_button_clicked.ok_or(())?;
        let create_game_button = quick_next_game.create_game_button.ok_or(())?;

        on_create_game_button_clicked(create_game_button);

        quick_next_game.on_create_game_button_clicked       = None;
        quick_next_game.on_create_game_tab_button_clicked   = None;
        quick_next_game.create_game_button                  = None;
        quick_next_game.auto_create_game                    = false;

        Ok(())
    }

}

pub fn init(_modules: &D2Modules) -> Result<(), HookError> {
    HackMap::get().on_key_down(|vk| {
        if vk == VK_OEM_PLUS {
            HackMap::get().generate_next_game_info(if unsafe { GetKeyState(VK_CONTROL as i32) } < 0 { 0 } else { 1 });
            let hwnd = D2Gfx::Window::GetWindow();
            get_stubs().SaveAndExitGame.unwrap()(0, &hwnd);
        }

        false
    });

    unsafe {
        inline_hook_jmp(0, D2Client::AddressTable.Game.SaveAndExitGame, SaveAndExitGame as usize, Some(&mut STUBS.SaveAndExitGame), None)?;
        inline_hook_jmp(0, D2Win::AddressTable.Control.CreateControl, CreateControl as usize, Some(&mut STUBS.CreateControl), None)?;
        inline_hook_jmp(0, D2Multi::AddressTable.BNet.EnterBNLobby, EnterBNLobby as usize, Some(&mut STUBS.EnterBNLobby), None)?;
    }

    Ok(())
}
