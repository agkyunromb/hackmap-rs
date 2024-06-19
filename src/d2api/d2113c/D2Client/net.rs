use super::types::*;
use std::arch::asm;
use super::D2RVA;

pub struct NetOffset {
    pub SendPacket: FuncAddress,
}

pub static AddressTable: Holder<NetOffset> = Holder::new();

pub fn SendPacket(payload: PVOID, size: usize) -> usize {
    let seqId: usize;

    unsafe {
        asm!(
            "push ebx",
            "mov  ebx, {1}",
            "push {0}",
            "call {2}",
            "pop  ebx",
            in(reg) payload,
            in(reg) size,
            in(reg) AddressTable.SendPacket,
            lateout("eax") seqId,
            options(nostack),
        );
    }

    seqId
}

pub(super) fn init(d2client: usize) {
    AddressTable.initialize(NetOffset {
        SendPacket: d2client + D2RVA::D2Client(0x6FAC43E0),
    });
}
