use super::common::*;

pub struct BNetOffset {
    pub EnterBNLobby: FuncAddress,
}

pub struct D2MultiOffset {
    pub BNet: BNetOffset,
}

pub static AddressTable: OnceHolder<D2MultiOffset> = OnceHolder::new();

pub mod BNet {
    use super::super::common::*;
    use super::AddressTable;

    pub fn EnterBNLobby() -> BOOL {
        addr_to_stdcall(EnterBNLobby, AddressTable.BNet.EnterBNLobby)()
    }
}

pub fn init(d2multi: usize) {
    AddressTable.initialize(D2MultiOffset {
        BNet: BNetOffset {
            EnterBNLobby: d2multi + D2RVA::D2Multi(0x6F9DB670),
        },
    });
}
