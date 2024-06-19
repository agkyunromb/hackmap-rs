use super::types::*;

pub struct UIOffset {
    pub SetUIVar: FuncAddress,
    pub HandleUIVars: FuncAddress,
}

pub static AddressTable: Holder<UIOffset> = Holder::new();

pub fn SetUIVar(index: i32, state: i32, arg3: i32) {
    addr_to_fn(SetUIVar, AddressTable.SetUIVar)(index, state, arg3)
}

pub fn HandleUIVars(this: PVOID) {
    addr_to_fn(HandleUIVars, AddressTable.HandleUIVars)(this)
}

pub(super) fn init(d2client: usize) {
    AddressTable.initialize(UIOffset {
        SetUIVar: d2client + 0x100000,
        HandleUIVars: d2client + 0x200000,
    });
}
