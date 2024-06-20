use super::types;

pub use super::D2RVA;

pub struct NetOffset {
    pub SendPacket    : types::FuncAddress,
}

pub struct UIOffset {
    pub SetUIVar      : types::FuncAddress,
    pub HandleUIVars  : types::FuncAddress,
}

pub struct D2ClientOffset {
    UI    : UIOffset,
    Net   : NetOffset,
}


pub static AddressTable: types::Holder<D2ClientOffset> = types::Holder::new();

pub mod Net {
    use super::types::*;
    use std::arch::asm;
    use super::D2RVA;
    use super::AddressTable;

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
                in(reg) AddressTable.Net.SendPacket,
                lateout("eax") seqId,
                options(nostack),
            );
        }

        seqId
    }
}

pub mod UI {
    use super::types::*;
    use super::D2RVA;
    use super::AddressTable;

    pub fn SetUIVar(index: i32, state: i32, arg3: i32) {
        addr_to_fastcall(SetUIVar, AddressTable.UI.SetUIVar)(index, state, arg3)
    }

    pub fn HandleUIVars(this: PVOID) {
        addr_to_stdcall(HandleUIVars, AddressTable.UI.HandleUIVars)(this)
    }
}

pub fn init(d2client: usize) {
    AddressTable.initialize(D2ClientOffset{
        UI: UIOffset {
            SetUIVar        : d2client + D2RVA::D2Client(0x6FB72790),
            HandleUIVars    : d2client + D2RVA::D2Client(0x6FAF437B),
        },
        Net: NetOffset{
            SendPacket: d2client + D2RVA::D2Client(0x6FAC43E0),
        },
    });
}
