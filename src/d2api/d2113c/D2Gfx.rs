use super::common::*;

pub struct WindowOffset {
    pub GetWindow: FuncAddress,
}

pub struct TextureOffset {
    pub CelDrawClipped: FuncAddress,
}

pub struct D2GfxOffset {
    pub Window  : WindowOffset,
    pub Texture : TextureOffset,
}

pub static AddressTable: OnceHolder<D2GfxOffset> = OnceHolder::new();

pub mod Window {
    use super::super::common::*;
    use super::AddressTable;

    pub fn GetWindow() -> HWND {
        addr_to_stdcall(GetWindow, AddressTable.Window.GetWindow)()
    }
}

pub mod Texture {
    use super::super::common::*;
    use super::AddressTable;

    #[repr(C, packed(1))]
    pub struct D2GfxDataStrc {
        pCurrentCell    : PVOID,
        pCellFile       : PVOID,
        nFrame          : u32,
        nDirection      : u32,
        nMaxDirections  : i32,
        nMaxFrames      : i32,
        fFlags          : u32,
        fState          : u8,

        // ...
    }

    pub fn CelDrawClipped(pData: &D2GfxDataStrc, nXPos: i32, nYPos: i32, pCropRect: PVOID, eDrawMode: D2DrawMode) {
        addr_to_stdcall(CelDrawClipped, AddressTable.Texture.CelDrawClipped)(pData, nXPos, nYPos, pCropRect, eDrawMode)
    }
}

pub fn init(d2gfx: usize) {
    AddressTable.initialize(D2GfxOffset{
        Window: WindowOffset{
            GetWindow: d2gfx + D2RVA::D2Gfx(0x6FA87FB0),
        },
        Texture: TextureOffset{
            CelDrawClipped: d2gfx + D2RVA::D2Gfx(0x6FA87FB0),
        },
    });
}
