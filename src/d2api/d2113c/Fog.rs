use super::common::*;

pub struct FogOffset {
    pub Trace: FuncAddress,
}

pub static AddressTable: OnceHolder<FogOffset> = OnceHolder::new();

fn _Trace(_: *const i8) {}

pub fn Trace(log: &str) {
    let s = std::ffi::CString::new(log).unwrap();
    addr_to_cdecl(_Trace, AddressTable.Trace)(s.as_ptr())
}

pub fn init(fog: usize) {
    AddressTable.initialize(FogOffset{
        Trace: fog + D2RVA::Fog(0x6FF69100),
    });
}
