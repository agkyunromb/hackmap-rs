use super::types;
pub use super::D2RVA;

pub struct D2MultiOffset {
    pub EnterBNLobby: types::FuncAddress,
}

pub static AddressTable: types::Holder<D2MultiOffset> = types::Holder::new();

pub mod BNet {
    use windows_sys::Win32::Foundation::BOOL;
    use super::types::*;
    use super::D2RVA;
    use super::AddressTable;

    pub fn EnterBNLobby() -> BOOL {
        addr_to_stdcall(EnterBNLobby, AddressTable.EnterBNLobby)()
    }
}

pub fn init(d2multi: usize) {
    AddressTable.initialize(D2MultiOffset{
        EnterBNLobby    : d2multi + D2RVA::D2Multi(0x6F9DB670),
    });
}
