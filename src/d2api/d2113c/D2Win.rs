use super::common::*;

pub struct EditBoxOffset {
    pub SetTextW              : FuncAddress,
    pub SelectAll             : FuncAddress,
}

pub struct MsgHandlerOffset {
    pub RegisterMsgHandler    : FuncAddress,
}

pub struct D2WinOffset {
    pub EditBox               : EditBoxOffset,
    pub MsgHandler            : MsgHandlerOffset,
}

pub static AddressTable: OnceHolder<D2WinOffset> = OnceHolder::new();

pub mod EditBox {
    use super::super::common::*;
    use super::AddressTable;

    pub fn SetTextW(ctrl: PVOID, text: PCWSTR) -> u32 {
        addr_to_fastcall(SetTextW, AddressTable.EditBox.SetTextW)(ctrl, text)
    }

    pub fn SelectAll(ctrl: PVOID) -> u32 {
        addr_to_fastcall(SelectAll, AddressTable.EditBox.SelectAll)(ctrl)
    }
}

pub mod MsgHandler {
    use super::super::common::*;
    use super::AddressTable;

    #[repr(C, packed(1))]
    pub struct StormMsgHandlerParams {
        pub hwnd              : HWND,
        pub message           : u32,
        pub wparam            : u32,
        pub lparam            : u32,
        pub command_source    : u32,
        pub arg               : u32,
        pub returned          : BOOL,
        pub result            : i32,
    }

    impl StormMsgHandlerParams {
        pub fn virtual_key(&self) -> u16 {
            self.wparam as u16
        }

        pub fn key_pressed(&self) -> bool {
            (self.lparam & (1 << 30)) != 0
        }

        pub fn x(&self) -> u16 {
            self.lparam as u16
        }

        pub fn y(&self) -> u16 {
            (self.lparam >> 16) as u16
        }
    }

    pub type StormMsgHandler = extern "stdcall" fn(msg: &mut StormMsgHandlerParams);

    pub fn RegisterMsgHandler(hwnd: HWND, msg_type: u32, msg: u32, handler: StormMsgHandler) {
        addr_to_fastcall(RegisterMsgHandler, AddressTable.MsgHandler.RegisterMsgHandler)(hwnd, msg_type, msg, handler)
    }
}

pub fn init(d2win: usize) {
    AddressTable.initialize(D2WinOffset{
        EditBox: EditBoxOffset{
            SetTextW              : d2win + D2RVA::D2Win(0x6FA87FB0),
            SelectAll             : d2win + D2RVA::D2Win(0x6FA87FB0),
        },
        MsgHandler: MsgHandlerOffset{
            RegisterMsgHandler    : d2win + D2RVA::D2Win(0x6F8F1240),
        }
    });
}
