use super::common::*;
use super::D2Gfx::*;

pub struct D2CMPOffset {
    pub CelFileNormalize    : FuncAddress,
    pub CelFileFreeHardware : FuncAddress,
}

pub static AddressTable: OnceHolder<D2CMPOffset> = OnceHolder::new();

fn _CelFileNormalize(_cell: &D2CellFileHeader, _normalizedCell: *mut *mut D2CellFileHeader, _file: usize, _line: usize, _specVersion: i32, _unused: usize) {}

pub fn CelFileNormalize(cell: &D2CellFileHeader, specVersion: i32) {
    let mut p: *mut D2CellFileHeader = null_mut();
    addr_to_stdcall(_CelFileNormalize, AddressTable.CelFileNormalize)(cell, addr_of_mut!(p), 0, 0, specVersion, 0)
}

pub fn CelFileFreeHardware(cell: &D2CellFileHeader) -> BOOL {
    addr_to_stdcall(CelFileFreeHardware, AddressTable.CelFileFreeHardware)(cell)
}

pub fn init(d2cmp: usize) {
    AddressTable.initialize(D2CMPOffset{
        CelFileNormalize    : d2cmp + D2RVA::D2CMP(0x6FE21AC0),
        CelFileFreeHardware : d2cmp + D2RVA::D2CMP(0x6FE21520),
    });
}
