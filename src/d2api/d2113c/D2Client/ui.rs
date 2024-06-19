use super::types::*;
use super::D2RVA;

pub struct UIOffset {
    pub SetUIVar: FuncAddress,
    pub HandleUIVars: FuncAddress,
}

pub static AddressTable: Holder<UIOffset> = Holder::new();

pub fn SetUIVar(index: i32, state: i32, arg3: i32) {
    addr_to_fastcall(SetUIVar, AddressTable.SetUIVar)(index, state, arg3)
}

pub fn HandleUIVars(this: PVOID) {
    addr_to_stdcall(HandleUIVars, AddressTable.HandleUIVars)(this)
}

pub(super) fn init(d2client: usize) {
    AddressTable.initialize(UIOffset {
        SetUIVar        : d2client + D2RVA::D2Client(0x6FB72790),
        HandleUIVars    : d2client + D2RVA::D2Client(0x6FAF437B),
    });
}
