use super::super::super::d2consts::*;
use super::drlg::*;

#[repr(C, packed(1))]
pub union D2Unit_10 {
    dwAnimMode      : u32,
    dwItemMode      : u32,
    dwCollideType   : u32,
}

#[repr(C, packed(1))]
pub union D2Unit_Data_14 {
    dwAnimMode      : u32,
    dwItemMode      : u32,
    dwCollideType   : u32,
}

#[repr(C, packed(1))]
pub struct D2Unit {
    pub dwUnitType  : D2UnitTypes,          // 0x00
    pub dwClassId   : u32,                  // 0x04
    pub pMemoryPool : *const u8,            // 0x08
    pub dwUnitId    : u32,                  // 0x0C
    pub Mode        : D2Unit_10,            // 0x10
    pub Data        : D2Unit_Data_14,       // 0x14
    pub nAct        : u8,                   // 0x18
    pub _pad_19_1C  : [u8; 3],              // 0x19
    pub pDrlgAct    : *mut D2DrlgAct,       // 0x1C
}

impl D2Unit {
    pub fn get_drlg_act(&self) -> &mut D2DrlgAct {
        unsafe {
            &mut *self.pDrlgAct
        }
    }
}
