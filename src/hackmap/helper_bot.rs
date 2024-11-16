use std::collections::HashMap;
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use super::common::*;
use super::item_state_monitor::ItemStateMonitor;
use super::HackMap;
use super::config::ConfigRef;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetKeyState, VK_CONTROL};
use D2Win::MsgHandler::{StormMsgHandler, StormMsgHandlerParams};
use D2Common::{SCMD_PACKET_3F_USE_STACKABLE_ITEM, D2Inventory, D2Unit};

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

fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
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

    items_removing              : HashMap<u32, u64>,
    next_fill_belt_timestamp    : u64,
    belt_free_slots             : u32,
}

impl HelperBot {
    pub fn new(cfg: ConfigRef) -> Self{
        Self {
            cfg,
            items_removing          : HashMap::new(),
            next_fill_belt_timestamp: 0,
            belt_free_slots         : 0,
        }
    }

    fn on_auto_item_to_belt(&mut self) {
        if self.belt_free_slots == 0 {
            return;
        }

        let current_timestamp = get_current_timestamp();

        if self.next_fill_belt_timestamp > current_timestamp {
            return;
        }

        const AutoFillInterval: u64 = 500;

        self.next_fill_belt_timestamp = current_timestamp + AutoFillInterval;

        if D2CommonEx::Inventory::get_player_cursor_item().is_some() {
            return;
        }

        let player = match D2Client::Units::GetClientPlayer() {
            Some(p) => p,
            None => return,
        };

        self.items_removing.retain(|_, &mut ts| ts < current_timestamp);

        self.belt_free_slots -= 1;

        D2CommonEx::Inventory::iter_inventory(player, |inventory, item| {
            if D2Common::Items::GetInvPage(item) != D2ItemInvPage::Inventory {
                return false;
            }

            if self.items_removing.contains_key(&item.dwUnitId) {
                return false;
            }

            match D2Common::Inventory::GetFreeBeltSlot(inventory, item) {
                Some(_) => {
                    D2ClientEx::Utils::send_item_to_belt(item);
                    self.items_removing.insert(item.dwUnitId, current_timestamp + 1000 * 5);
                    true
                },
                None => false,
            }
        });
    }

    fn on_use_stackable_item(&mut self, cmd: D2GSCmd, payload: *const u8) {
        if HackMap::config().borrow().tweaks.auto_item_to_belt == false {
            return;
        }

        let mut state = ItemStateMonitor::new();

        state.on_scmd(cmd, payload);

        if state.put_in_belt {
            self.items_removing.remove(&state.unit_id);
            return;
        }

        if state.remove_from_belt == false {
            return;
        }

        // let player = match D2Client::Units::GetClientPlayer() {
        //     Some(p) => p,
        //     None => return,
        // };

        if D2CommonEx::Inventory::get_player_cursor_item().is_some() {
            return;
        }

        self.belt_free_slots += 1;

        // let current_timestamp = get_current_timestamp();

        // self.items_removing.retain(|_, &mut ts| ts < current_timestamp);

        // D2CommonEx::Inventory::iter_inventory(player, |inventory, item| {
        //     if D2Common::Items::GetInvPage(item) != D2ItemInvPage::Inventory {
        //         return false;
        //     }

        //     if self.items_removing.contains_key(&item.dwUnitId) {
        //         return false;
        //     }

        //     match D2Common::Inventory::GetFreeBeltSlot(inventory, item) {
        //         Some(_) => {
        //             D2ClientEx::Utils::send_item_to_belt(item);
        //             println!("item to belt: {}", item.dwUnitId);
        //             self.items_removing.insert(item.dwUnitId, current_timestamp + 15);
        //             true
        //         },
        //         None => false,
        //     }
        // });

    }

    fn on_fast_drop(&mut self) -> Option<()> {
        D2ClientEx::Utils::send_drop_item(D2CommonEx::Inventory::get_player_cursor_item()?);
        None
    }

    fn on_fast_transmute(&mut self) {
        if D2Client::UI::GetUIVar(D2UIvars::Cube) != 0 {
            return;
        }
    }

    pub fn init(&mut self, _modules: &D2Modules) -> Result<(), HookError> {
        unsafe {
            STUBS.Handle_D2GS_CHAT_26 = Some(D2Client::Net::SwapD2GSHandler(0x26, Handle_D2GS_CHAT_26));
        }

        D2ClientEx::Net::on_post_recv(|cmd, payload| {
            match cmd {
                D2GSCmd::ITEM_ACTION => {
                    HackMap::helper_bot().on_use_stackable_item(cmd, payload);
                    return;
                },

                _ => {},
            }
        });

        D2ClientEx::Game::on_join_game(|| {
            HackMap::helper_bot().belt_free_slots = 0;
        });

        D2ClientEx::Game::on_game_loop(|| {
            HackMap::helper_bot().on_auto_item_to_belt();
        });

        HackMap::input().reg_toggle("auto_item_to_belt", |vk| {
            let cfg = HackMap::config();
            let mut cfg = cfg.borrow_mut();

            if vk == cfg.hotkey.auto_item_to_belt {
                cfg.tweaks.auto_item_to_belt = !cfg.tweaks.auto_item_to_belt;
                return (true, cfg.tweaks.auto_item_to_belt)
            }

            (false, false)
        });

        HackMap::input().on_key_down(|vk| {
            let cfg = HackMap::config();
            let cfg = cfg.borrow();

            if vk != cfg.hotkey.fast_drop {
                return false;
            }

            HackMap::helper_bot().on_fast_drop();

            false
        });

        Ok(())
    }
}
