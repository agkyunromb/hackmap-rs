mod cell;
use super::common::*;
pub use cell::*;

pub struct WindowOffset {
    pub GetWindow: FuncAddress,
    pub GetState: FuncAddress,
}

pub struct TextureOffset {
    pub CelDrawClipped: FuncAddress,
    pub CelDraw: FuncAddress,
}

pub struct DrawOffset {
    pub DrawLine: FuncAddress,
}

pub struct D2GfxOffset {
    pub Window: WindowOffset,
    pub Texture: TextureOffset,
    pub Draw: DrawOffset,
}

pub static AddressTable: OnceHolder<D2GfxOffset> = OnceHolder::new();

pub mod Window {
    use super::super::common::*;
    use super::AddressTable;

    pub fn GetWindow() -> HWND {
        addr_to_stdcall(GetWindow, AddressTable.Window.GetWindow)()
    }

    pub fn GetState() -> i32 {
        addr_to_stdcall(GetState, AddressTable.Window.GetState)()
    }
}

pub mod Texture {
    use super::super::common::*;
    use super::cell::*;
    use super::AddressTable;

    pub fn CelDrawClipped(data: &D2GfxData, x: i32, y: i32, cropRect: PVOID, drawMode: D2DrawMode) {
        addr_to_stdcall(CelDrawClipped, AddressTable.Texture.CelDrawClipped)(
            data, x, y, cropRect, drawMode,
        )
    }

    pub fn CelDraw(
        data: &D2GfxData,
        x: i32,
        y: i32,
        gamma: u32,
        drawMode: D2DrawMode,
        palette: *const u8,
    ) {
        addr_to_stdcall(CelDraw, AddressTable.Texture.CelDraw)(data, x, y, gamma, drawMode, palette)
    }
}

pub mod Draw {
    use super::super::common::*;
    use super::AddressTable;

    pub fn DrawLine(x1: i32, y1: i32, x2: i32, y2: i32, color: u8, alpha: u8) {
        addr_to_stdcall(DrawLine, AddressTable.Draw.DrawLine)(x1, y1, x2, y2, color, alpha)
    }
}

pub fn init(d2gfx: usize) {
    AddressTable.initialize(D2GfxOffset {
        Window: WindowOffset {
            GetWindow: d2gfx + D2RVA::D2Gfx(0x6FA87FB0),
            GetState: d2gfx + D2RVA::D2Gfx(0x6FA888B0),
        },
        Texture: TextureOffset {
            CelDrawClipped: d2gfx + D2RVA::D2Gfx(0x6FA8AFF0),
            CelDraw: d2gfx + D2RVA::D2Gfx(0x6FA8B080),
        },
        Draw: DrawOffset {
            DrawLine: d2gfx + D2RVA::D2Gfx(0x6FA8B9C0),
        },
    });
}
