use super::common::*;

pub struct ControlOffset {
    pub CreateControl           : FuncAddress,
}

pub struct EditBoxOffset {
    pub SetTextW                : FuncAddress,
    pub SelectAll               : FuncAddress,
}

pub struct MsgHandlerOffset {
    pub RegisterMsgHandler      : FuncAddress,
}

pub struct TextOffset {
    pub SetFont                 : FuncAddress,
    pub GetTextDimensions       : FuncAddress,
    pub DrawText                : FuncAddress,
    pub DrawFramedText          : FuncAddress,
    pub DrawBoxText             : FuncAddress,
    pub MixRGB                  : FuncAddress,
}

pub struct D2WinOffset {
    pub Control                 : ControlOffset,
    pub EditBox                 : EditBoxOffset,
    pub MsgHandler              : MsgHandlerOffset,
    pub Text                    : TextOffset,
}

pub static AddressTable: OnceHolder<D2WinOffset> = OnceHolder::new();

pub mod Control {
    use super::super::common::*;
    use super::AddressTable;

    pub type PerformFnType = extern "stdcall" fn(ctrl: PVOID) -> BOOL;

    #[repr(C)]
    pub struct D2WinControlInitStrc {
        pub ctrl_type   : D2ControlTypes,
        pub x           : i32,
        pub y           : i32,
        pub width       : i32,
        pub height      : i32,
        pub field_14    : i32,
        pub string_id   : i32,
        pub field_1C    : PVOID,
        pub perform     : PerformFnType,
    }

    pub fn CreateControl(init_info: &D2WinControlInitStrc) -> PVOID {
        addr_to_stdcall(CreateControl, AddressTable.Control.CreateControl)(init_info)
    }
}

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

pub mod Text {
    use super::super::common::*;
    use super::AddressTable;
    use super::super::super::d2consts::*;

    pub fn SetFont(font: D2Font) -> D2Font {
        addr_to_fastcall(SetFont, AddressTable.Text.SetFont)(font)
    }

    pub fn _GetTextDimensions(_text: PCWSTR, _width: *mut i32, _height: *mut i32) {}

    pub fn GetTextDimensions(text: PCWSTR) -> (i32, i32) {
        let mut width: i32 = 0;
        let mut height: i32 = 0;

        addr_to_fastcall(_GetTextDimensions, AddressTable.Text.GetTextDimensions)(text, &mut width, &mut height);

        (width, height)
    }

    pub fn DrawText(text: PCWSTR, x: i32, y: i32, color: D2StringColorCodes, center: BOOL) {
        addr_to_fastcall(DrawText, AddressTable.Text.DrawText)(text, x, y, color, center)
    }

    pub fn DrawFramedText(text: PCWSTR, x: i32, y: i32, color: i32, align: i32) {
        addr_to_fastcall(DrawFramedText, AddressTable.Text.DrawFramedText)(text, x, y, color, align)
    }

    pub fn DrawBoxText(text: PCWSTR, x: i32, y: i32, boxColor: u32, drawMode: D2DrawMode, textColor: u32) {
        addr_to_fastcall(DrawBoxText, AddressTable.Text.DrawBoxText)(text, x, y, boxColor, drawMode, textColor)
    }

    pub fn MixRGB(r: u8, g: u8, b: u8) -> u8 {
        addr_to_fastcall(MixRGB, AddressTable.Text.MixRGB)(r, b, g)
    }
}

pub fn init(d2win: usize) {
    AddressTable.initialize(D2WinOffset{
        Control: ControlOffset{
            CreateControl       : d2win + D2RVA::D2Win(0x6F8F8560),
        },
        EditBox: EditBoxOffset{
            SetTextW            : d2win + D2RVA::D2Win(0x6F8F4DF0),
            SelectAll           : d2win + D2RVA::D2Win(0x6F8F3720),
        },
        MsgHandler: MsgHandlerOffset{
            RegisterMsgHandler  : d2win + D2RVA::D2Win(0x6F8F1240),
        },
        Text: TextOffset{
            SetFont             : d2win + D2RVA::D2Win(0x6F8F2FE0),
            GetTextDimensions   : d2win + D2RVA::D2Win(0x6F8F2700),
            DrawText            : d2win + D2RVA::D2Win(0x6F8F2FA0),
            DrawFramedText      : d2win + D2RVA::D2Win(0x6F8F18F0),
            DrawBoxText         : d2win + D2RVA::D2Win(0x6F8F2E90),
            MixRGB              : d2win + D2RVA::D2Win(0x6F8EED70),
        },
    });
}
