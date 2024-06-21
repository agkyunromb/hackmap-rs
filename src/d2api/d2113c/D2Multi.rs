use super::types;
pub use super::D2RVA;

pub struct BNetOffset {
    pub EnterBNLobby: types::FuncAddress,
}

pub struct D2MultiOffset {
    pub BNet: BNetOffset,
}

pub static AddressTable: types::OnceHolder<D2MultiOffset> = types::OnceHolder::new();

pub mod BNet {
    use windows_sys::Win32::Foundation::BOOL;
    use super::types::*;
    use super::D2RVA;
    use super::AddressTable;

    pub fn EnterBNLobby() -> BOOL {
        addr_to_stdcall(EnterBNLobby, AddressTable.BNet.EnterBNLobby)()
    }
}

pub fn init(d2multi: usize) {
    AddressTable.initialize(D2MultiOffset{
        BNet: BNetOffset{
            EnterBNLobby    : d2multi + D2RVA::D2Multi(0x6F9DB670),
        },
    });
}
