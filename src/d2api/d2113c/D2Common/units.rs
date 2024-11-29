use super::super::super::{d2consts::*, types::*};
use super::datatbls::*;
use super::drlg::*;

#[repr(C, packed(1))]
pub union D2Unit_10 {
    pub dwAnimMode: u32,
    pub dwItemMode: u32,
    pub dwCollideType: u32,
}

#[repr(C, packed(1))]
pub struct D2MonsterData {
    pub pMonstatsTxt: *mut D2MonStatsTxt, // 0x00
    pub nComponent: [u8; 16],             // 0x0004
    pub wNameSeed: u16,                   // 0x0014
    pub nTypeFlag: D2MonTypeFlags,        // 0x0016
    pub nLastAnimMode: u8,                // 0x0017
    pub dwDurielFlag: u32,                // 0x0018
    pub nMonUmod: [D2MonUMods; 10],       // 0x001C
    pub wBossHcIdx: u16,                  // 0x0026
}

impl D2MonsterData {
    pub fn get_mon_stats_txt(&self) -> Option<&mut D2MonStatsTxt> {
        ptr_to_ref_mut(self.pMonstatsTxt)
    }
}

#[repr(C, packed(1))]
pub union D2Unit_Data_14 {
    pub pMonsterData: *mut D2MonsterData,
}

#[repr(C, packed(4))]
pub struct D2Unit {
    pub dwUnitType: D2UnitTypes,      // 0x00
    pub dwClassId: u32,               // 0x04
    pub pMemoryPool: *const u8,       // 0x08
    pub dwUnitId: u32,                // 0x0C
    pub Mode: D2Unit_10,              // 0x10
    pub Data: D2Unit_Data_14,         // 0x14
    pub nAct: u8,                     // 0x18
    pub _pad_19_1C: [u8; 3],          // 0x19
    pub pDrlgAct: *mut D2DrlgAct,     // 0x1C
    _pad_20_60: [u8; 0x40],           // 0x20
    pub pInventory: *mut D2Inventory, // 0x60
    _pad_64_94: [u8; 0x30],           // 0x64
    pub dwOwnerType: D2UnitTypes,     // 0x94
    pub dwOwnerGUID: u32,             // 0x98
    _pad_9C_C4: [u8; 0x28],           // 0x64
    pub dwFlags: D2UnitFlags,         // 0xC4
    pub dwFlagEx: D2UnitFlagsEx,      // 0xC8
    _pad_CC_E8: [u8; 0x1C],           // 0xCC
    pub pRoomNext: *mut D2Unit,       // 0xE8
}

impl D2Unit {
    pub fn get_drlg_act(&self) -> &D2DrlgAct {
        unsafe { &mut *self.pDrlgAct }
    }

    pub fn get_monster_data(&self) -> Option<&mut D2MonsterData> {
        unsafe { ptr_to_ref_mut(self.Data.pMonsterData) }
    }
}

#[repr(C, packed(4))]
pub struct D2Inventory {}

#[repr(C, packed(4))]
pub struct D2Coord {
    pub nX: i32,
    pub nY: i32,
}
