use super::super::super::d2consts::*;
use std::ptr::{addr_of, null, null_mut};

#[repr(C, packed(1))]
pub struct D2GfxData {
    pub nFrame: u32,    // 0x00
    _04_34: [u8; 0x30], // 0x04
    // pub pName           : *const u8,                // 0x2C
    // pub nMaxDirections  : i32,                      // 0x10
    // pub nMaxFrames      : i32,                      // 0x14
    // pub fFlags          : u32,                      // 0x18
    // pub fState          : u8,                       // 0x1C
    // pub fItemFlags      : u8,                       // 0x1D
    // pub byte_1E         : u8,                       // 0x1E
    // pub byte_1F         : u8,                       // 0x1F
    // pub nUnitType       : i32,                      // 0x20
    // pub nUnitIndex      : i32,                      // 0x24
    // pub nMode           : i32,                      // 0x28
    // pub nOverlay        : i32,                      // 0x2C
    // pub dwName          : [u32; 5],                 // 0x30
    pub pCellFile: *mut D2CellFileHeader,   // 0x34
    _38_3C: [u8; 0x04],                     // 0x38
    pub pCurrentCell: *mut D2CellFileFrame, // 0x3C
    pub nDirection: u32,                    // 0x40
    _44_48: [u8; 0x04],                     // 0x44

                                            /* size: 0x0048 */
}

impl D2GfxData {
    pub fn new() -> Self {
        Self {
            nFrame: 0,
            _04_34: [0; 0x30],
            pCellFile: null_mut(),
            _38_3C: [0; 0x04],
            pCurrentCell: null_mut(),
            nDirection: 0,
            _44_48: [0; 0x04],
        }
    }
}

#[repr(C, packed(1))]
pub struct D2CellFileHeader {
    pub Version: u32,                 // 0x00
    pub Flags: u32,                   // 0x04
    pub Format: u32,                  // 0x08     0: 8bit, 1: 24bit, 2: ??
    pub Termination: u32,             // 0x0C     0xEEEEEEEE FIXME
    pub Directions: u32,              // 0x10
    pub FramesPerDirection: u32,      // 0x14
    pub Frames: *mut D2CellFileFrame, // 0x18
}

impl D2CellFileHeader {
    pub fn get_frame(&self, frame_index: usize) -> &D2CellFileFrame {
        unsafe {
            let frames = std::slice::from_raw_parts(addr_of!(self.Frames), frame_index + 1);
            &mut *frames[frame_index]
        }
    }
}

#[repr(C, packed(1))]
pub struct D2CellFileFrame {
    pub Flags: u32,                      // 0x00
    pub Width: i32,                      // 0x04
    pub Height: i32,                     // 0x08
    pub OffsetX: i32,                    // 0x0C
    pub OffsetY: i32,                    // 0x10
    pub DWORD_14: u32,                   // 0x14
    pub NextFrame: *mut D2CellFileFrame, // 0x18
    pub Size: usize,                     // 0x1C
}
