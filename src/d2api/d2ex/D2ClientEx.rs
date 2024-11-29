use std::cell::RefCell;
use std::rc::Rc;
use std::sync::OnceLock;

use crate::d2api::d2consts::*;
use crate::d2api::d2ex::common::*;

#[derive(PartialEq)]
enum GameEvent {
    JoinGame,
    GameLoop,
    LeaveGame,
}

struct NetInfo {
    pre_send_callbacks: Vec<Box<dyn FnMut(D2ClientCmd, *mut u8)>>,
    post_send_callbacks: Vec<Box<dyn FnMut(D2ClientCmd, *mut u8)>>,
    pre_recv_callbacks: Vec<Box<dyn FnMut(D2GSCmd, *mut u8)>>,
    post_recv_callbacks: Vec<Box<dyn FnMut(D2GSCmd, *mut u8)>>,
    send_callbacks: Vec<Box<dyn FnMut(D2GSCmd, *mut u8)>>,

    stub_send_packet: Option<extern "stdcall" fn(u32, u32, *const u8) -> usize>,
}

struct GameInfo {
    event_callbacks: Vec<Box<dyn FnMut(GameEvent)>>,
    is_in_loop: bool,

    stub_d2_sound_cleanup: Option<extern "stdcall" fn()>,
}

struct D2ClientEx {
    net: NetInfo,
    game: GameInfo,
}

impl D2ClientEx {
    const fn new() -> Self {
        Self {
            game: GameInfo {
                event_callbacks: vec![],
                is_in_loop: false,
                stub_d2_sound_cleanup: None,
            },
            net: NetInfo {
                pre_send_callbacks: vec![],
                post_send_callbacks: vec![],
                pre_recv_callbacks: vec![],
                post_recv_callbacks: vec![],
                send_callbacks: vec![],

                stub_send_packet: None,
            },
        }
    }

    #[allow(static_mut_refs)]
    pub fn get() -> &'static mut Self {
        static mut OBJ: D2ClientEx = D2ClientEx::new();

        unsafe { &mut OBJ }
    }
}

pub mod Net {
    use super::*;
    use D2Client::Net::D2GSHandler;

    impl NetInfo {
        fn on_pre_send<F: FnMut(D2ClientCmd, *mut u8) + 'static>(&mut self, cb: F) {
            self.pre_send_callbacks.push(Box::new(cb));
        }

        fn on_post_send<F: FnMut(D2ClientCmd, *mut u8) + 'static>(&mut self, cb: F) {
            self.post_send_callbacks.push(Box::new(cb));
        }

        fn on_pre_recv<F: FnMut(D2GSCmd, *mut u8) + 'static>(&mut self, cb: F) {
            self.pre_recv_callbacks.push(Box::new(cb));
        }

        fn on_post_recv<F: FnMut(D2GSCmd, *mut u8) + 'static>(&mut self, cb: F) {
            self.post_recv_callbacks.push(Box::new(cb));
        }

        pub fn on_send_packet(&mut self, size: u32, arg2: u32, payload: *mut u8) -> usize {
            let cmd: D2ClientCmd = read_at(payload as usize);

            for cb in self.pre_send_callbacks.iter_mut() {
                cb(cmd, payload);
            }

            let ret = self.stub_send_packet.unwrap()(size, arg2, payload);

            for cb in self.post_send_callbacks.iter_mut() {
                cb(cmd, payload);
            }

            ret
        }

        fn call_gscmd_handler(&mut self, handler: D2GSHandler, payload: *mut u8) {
            let cmd: D2GSCmd = read_at(payload as usize);

            for cb in self.pre_recv_callbacks.iter_mut() {
                cb(cmd, payload);
            }

            handler(payload);

            for cb in self.post_recv_callbacks.iter_mut() {
                cb(cmd, payload);
            }
        }
    }

    pub fn on_pre_recv<F: FnMut(D2GSCmd, *mut u8) + 'static>(cb: F) {
        D2ClientEx::get().net.on_pre_recv(cb)
    }

    pub fn on_post_recv<F: FnMut(D2GSCmd, *mut u8) + 'static>(cb: F) {
        D2ClientEx::get().net.on_post_recv(cb)
    }

    pub fn send_packet<T>(payload: &T) -> usize {
        let slice = unsafe {
            std::slice::from_raw_parts(addr_of!(*payload) as *const u8, std::mem::size_of::<T>())
        };

        D2Client::Net::SendPacket(slice.as_ptr(), slice.len())
    }

    pub extern "stdcall" fn on_send_packet(size: u32, arg2: u32, payload: *mut u8) -> usize {
        D2ClientEx::get().net.on_send_packet(size, arg2, payload)
    }

    #[cfg(feature = "113c")]
    pub(super) fn call_gscmd_handler() {
        let handler: D2GSHandler;
        let payload: *mut u8;

        unsafe {
            asm!(
                "",
                out("eax") handler,
                out("edi") payload,
            );
        }

        D2ClientEx::get().net.call_gscmd_handler(handler, payload)
    }
}

pub mod Game {
    use super::*;

    impl GameInfo {
        fn on_join_game<F: FnMut() + 'static>(&mut self, mut cb: F) {
            self.on(move |event| {
                if event == GameEvent::JoinGame {
                    cb();
                }
            })
        }

        fn on_game_loop<F: FnMut() + 'static>(&mut self, mut cb: F) {
            self.on(move |event| {
                if event == GameEvent::GameLoop {
                    cb();
                }
            })
        }

        fn on_leave_game<F: FnMut() + 'static>(&mut self, mut cb: F) {
            self.on(move |event| {
                if event == GameEvent::LeaveGame {
                    cb();
                }
            })
        }

        fn on<F: FnMut(GameEvent) + 'static>(&mut self, cb: F) {
            self.event_callbacks.push(Box::new(cb));
        }

        pub(super) fn join_game_pre_recv_callback(&mut self, cmd: D2GSCmd, _payload: *mut u8) {
            if self.is_in_loop {
                return;
            }

            if cmd == D2GSCmd::GAME_LOADING {
                self.is_in_loop = true;

                for cb in self.event_callbacks.iter_mut() {
                    cb(GameEvent::JoinGame);
                }
            }
        }

        fn run_game_loop(&mut self) -> i32 {
            if self.is_in_loop {
                for cb in self.event_callbacks.iter_mut() {
                    cb(GameEvent::GameLoop);
                }
            }

            D2Gfx::Window::GetState()
        }

        fn d2_sound_cleanup(&mut self) {
            self.is_in_loop = false;

            self.stub_d2_sound_cleanup.unwrap()();

            for cb in self.event_callbacks.iter_mut() {
                cb(GameEvent::LeaveGame);
            }
        }
    }

    pub fn on_join_game<F: FnMut() + 'static>(cb: F) {
        D2ClientEx::get().game.on_join_game(cb);
    }

    pub fn on_game_loop<F: FnMut() + 'static>(cb: F) {
        D2ClientEx::get().game.on_game_loop(cb);
    }

    pub fn on_leave_game<F: FnMut() + 'static>(cb: F) {
        D2ClientEx::get().game.on_leave_game(cb);
    }

    pub(super) fn d2_sound_cleanup() {
        D2ClientEx::get().game.d2_sound_cleanup()
    }

    pub(super) fn run_game_loop() -> i32 {
        D2ClientEx::get().game.run_game_loop()
    }
}
pub mod Inventory {
    use D2Common::D2Unit;

    use super::*;

    pub fn get_free_position_for_item(
        item: &D2Unit,
        inv_page: D2ItemInvPage,
    ) -> Option<(i32, i32)> {
        let player = D2Client::Units::GetClientPlayer()?;

        D2Common::Inventory::GetFreePosition(
            ptr_to_ref(player.pInventory)?,
            item,
            D2Common::Units::GetInventoryRecordId(player, inv_page, D2Client::Game::IsLodGame()),
            D2ItemInvPage::Cube,
        )
    }
}

pub mod Utils {
    use super::*;
    use crate::D2CommonEx;
    use D2Common::D2Unit;

    pub fn send_pickup_item(item: &D2Unit, to_cursor: bool) {
        let cmd = D2Common::SCMD_PACKET_16_PIKCUP_ITEM {
            nHeader: D2ClientCmd::PICKUP_ITEM as u8,
            dwUnitType: D2UnitTypes::Item as u32,
            dwUnitGUID: item.dwUnitId,
            bCursor: if to_cursor { 1 } else { 0 },
        };

        super::Net::send_packet(&cmd);
    }

    pub fn send_drop_item(item: &D2Unit) {
        let cmd = D2Common::SCMD_PACKET_17_DROP_CURSOR_ITEM {
            nHeader: D2ClientCmd::DROP_ITEM as u8,
            dwItemGUID: item.dwUnitId,
        };

        super::Net::send_packet(&cmd);
    }

    pub fn send_use_item(player: &D2Unit, item: &D2Unit) {
        let coord = D2Common::Units::GetCoords(player);

        let cmd = D2Common::SCMD_PACKET_20_USE_ITEM {
            nHeader: D2ClientCmd::USE_ITEM as u8,
            nItemGUID: item.dwUnitId,
            nPosX: coord.nX,
            nPosY: coord.nY,
        };

        super::Net::send_packet(&cmd);
    }

    pub fn send_cursor_item_to_cube(cursor_item: &D2Unit, cube: &D2Unit) {
        let payload = D2Common::SCMD_PACKET_2A_ITEM_TO_CUBE {
            nHeader: D2ClientCmd::ITEM_TO_CUBE as u8,
            dwItemGUID: cursor_item.dwUnitId,
            dwCubeGUID: cube.dwUnitId,
        };

        super::Net::send_packet(&payload);
    }

    pub fn send_add_skill(skill_id: u16) {
        let payload = D2Common::SCMD_PACKET_3B_ADD_SKILL {
            nHeader: D2ClientCmd::ADD_SKILL as u8,
            nSkillID: skill_id,
        };

        super::Net::send_packet(&payload);
    }

    pub fn send_item_to_belt(item: &D2Unit) {
        let cmd = D2Common::SCMD_PACKET_63_ITEM_TO_BELT {
            nHeader: D2ClientCmd::ITEM_TO_BELT as u8,
            dwItemGUID: item.dwUnitId,
        };

        super::Net::send_packet(&cmd);
    }

    pub fn cursor_item_to_cube() -> Option<()> {
        let cursor_item = D2CommonEx::Inventory::get_player_cursor_item()?;
        let cube = get_cube_from_inv()?;

        super::Inventory::get_free_position_for_item(cursor_item, D2ItemInvPage::Cube)?;

        send_cursor_item_to_cube(cursor_item, cube);

        None
    }

    pub fn use_item(player: &D2Unit, item: &D2Unit) {
        if D2Common::Items::GetBaseCode(item) == D2ItemCodes::Cube
            && D2Client::UI::GetUIVar(D2UIvars::Stash) != 0
        {
            D2Client::UI::SetIsStashOpened(1);
        }

        send_use_item(player, item);
        D2Client::Units::SetUnitUninterruptable(item);
        D2Client::UI::PlaySound(D2Client::Units::GetItemUseSound(item));
    }

    pub fn get_cube_from_inv() -> Option<&'static mut D2Unit> {
        let player = D2Client::Units::GetClientPlayer()?;

        D2CommonEx::Inventory::iter_inventory(player, |_inventory, item| {
            if D2Common::Inventory::UnitIsItem(item) == FALSE {
                return false;
            }

            match D2Common::Items::GetInvPage(item) {
                D2ItemInvPage::Inventory | D2ItemInvPage::Stash => {}
                _ => return false,
            }

            if D2Common::Items::GetBaseCode(item) != D2ItemCodes::Cube {
                return false;
            }

            true
        })

        // let mut opt_item = D2Common::Inventory::GetFirstItem(ptr_to_ref(player.pInventory)?);

        // while let Some(item) = opt_item {
        //     loop {
        //         if D2Common::Inventory::UnitIsItem(item) == FALSE {
        //             break;
        //         }

        //         match D2Common::Items::GetInvPage(item) {
        //             D2ItemInvPage::Inventory | D2ItemInvPage::Stash => {},
        //             _ => break,
        //         }

        //         if D2Common::Items::GetBaseCode(item) != D2ItemCodes::Cube {
        //             break;
        //         }

        //         return Some(item);
        //     }
        //     opt_item = D2Common::Inventory::GetNextItem(item);
        // }

        // None
    }
}

pub(super) fn init(_modules: &D2Modules) -> Result<(), HookError> {
    let cli = D2ClientEx::get();

    inline_hook_call::<()>(
        0,
        D2Client::AddressTable.Game.Call_D2GFX_GetWindow,
        Game::run_game_loop as usize,
        None,
        None,
    )?;
    inline_hook_call(
        0,
        D2Client::AddressTable.Game.Call_D2SoundCleanup,
        Game::d2_sound_cleanup as usize,
        Some(&mut cli.game.stub_d2_sound_cleanup),
        None,
    )?;
    inline_hook_call::<()>(
        0,
        D2Client::AddressTable.Net.Call_GSCmdHandler,
        Net::call_gscmd_handler as usize,
        None,
        None,
    )?;
    inline_hook_jmp(
        0,
        D2Client::AddressTable.Net.DoSendPacket,
        Net::on_send_packet as usize,
        Some(&mut cli.net.stub_send_packet),
        None,
    )?;

    Net::on_pre_recv(move |cmd, payload| {
        cli.game.join_game_pre_recv_callback(cmd, payload);
    });

    Ok(())
}
