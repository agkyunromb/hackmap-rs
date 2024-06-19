use super::types::*;

pub struct NetOffset {
    pub SendPacket: FuncAddress,
}

pub static AddressTable: Holder<NetOffset> = Holder::new();

pub fn SendPacket(payload: PVOID, size: usize) -> usize {
    addr_to_fn(SendPacket, AddressTable.SendPacket)(payload, size)
}

pub(super) fn init(d2client: usize) {
    AddressTable.initialize(NetOffset {
        SendPacket: d2client + 0x12345,
    });
}
