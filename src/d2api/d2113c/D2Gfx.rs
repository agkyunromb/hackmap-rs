use super::types;

pub use super::D2RVA;

pub struct WindowOffset {
    pub GetWindow: types::FuncAddress,
}

pub struct D2GfxOffset {
    pub Window: WindowOffset,
}

pub static AddressTable: types::Holder<D2GfxOffset> = types::Holder::new();

pub mod Window {
    use super::types::*;
    use super::D2RVA;
    use windows_sys::Win32::Foundation::HWND;
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
