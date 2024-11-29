use super::common::*;

pub struct StormOffset {
    pub MPQLoadFile: FuncAddress,
}

pub static AddressTable: OnceHolder<StormOffset> = OnceHolder::new();

pub fn MPQLoadFile(
    fileInfo: *const u8,
    buffer: *mut u8,
    bufferSize: usize,
    fileSize: *mut usize,
    eventInfo: *const u8,
    arg6: usize,
    arg7: usize,
) -> BOOL {
    addr_to_stdcall(MPQLoadFile, AddressTable.MPQLoadFile)(
        fileInfo, buffer, bufferSize, fileSize, eventInfo, arg6, arg7,
    )
}

pub fn init(storm: usize) {
    AddressTable.initialize(StormOffset {
        MPQLoadFile: storm + D2RVA::Storm(0x6FC195B0),
    });
}
