use super::super::super::d2consts::*;
use super::datatbls::*;
use super::Units::D2Unit;

pub type D2LevelId = i32;

#[repr(C, packed(1))]
pub struct D2Drlg {

}

#[repr(C, packed(1))]
pub struct D2DrlgAct {
        _pad_00_14      : [u8; 0x14],               // 0x00
    pub nAct            : u8,                       // 0x14
        _pad_15_18      : [u8; 0x03],               // 0x15
}

#[repr(C, packed(1))]
pub struct D2DrlgLevel {
        _pad_00_10      : [u8; 0x10],               // 0x00
    pub pFirstRoomEx    : *mut D2DrlgRoom,          // 0x04
}

#[repr(C, packed(4))]
pub struct D2DrlgRoom {
        _pad_00_24      : [u8; 0x24],               // 0x00
    pub pDrlgRoomNext   : *mut D2DrlgRoom,          // 0x24
        _pad_28_30      : [u8; 0x08],               // 0x28
    pub pRoom           : *mut D2ActiveRoom,        // 0x30
    pub nTileXPos       : i32,                      // 0x34
    pub nTileYPos       : i32,                      // 0x38
        _pad_34_48      : [u8; 0x0C],               // 0x3C
    pub nType           : D2DrlgTypes,              // 0x48
    pub pRoomTiles      : *mut D2RoomTile,          // 0x4C
}

#[repr(C, packed(1))]
pub struct D2RoomTile {
    pub pDrlgRoom           : *mut D2DrlgRoom,          // 0x00
    pub pNext               : *mut D2RoomTile,          // 0x04
    pub bEnabled            : i32,                      // 0x08
    pub dword_0C            : u32,                      // 0x0C
    pub pLvlWarpTxtRecord   : *mut D2LvlWarpTxt,        // 0x10
    pub dword_14            : u32,                      // 0x14
}

#[repr(C, packed(1))]
pub struct D2ActiveRoom {
        _pad_00_74          : [u8; 0x74],               // 0x00
    pub pUnitFirst          : *mut D2Unit,              // 0x74
}

#[repr(C, packed(1))]
pub struct D2PresetUnit {
    pub bSpawned            : i32,                      // 0x00
    pub nIndex              : i32,                      // 0x04
    pub nXpos               : i32,                      // 0x08
    pub pNext               : *mut D2PresetUnit,        // 0x0C
    pub pMapAI              : usize,                    // 0x10
    pub nUnitType           : D2UnitTypes,              // 0x14
    pub nYpos               : i32,                      // 0x18
    pub nMode               : i32,                      // 0x1C
}
