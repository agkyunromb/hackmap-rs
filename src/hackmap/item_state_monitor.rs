use super::common::*;
use D2Common::{SCMD_PACKET_9C_ITEMACTION, SCMD_PACKET_9D_ITEM_OWNED, D2ItemActionType};

pub(super) struct ItemStateMonitor {
    pub unit_id             : u32,  // unitId == 0 means match all units
    pub storage_to_cursor   : bool,
    pub cursor_to_ground    : bool,
    pub cursor_to_storage   : bool,
    pub ground_to_cursor    : bool,
    pub add_to_ground       : bool,
    pub put_in_belt         : bool,
    pub remove_from_belt    : bool,
}

impl ItemStateMonitor {
    pub const fn new() -> Self {
        Self {
            unit_id             : 0,
            storage_to_cursor   : false,
            cursor_to_ground    : false,
            cursor_to_storage   : false,
            ground_to_cursor    : false,
            put_in_belt         : false,
            remove_from_belt    : false,
            add_to_ground       : false,
        }
    }

    pub fn reset(&mut self, unit_id: u32) {
        self.unit_id            = unit_id;
        self.storage_to_cursor  = false;
        self.cursor_to_ground   = false;
        self.cursor_to_storage  = false;
        self.ground_to_cursor   = false;
        self.put_in_belt        = false;
        self.remove_from_belt   = false;
        self.add_to_ground      = false;
    }

    pub fn on_scmd(&mut self, cmd: D2GSCmd, payload: *const u8) {
        match cmd {
            D2GSCmd::ITEM_ACTION => {
                self.on_item_action(unsafe { &*(payload as *const SCMD_PACKET_9C_ITEMACTION) });
            },

            D2GSCmd::ITEM_OWNED => {
                self.on_item_owned(unsafe { &*(payload as *const SCMD_PACKET_9D_ITEM_OWNED) });
            },

            _ => {},
        }
    }

    pub fn on_item_action(&mut self, item_action: &SCMD_PACKET_9C_ITEMACTION) {
        if self.unit_id != 0 && self.unit_id != item_action.nItemId {
            return;
        }

        match item_action.nAction {
            D2ItemActionType::AddToGround => {
                self.add_to_ground = true;
            },

            D2ItemActionType::GroundToCursor => {
                self.ground_to_cursor = true;
            },

            D2ItemActionType::DropToGround => {
                self.cursor_to_ground = true;
            },

            D2ItemActionType::PutInContainer => {
                self.cursor_to_storage = true;
            },

            D2ItemActionType::PutInBelt => {
                self.put_in_belt = true;
            },

            D2ItemActionType::RemoveFromBelt => {
                self.remove_from_belt = true;
            },

            _ => {},
        }

        self.unit_id = item_action.nItemId;
    }

    pub fn on_item_owned(&mut self, item_owned: &SCMD_PACKET_9D_ITEM_OWNED) {
        if self.unit_id != 0 && self.unit_id != item_owned.nItemId {
            return;
        }

        match item_owned.nAction {
            D2ItemActionType::RemoveFromContainer => {
                self.storage_to_cursor = true;
            },

            _ => {},
        }

        self.unit_id = item_owned.nItemId;
    }

}
