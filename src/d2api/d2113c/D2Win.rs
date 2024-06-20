use super::types;
pub use super::D2RVA;

pub struct EditBoxOffset {
    pub SetTextW    : types::FuncAddress,
    pub SelectAll   : types::FuncAddress,
}

pub struct D2WinOffset {
    pub EditBox: EditBoxOffset,
}

pub static AddressTable: types::Holder<D2WinOffset> = types::Holder::new();

pub mod EditBox {
    use windows_sys::core::PCWSTR;
    use super::types::*;
    use super::D2RVA;
    use super::AddressTable;

    pub fn SetTextW(ctrl: PVOID, text: PCWSTR) -> u32 {
        addr_to_fastcall(SetTextW, AddressTable.EditBox.SetTextW)(ctrl, text)
    }

    pub fn SelectAll(ctrl: PVOID) -> u32 {
        addr_to_fastcall(SelectAll, AddressTable.EditBox.SelectAll)(ctrl)
    }
}

pub fn init(d2win: usize) {
    AddressTable.initialize(D2WinOffset{
        EditBox: EditBoxOffset{
            SetTextW    : d2win + D2RVA::D2Gfx(0x6FA87FB0),
            SelectAll   : d2win + D2RVA::D2Gfx(0x6FA87FB0),
        },
    });
}
