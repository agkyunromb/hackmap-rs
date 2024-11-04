use super::common::*;

pub struct NetOffset {
    pub SendPacket          : FuncAddress,
    pub Call_GSCmdHandler   : FuncAddress,
    pub gD2GSHandlers       : FuncAddress,
}

pub struct UIOffset {
    pub SetUIVar                : FuncAddress,
    pub HandleUIVars            : FuncAddress,
    pub DisplayGlobalMessage    : FuncAddress,
    pub DisplayQuickMessage     : FuncAddress,

    pub CallHandleUIVars        : FuncAddress,

    pub gUIVars                 : FuncAddress,
    pub gUIOpenMode             : FuncAddress,
    pub gAttackWithLeftButton   : FuncAddress,
    pub gAttackWithRightButton  : FuncAddress,
}

pub struct GameOffset {
    pub RunGameLoop             : FuncAddress,
    pub SaveAndExitGame         : FuncAddress,
    pub Info                    : FuncAddress,
    pub Call_D2SoundCleanup     : FuncAddress,
    pub Call_D2GFX_GetWindow    : FuncAddress,
    pub IsLodGame               : FuncAddress,
    pub gIsGameExited               : FuncAddress,
}

pub struct AutoMapOffset {
    pub NewAutoMapCell          : FuncAddress,
    pub AddAutoMapCell          : FuncAddress,
    pub DrawAutoMapCells        : FuncAddress,

    pub CallDrawAutoMapCell     : FuncAddress,

    pub gAutoMapCellBlockHead   : FuncAddress,
    pub gAutoMapCellCount       : FuncAddress,
    pub gCurrentAutoMapLayer    : FuncAddress,
    pub gPointDivisor           : FuncAddress,
    pub gPointOffsetX           : FuncAddress,
    pub gPointOffsetY           : FuncAddress,
    pub gRect                   : FuncAddress,
    pub gIsMiniMapOn            : FuncAddress,
}

pub struct UnitsOffset {
    pub GetMonsterOwnerID       : FuncAddress,
    pub GetName                 : FuncAddress,
    pub ShouldShowUnit          : FuncAddress,
    pub GetClientUnit           : FuncAddress,
    pub gClientPlayer           : FuncAddress,
    pub gClientUnitTypeTable    : FuncAddress,
    pub gHasNpcSelected         : FuncAddress,
}

pub struct D2ClientOffset {
    pub UI      : UIOffset,
    pub Net     : NetOffset,
    pub Game    : GameOffset,
    pub AutoMap : AutoMapOffset,
    pub Units   : UnitsOffset,
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

    pub fn SendPacket(payload: *const u8, size: usize) -> usize {
        let seqId: usize;

        unsafe {
            asm!(
                "push {0}",
                "call {1}",
                in(reg) payload,
                in(reg) AddressTable.Net.SendPacket,
                in("ebx") size,
                lateout("eax") seqId,
                clobber_abi("C"),
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

    pub fn GetUIOpenMode() -> u32 {
        read_at(AddressTable.UI.gUIOpenMode)
    }

    pub fn SetAttackWithLeftButton(v: u32) {
        write_at(AddressTable.UI.gAttackWithLeftButton, v);
    }

    pub fn SetAttackWithRightButton(v: u32) {
        write_at(AddressTable.UI.gAttackWithRightButton, v);
    }

    pub fn HandleUIVars(this: PVOID) {
        addr_to_stdcall(HandleUIVars, AddressTable.UI.HandleUIVars)(this)
    }

    pub fn _DisplayGlobalMessage(_text: PCWSTR, _color: D2StringColorCodes) {}

    pub fn DisplayGlobalMessage(text: &str, color: D2StringColorCodes) {
        addr_to_stdcall(_DisplayGlobalMessage, AddressTable.UI.DisplayGlobalMessage)(text.to_utf16().as_ptr(), color)
    }

    pub fn DisplayQuickMessage(text: &str, color: D2StringColorCodes) {
        addr_to_stdcall(_DisplayGlobalMessage, AddressTable.UI.DisplayQuickMessage)(text.to_utf16().as_ptr(), color)
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

    pub fn IsLodGame() -> BOOL {
        addr_to_stdcall(IsLodGame, AddressTable.Game.IsLodGame)()
    }

    pub fn IsGameExited() -> BOOL {
        read_at(AddressTable.Game.gIsGameExited)
    }
}

pub mod AutoMap {
    use super::super::common::*;
    use super::AddressTable;

    #[repr(C, packed(1))]
    pub struct D2AutoMapCellData {
        pub fSaved     : u32,                       // 0x00
        pub nCellNo    : u16,                       // 0x04
        pub xPixel     : u16,                       // 0x06
        pub yPixel     : u16,                       // 0x08
        pub wWeight    : u16,                       // 0x0a
        pub pPrev      : *mut D2AutoMapCellData,    // 0x0c
        pub pNext      : *mut D2AutoMapCellData,    // 0x10
    }

    #[repr(C, packed(1))]
    pub struct D2AutoMapCellBlock {
        pub Elements    : [D2AutoMapCellData; 0x200],
        pub NextBlock   : *mut D2AutoMapCellBlock,
    }

    #[repr(C, packed(4))]
    pub struct D2AutoMapLayer {
        pub nLayerNo    : u32,                      // 0x00
        pub fSaved      : u32,                      // 0x04
        pub pFloors     : *mut D2AutoMapCellData,   // 0x08
        pub pWalls      : *mut D2AutoMapCellData,   // 0x0c
        pub pObjects    : *mut D2AutoMapCellData,   // 0x10
        pub pExtras     : *mut D2AutoMapCellData,   // 0x14
        pub pNext       : *mut D2AutoMapCellData,   // 0x18
    }

    pub fn NewAutoMapCell() -> &'static mut D2AutoMapCellData {
        addr_to_stdcall(NewAutoMapCell, AddressTable.AutoMap.NewAutoMapCell)()
    }

    pub fn AddAutoMapCell(cell: &D2AutoMapCellData, objectList: *mut *mut D2AutoMapCellData) {
        addr_to_fastcall(AddAutoMapCell, AddressTable.AutoMap.AddAutoMapCell)(cell, objectList)
    }

    pub fn DrawAutoMapCells() {
        addr_to_stdcall(DrawAutoMapCells, AddressTable.AutoMap.DrawAutoMapCells)()
    }

    pub fn AutoMapCellBlockHead() -> *mut *mut D2AutoMapCellBlock {
        unsafe { &mut *(AddressTable.AutoMap.gAutoMapCellBlockHead as *mut *mut D2AutoMapCellBlock) }
    }

    pub fn AutoMapCellCount() -> &'static mut usize {
        unsafe { &mut *(AddressTable.AutoMap.gAutoMapCellCount as *mut usize) }
    }

    pub fn CurrentAutoMapLayer() -> Option<&'static mut D2AutoMapLayer> {
        let layer: *mut D2AutoMapLayer = read_at(AddressTable.AutoMap.gCurrentAutoMapLayer);
        ptr_to_ref_mut(layer)
    }

    pub fn PointDivisor() -> &'static mut i32 {
        unsafe { &mut *(AddressTable.AutoMap.gPointDivisor as *mut i32) }
    }

    pub fn PointOffsetX() -> &'static mut i32 {
        unsafe { &mut *(AddressTable.AutoMap.gPointOffsetX as *mut i32) }

    }

    pub fn PointOffsetY() -> &'static mut i32 {
        unsafe { &mut *(AddressTable.AutoMap.gPointOffsetY as *mut i32) }

    }

    pub fn Rect() -> &'static mut RECT {
        unsafe { &mut *(AddressTable.AutoMap.gRect as *mut RECT) }
    }

    pub fn IsMiniMapOn() -> BOOL {
        read_at(AddressTable.AutoMap.gIsMiniMapOn)
    }

}

pub mod Units {
    use std::ptr::addr_of_mut;

    use super::super::common::*;
    use super::AddressTable;
    use super::super::D2Common::D2Unit;

    pub fn GetMonsterOwnerID(unit: &D2Unit) -> u32 {
        addr_to_fastcall(GetMonsterOwnerID, AddressTable.Units.GetMonsterOwnerID)(unit)
    }

    pub fn GetName(unit: *const D2Unit) -> PCWSTR {
        let name: PCWSTR;

        unsafe {
            asm!(
                "call {0}",
                in(reg) AddressTable.Units.GetName,
                in("eax") unit,
                lateout("eax") name,
            );
        }

        name
    }

    pub fn _GetClientUnit(_unitId: u32, _unitType: D2UnitTypes) -> *mut D2Unit { null_mut() }

    pub fn GetClientUnit(unitId: u32, unitType: D2UnitTypes) -> Option<&'static mut D2Unit> {
        ptr_to_ref_mut(addr_to_fastcall(_GetClientUnit, AddressTable.Units.GetClientUnit)(unitId, unitType))
    }

    pub fn GetClientPlayer() -> Option<&'static mut D2Unit> {
        let clinet_player: *mut D2Unit = read_at(AddressTable.Units.gClientPlayer);
        ptr_to_ref_mut(clinet_player)
    }

    pub fn GetClientUnitTypeTable() -> PVOID {
        read_at(AddressTable.Units.gClientUnitTypeTable + 4)
    }

    pub fn GetHasNpcSelected() -> BOOL {
        read_at(AddressTable.Units.gHasNpcSelected)
    }

    pub fn IsCorpse(unit: &D2Unit) -> bool {
        let flags = unit.dwFlags;

        if flags.contains(D2UnitFlags::IsDead) {
            return true;
        }

        match unit.dwUnitType {
            D2UnitTypes::Player => {
                let anim_mode = unsafe { unit.Mode.dwAnimMode };
                if anim_mode == D2PlayerModes::Death as u32 || anim_mode == D2PlayerModes::Dead as u32 {
                    return true;
                }
            },

            D2UnitTypes::Monster => {
                let anim_mode = unsafe { unit.Mode.dwAnimMode };
                if anim_mode == D2MonModes::Death as u32 || anim_mode == D2MonModes::Dead as u32 {
                    return true;
                }
            }

            _ => {},
        }

        false
    }
}

pub fn init(d2client: usize) {
    AddressTable.initialize(D2ClientOffset{
        UI: UIOffset {
            SetUIVar                : d2client + D2RVA::D2Client(0x6FB72790),
            HandleUIVars            : d2client + D2RVA::D2Client(0x6FB739E0),
            DisplayGlobalMessage    : d2client + D2RVA::D2Client(0x6FB2D850),
            DisplayQuickMessage     : d2client + D2RVA::D2Client(0x6FB2D610),

            CallHandleUIVars        : d2client + D2RVA::D2Client(0x6FAF437B),

            gUIVars                 : d2client + D2RVA::D2Client(0x6FBAAD80),
            gUIOpenMode             : d2client + D2RVA::D2Client(0x6FBCC414),
            gAttackWithLeftButton   : d2client + D2RVA::D2Client(0x6FBCC3DC),
            gAttackWithRightButton  : d2client + D2RVA::D2Client(0x6FBCC3E0),
        },
        Net: NetOffset{
            SendPacket              : d2client + D2RVA::D2Client(0x6FAC43E0),
            Call_GSCmdHandler       : d2client + D2RVA::D2Client(0x6FB5CFAF),
            gD2GSHandlers           : d2client + D2RVA::D2Client(0x6FB8DE60),
        },
        Game: GameOffset{
            RunGameLoop             : d2client + D2RVA::D2Client(0x6FAF4F40),
            SaveAndExitGame         : d2client + D2RVA::D2Client(0x6FB15E00),
            Info                    : d2client + D2RVA::D2Client(0x6FBCB980),
            Call_D2SoundCleanup     : d2client + D2RVA::D2Client(0x6FAF515D),
            Call_D2GFX_GetWindow    : d2client + D2RVA::D2Client(0x6FAF423C),
            IsLodGame               : d2client + D2RVA::D2Client(0x6FAF1940),
            gIsGameExited           : d2client + D2RVA::D2Client(0x6FBCC404),
        },
        AutoMap: AutoMapOffset{
            NewAutoMapCell          : d2client + D2RVA::D2Client(0x6FB0F6B0),
            AddAutoMapCell          : d2client + D2RVA::D2Client(0x6FB11320),
            DrawAutoMapCells        : d2client + D2RVA::D2Client(0x6FB10C40),

            CallDrawAutoMapCell     : d2client + D2RVA::D2Client(0x6FB104EA),

            gAutoMapCellBlockHead   : d2client + D2RVA::D2Client(0x6FBCC1B8),
            gAutoMapCellCount       : d2client + D2RVA::D2Client(0x6FBCC1BC),
            gCurrentAutoMapLayer    : d2client + D2RVA::D2Client(0x6FBCC1C4),

            gPointDivisor           : d2client + D2RVA::D2Client(0x6FBA16B0),
            gPointOffsetX           : d2client + D2RVA::D2Client(0x6FBCC1F8),
            gPointOffsetY           : d2client + D2RVA::D2Client(0x6FBCC1FC),
            gRect                   : d2client + D2RVA::D2Client(0x6FBCC228),
            gIsMiniMapOn            : d2client + D2RVA::D2Client(0x6FBCC1B0),
        },
        Units: UnitsOffset{
            GetMonsterOwnerID       : d2client + D2RVA::D2Client(0x6FAD16A0),
            GetName                 : d2client + D2RVA::D2Client(0x6FB55D90),
            ShouldShowUnit          : d2client + D2RVA::D2Client(0x6FB16620),
            GetClientUnit           : d2client + D2RVA::D2Client(0x6FB55B40),
            gClientPlayer           : d2client + D2RVA::D2Client(0x6FBCBBFC),
            gClientUnitTypeTable    : d2client + D2RVA::D2Client(0x6FBBA608),
            gHasNpcSelected         : d2client + D2RVA::D2Client(0x6FBC9721),
        },
    });
}
