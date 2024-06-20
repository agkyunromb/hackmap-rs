mod uivars;

pub use uivars::*;
use super::common::*;

pub struct NetOffset {
    pub SendPacket    : FuncAddress,
}

pub struct UIOffset {
    pub SetUIVar        : FuncAddress,
    pub HandleUIVars    : FuncAddress,

    pub gUIVars         : FuncAddress,
}

pub struct D2ClientOffset {
    pub UI    : UIOffset,
    pub Net   : NetOffset,
}


pub static AddressTable: OnceHolder<D2ClientOffset> = OnceHolder::new();

pub mod Net {
    use super::super::common::*;
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
    use super::super::common::*;
    use super::AddressTable;

    pub fn SetUIVar(index: super::D2UIvars, state: i32, arg3: i32) {
        addr_to_fastcall(SetUIVar, AddressTable.UI.SetUIVar)(index, state, arg3)
    }

    pub fn GetUIVar(var: super::D2UIvars) -> i32 {
        read_at(AddressTable.UI.gUIVars + var as usize * 4)
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

            gUIVars         : d2client + D2RVA::D2Client(0x6FBAAD80),
        },
        Net: NetOffset{
            SendPacket: d2client + D2RVA::D2Client(0x6FAC43E0),
        },
    });
}
