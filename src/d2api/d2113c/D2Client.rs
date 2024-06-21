use super::common::*;

pub struct NetOffset {
    pub SendPacket    : FuncAddress,

    pub gD2GSHandlers : FuncAddress,
}

pub struct UIOffset {
    pub SetUIVar        : FuncAddress,
    pub HandleUIVars    : FuncAddress,

    pub gUIVars         : FuncAddress,
}

pub struct GameOffset {
    pub SaveAndExitGame : FuncAddress,
    pub Info            : FuncAddress,
}

pub struct D2ClientOffset {
    pub UI    : UIOffset,
    pub Net   : NetOffset,
    pub Game  : GameOffset,
}


pub static AddressTable: OnceHolder<D2ClientOffset> = OnceHolder::new();

pub mod Net {
    use super::super::common::*;
    use super::AddressTable;

    pub type D2GSHandler = extern "fastcall" fn(payload: *const u8);
    pub type UnitProcessor = extern "fastcall" fn(unit: PVOID, payload: *const u8);

    pub const D2GS_MAX_CMD: usize = 175;

    #[repr(C)]
    pub struct D2GSMsgStruct {
        handler         : D2GSHandler,
        cmdSize         : u32,
        process_unit    : UnitProcessor,
    }

    pub fn GetD2GSHandlers() -> &'static mut [D2GSMsgStruct] {
        unsafe {
            std::slice::from_raw_parts_mut(AddressTable.Net.gD2GSHandlers as *mut D2GSMsgStruct, D2GS_MAX_CMD)
        }
    }

    pub fn GetD2GSHandler(cmd: u32) -> &'static mut D2GSMsgStruct {
        &mut GetD2GSHandlers()[cmd as usize]
    }

    pub fn SwapD2GSHandler(cmd: u32, new_handler: D2GSHandler) -> D2GSHandler {
        let handler_table = GetD2GSHandlers();
        let old_handler = handler_table[cmd as usize].handler;

        handler_table[cmd as usize].handler = new_handler;

        old_handler
    }

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

    pub fn SetUIVar(index: D2UIvars, state: i32, arg3: i32) {
        addr_to_fastcall(SetUIVar, AddressTable.UI.SetUIVar)(index, state, arg3)
    }

    pub fn GetUIVar(var: D2UIvars) -> i32 {
        read_at(AddressTable.UI.gUIVars + var as usize * 4)
    }

    pub fn HandleUIVars(this: PVOID) {
        addr_to_stdcall(HandleUIVars, AddressTable.UI.HandleUIVars)(this)
    }
}

pub mod Game {
    use super::super::common::*;
    use super::AddressTable;

    pub struct GameInfo(usize);

    impl GameInfo {
        pub fn get_name(&self) -> String {
            ((self.0 + 0x1B) as PCSTR).to_str().to_string()
        }

        pub fn get_password(&self) -> String {
            ((self.0 + 0x241) as PCSTR).to_str().to_string()
        }
    }

    pub fn SaveAndExitGame(_: i32, hwnd: &HWND) {
        addr_to_fastcall(SaveAndExitGame, AddressTable.Game.SaveAndExitGame)(0, hwnd)
    }

    pub fn GetGameInfo() -> GameInfo {
        GameInfo(read_at(AddressTable.Game.Info))
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
            SendPacket      : d2client + D2RVA::D2Client(0x6FAC43E0),

            gD2GSHandlers   : d2client + D2RVA::D2Client(0x6FB8DE60),
        },
        Game: GameOffset{
            Info            : d2client + D2RVA::D2Client(0x6FBCB980),
            SaveAndExitGame : d2client + D2RVA::D2Client(0x6FB15E00),
        }
    });
}
