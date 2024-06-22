use std::ptr::addr_of_mut;

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

use D2Client::AutoMap::{D2AutomapCellBlock, D2AutomapCellData};

enum CellDataEx {
    LevelId(i32),
    ShrineType(i32),
}

#[repr(C, packed(1))]
struct D2AutomapCellDataEx {
    pub data        : D2AutomapCellData,
    pub data_extra  : CellDataEx,

    _pad            : [u32; 1],
}

const _: () = {assert!(0x2800 % size_of::<D2AutomapCellDataEx>() == 0);};

#[repr(C, packed(1))]
struct D2AutomapCellBlockEx {
    pub Elements    : [D2AutomapCellDataEx; 0x2800 / std::mem::size_of::<D2AutomapCellDataEx>()],
    pub NextBlock   : *mut D2AutomapCellBlockEx,
}

fn new_automap_cell(g_cell_block_head: *mut *mut D2AutomapCellBlock, g_automap_cell_count: &mut usize) -> *mut D2AutomapCellData {
    unsafe {
        let element_count = &(**g_cell_block_head).Elements.len();
        let block_count = *g_automap_cell_count / element_count;
        let block_index = *g_automap_cell_count % element_count;

        *g_automap_cell_count += 1;

        let mut head: *mut D2AutomapCellBlock = *g_cell_block_head;
        let mut prev: *mut D2AutomapCellBlock = null_mut();

        for _ in 0..block_count {
            prev = head;
            head = (&*head).NextBlock;
            if head.is_null() {
                break;
            }
        }

        if head.is_null() {
            head = Fog::Alloc(std::mem::size_of_val(&*head));
            std::ptr::write_bytes(head, 0, 1);
            if prev.is_null() {
                *g_cell_block_head = head;
            } else {
                (&mut *prev).NextBlock = head;
            }
        }

        let cell = &mut (&mut *head).Elements[block_index];
        std::ptr::write_bytes(cell, 0, 1);

        cell
    }
}

extern "stdcall" fn NewAutomapCell() -> *mut D2AutomapCellData {
    new_automap_cell(D2Client::AutoMap::AutoMapCellBlockHead(), D2Client::AutoMap::AutoMapCellCount())
}

pub fn init(_modules: &D2Modules) -> Result<(), HookError> {
    // let D2Sigma = modules.D2Sigma.unwrap();

    unsafe {
        if D2Sigma::initialized() {
            STUBS.Handle_D2GS_LOADCOMPLETE_04 = Some(D2Client::Net::SwapD2GSHandler(0x04, Handle_D2GS_LOADCOMPLETE_04));
        }

        inline_hook_jmp::<()>(0, D2Client::AddressTable.AutoMap.NewAutomapCell, NewAutomapCell as usize, None, None)?;
    }

    Ok(())
}
