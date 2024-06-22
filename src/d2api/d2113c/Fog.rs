use super::common::*;

pub struct FogOffset {
    pub Trace: FuncAddress,
    pub Alloc: FuncAddress,
}

pub static AddressTable: OnceHolder<FogOffset> = OnceHolder::new();

fn _Trace(_: *const i8) {}

pub fn Trace(log: &str) {
    let s = std::ffi::CString::new(log).unwrap();
    addr_to_cdecl(_Trace, AddressTable.Trace)(s.as_ptr())
}

pub fn _Alloc(_size: usize, _file: usize, _line: usize, _zero: usize) -> PVOID { std::ptr::null_mut() }

pub fn Alloc<T>(size: usize) -> *mut T {
    addr_to_fastcall(_Alloc, AddressTable.Alloc)(size, 0, 0, 0) as *mut T
}

pub fn init(fog: usize) {
    AddressTable.initialize(FogOffset{
        Trace: fog + D2RVA::Fog(0x6FF69100),
        Alloc: fog + D2RVA::Fog(0x6FF6CD10),
    });
}
