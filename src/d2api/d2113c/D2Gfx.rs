use super::common::*;

pub struct WindowOffset {
    pub GetWindow: FuncAddress,
}

pub struct D2GfxOffset {
    pub Window: WindowOffset,
}

pub static AddressTable: OnceHolder<D2GfxOffset> = OnceHolder::new();

pub mod Window {
    use super::super::common::*;
    use super::AddressTable;

    pub fn GetWindow() -> HWND {
        addr_to_stdcall(GetWindow, AddressTable.Window.GetWindow)()
    }
}

pub fn init(d2gfx: usize) {
    AddressTable.initialize(D2GfxOffset{
        Window: WindowOffset{
            GetWindow: d2gfx + D2RVA::D2Gfx(0x6FA87FB0),
        },
    });
}
