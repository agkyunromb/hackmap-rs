use super::common::*;
use super::D2Common::{D2ItemsTxt, D2Unit};

#[repr(C, packed(4))]
pub struct GetItemPropertiesContext {
    pub buf1: [u8; 0x100],               // 0x0000
    pub text: [u16; 0x4000],             // 0x0100
    pub text2: [u16; 0x2000],            // 0x8100
    pub text3: [[u16; 0x400]; 3],        // 0xC100
    pub client_unit_type_table: PVOID,   // 0xD900
    pub unit: *mut D2Unit,               // 0xD904
    pub owner: *mut D2Unit,              // 0xD908
    pub unit_item_txt: *mut D2Unit,      // 0xD90C
    pub owner_item_txt: *mut D2ItemsTxt, // 0xD910
    pub stat_list: PVOID,                // 0xD914
    pub set_items_txt: PVOID,            // 0xD918
    pub sets_txt: PVOID,                 // 0xD91C
}

impl GetItemPropertiesContext {
    pub fn new() -> Self {
        Self {
            buf1: [0; 0x100],
            text: [0; 0x4000],
            text2: [0; 0x2000],
            text3: [[0; 0x400]; 3],
            client_unit_type_table: null_mut(),
            unit: null_mut(),
            owner: null_mut(),
            unit_item_txt: null_mut(),
            owner_item_txt: null_mut(),
            stat_list: null_mut(),
            set_items_txt: null_mut(),
            sets_txt: null_mut(),
        }
    }
}

pub struct AutoMapOffset {
    pub RevealMap: FuncAddress,
    pub DrawAutoMap: FuncAddress,
    // pub DrawAutoMapUnits                : FuncAddress,
    pub DrawUnitBlob: FuncAddress,
}

pub struct UnitsOffset {
    pub GetName: FuncAddress,
    pub DisplayItemProperties: FuncAddress,
}

pub struct ItemsOffset {
    pub GetItemName: FuncAddress,
    pub GetItemNameColor: FuncAddress,
    pub LootFilterCheckUnit: FuncAddress,
}

pub struct ItemTextOffset {
    pub GetItemPropertiesInit: FuncAddress,
    pub GetItemProperties1: FuncAddress,
    pub GetItemProperties2: FuncAddress,
    pub GetItemProperties3: FuncAddress,
    pub GetItemProperties4: FuncAddress,
    pub GetItemProperties5: FuncAddress,
    pub GetItemProperties6: FuncAddress,
    pub GetItemProperties7: FuncAddress,
    pub GetItemProperties8: FuncAddress,
    pub GetItemProperties9: FuncAddress,
    pub GetItemProperties10: FuncAddress,
    pub GetItemProperties11: FuncAddress,
    pub GetItemProperties12: FuncAddress,
    pub GetItemProperties13: FuncAddress,
    pub GetItemProperties14: FuncAddress,
    pub GetItemProperties15: FuncAddress,
    pub GetItemProperties16: FuncAddress,
    pub GetItemProperties17: FuncAddress,
    pub GetItemProperties18: FuncAddress,
    pub GetItemProperties19: FuncAddress,
    pub GetItemProperties20: FuncAddress,
    pub GetItemProperties21: FuncAddress,
    pub GetItemProperties22: FuncAddress,
    pub GetItemProperties23: FuncAddress,
    pub GetItemProperties24: FuncAddress,
    pub GetItemProperties25: FuncAddress,
    pub GetItemProperties26: FuncAddress,
    pub GetItemProperties27: FuncAddress,
    pub GetName: FuncAddress,
    pub GetItemProperties29: FuncAddress,
    pub AddCtrlPressedHintText: FuncAddress,
    pub GetCostHintText: FuncAddress,
}

pub struct UIOffset {
    pub DrawItemProperties: FuncAddress,
    pub BossLifeBar_Call_Units_GetName: FuncAddress,
    pub MonsterLifeBar_Call_Units_GetName: FuncAddress,
    pub CheckIsMonsterShouldDisplayLifeBar: FuncAddress,
}

pub struct D2SigmaOffset {
    pub AutoMap: AutoMapOffset,
    pub Units: UnitsOffset,
    pub Items: ItemsOffset,
    pub ItemText: ItemTextOffset,
    pub UI: UIOffset,
}

pub static AddressTable: OnceHolder<D2SigmaOffset> = OnceHolder::new();

pub mod AutoMap {
    use super::*;

    pub fn RevealMap() {
        addr_to_stdcall(RevealMap, AddressTable.AutoMap.RevealMap)()
    }

    pub fn DrawUnitBlob(x: i32, y: i32, arg3: i32, color: u8) {
        addr_to_fastcall(DrawUnitBlob, AddressTable.AutoMap.DrawUnitBlob)(x, y, arg3, color)
    }
}

pub mod Units {
    use super::*;

    pub fn GetName(unit: &D2Unit) -> PCWSTR {
        addr_to_fastcall(GetName, AddressTable.Units.GetName)(unit)
    }

    pub fn DisplayItemProperties(clientUnitTypeTable: &D2Unit, unit: &D2Unit) {
        addr_to_fastcall(
            DisplayItemProperties,
            AddressTable.Units.DisplayItemProperties,
        )(clientUnitTypeTable, unit)
    }
}

pub mod Items {
    use super::*;

    pub fn GetItemName(unit: &D2Unit, buffer: PCWSTR, arg3: u32) -> PCWSTR {
        addr_to_stdcall(GetItemName, AddressTable.Items.GetItemName)(unit, buffer, arg3)
    }

    pub fn GetItemNameColor(unit: &D2Unit) -> D2StringColorCodes {
        addr_to_fastcall(GetItemNameColor, AddressTable.Items.GetItemNameColor)(unit)
    }
}

pub mod ItemText {
    use super::*;

    pub fn _GetItemPropertiesInit(
        _ctx: &mut GetItemPropertiesContext,
        _: usize,
        _player: &D2Unit,
        _item: &D2Unit,
        _owner: *const D2Unit,
    ) {
    }

    pub fn GetItemPropertiesInit(
        ctx: &mut GetItemPropertiesContext,
        player: &D2Unit,
        item: &D2Unit,
        owner: *const D2Unit,
    ) {
        addr_to_fastcall(
            _GetItemPropertiesInit,
            AddressTable.ItemText.GetItemPropertiesInit,
        )(ctx, 0, player, item, owner)
    }

    pub fn GetItemProperties1(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(GetItemProperties1, AddressTable.ItemText.GetItemProperties1)(ctx)
    }

    pub fn GetItemProperties2(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(GetItemProperties2, AddressTable.ItemText.GetItemProperties2)(ctx)
    }

    pub fn GetItemProperties3(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(GetItemProperties3, AddressTable.ItemText.GetItemProperties3)(ctx)
    }

    pub fn GetItemProperties4(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(GetItemProperties4, AddressTable.ItemText.GetItemProperties4)(ctx)
    }

    pub fn GetItemProperties5(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(GetItemProperties5, AddressTable.ItemText.GetItemProperties5)(ctx)
    }

    pub fn GetItemProperties6(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(GetItemProperties6, AddressTable.ItemText.GetItemProperties6)(ctx)
    }

    pub fn GetItemProperties7(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(GetItemProperties7, AddressTable.ItemText.GetItemProperties7)(ctx)
    }

    pub fn GetItemProperties8(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(GetItemProperties8, AddressTable.ItemText.GetItemProperties8)(ctx)
    }

    pub fn GetItemProperties9(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(GetItemProperties9, AddressTable.ItemText.GetItemProperties9)(ctx)
    }

    pub fn GetItemProperties10(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties10,
            AddressTable.ItemText.GetItemProperties10,
        )(ctx)
    }

    pub fn GetItemProperties11(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties11,
            AddressTable.ItemText.GetItemProperties11,
        )(ctx)
    }

    pub fn GetItemProperties12(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties12,
            AddressTable.ItemText.GetItemProperties12,
        )(ctx)
    }

    pub fn GetItemProperties13(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties13,
            AddressTable.ItemText.GetItemProperties13,
        )(ctx)
    }

    pub fn GetItemProperties14(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties14,
            AddressTable.ItemText.GetItemProperties14,
        )(ctx)
    }

    pub fn GetItemProperties15(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties15,
            AddressTable.ItemText.GetItemProperties15,
        )(ctx)
    }

    pub fn GetItemProperties16(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties16,
            AddressTable.ItemText.GetItemProperties16,
        )(ctx)
    }

    pub fn GetItemProperties17(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties17,
            AddressTable.ItemText.GetItemProperties17,
        )(ctx)
    }

    pub fn GetItemProperties18(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties18,
            AddressTable.ItemText.GetItemProperties18,
        )(ctx)
    }

    pub fn GetItemProperties19(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties19,
            AddressTable.ItemText.GetItemProperties19,
        )(ctx)
    }

    pub fn GetItemProperties20(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties20,
            AddressTable.ItemText.GetItemProperties20,
        )(ctx)
    }

    pub fn GetItemProperties21(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties21,
            AddressTable.ItemText.GetItemProperties21,
        )(ctx)
    }

    pub fn GetItemProperties22(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties22,
            AddressTable.ItemText.GetItemProperties22,
        )(ctx)
    }

    pub fn GetItemProperties23(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties23,
            AddressTable.ItemText.GetItemProperties23,
        )(ctx)
    }

    pub fn GetItemProperties24(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties24,
            AddressTable.ItemText.GetItemProperties24,
        )(ctx)
    }

    pub fn GetItemProperties25(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties25,
            AddressTable.ItemText.GetItemProperties25,
        )(ctx)
    }

    pub fn GetItemProperties26(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties26,
            AddressTable.ItemText.GetItemProperties26,
        )(ctx)
    }

    pub fn GetItemProperties27(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties27,
            AddressTable.ItemText.GetItemProperties27,
        )(ctx)
    }

    pub fn GetName(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(GetName, AddressTable.ItemText.GetName)(ctx)
    }

    pub fn GetItemProperties29(ctx: &mut GetItemPropertiesContext) {
        addr_to_fastcall(
            GetItemProperties29,
            AddressTable.ItemText.GetItemProperties29,
        )(ctx)
    }
}

pub fn initialized() -> bool {
    // return false;
    AddressTable.initialized()
}

pub fn init(d2sigma: usize) {
    let timestamp = unsafe {
        (*RtlImageNtHeader(d2sigma as PVOID))
            .FileHeader
            .TimeDateStamp
    };

    const D2Sigma_BaseAddress: usize = 0x10000000;

    let vmslide = d2sigma.wrapping_sub(D2Sigma_BaseAddress);

    match timestamp {
        0x663D01B3 => init_2_9_2(vmslide),
        0x6724FDBD => init_2_10(vmslide),
        0x673ECCE8 => init_2_10_3(vmslide),
        0x672AA2EF => init_2_10_2(vmslide),
        0x67461404 => init_2_10_5(vmslide),

        _ => {}
    }
}

fn init_2_10_5(vmslide: usize) {
    init_2_10_3(vmslide);
}

fn init_2_10_3(vmslide: usize) {
    AddressTable.initialize(D2SigmaOffset {
        AutoMap: AutoMapOffset {
            RevealMap: vmslide + 0x100A8FE0,
            DrawAutoMap: vmslide + 0x100599B0,
            // DrawAutoMapUnits    : vmslide + 0x00000000,
            DrawUnitBlob: vmslide + 0x10084DD0,
        },
        Units: UnitsOffset {
            GetName: vmslide + 0x100D26A0,
            DisplayItemProperties: vmslide + 0x100902D0,
        },
        Items: ItemsOffset {
            GetItemName: vmslide + 0x10090610,
            GetItemNameColor: vmslide + 0x100910E0,
            LootFilterCheckUnit: vmslide + 0x100A6990,
        },
        ItemText: ItemTextOffset {
            GetItemPropertiesInit: vmslide + 0x1008FD20,
            GetItemProperties1: vmslide + 0x1009BA70,
            GetItemProperties2: vmslide + 0x1009A1A0,
            GetItemProperties3: vmslide + 0x10099ED0,
            GetItemProperties4: vmslide + 0x1009A570,
            GetItemProperties5: vmslide + 0x10097C40,
            GetItemProperties6: vmslide + 0x10098640,
            GetItemProperties7: vmslide + 0x10098770,
            GetItemProperties8: vmslide + 0x1009B900,
            GetItemProperties9: vmslide + 0x10097B70,
            GetItemProperties10: vmslide + 0x1009B3E0,
            GetItemProperties11: vmslide + 0x10097830,
            GetItemProperties12: vmslide + 0x10098E10,
            GetItemProperties13: vmslide + 0x1009BB40,
            GetItemProperties14: vmslide + 0x1009A710,
            GetItemProperties15: vmslide + 0x1009C060,
            GetItemProperties16: vmslide + 0x10098340,
            GetItemProperties17: vmslide + 0x10097A30,
            GetItemProperties18: vmslide + 0x10099480,
            GetItemProperties19: vmslide + 0x10099A20,
            GetItemProperties20: vmslide + 0x10099750,
            GetItemProperties21: vmslide + 0x10099180,
            GetItemProperties22: vmslide + 0x10098C80,
            GetItemProperties23: vmslide + 0x10091900,
            GetItemProperties24: vmslide + 0x10091B70,
            GetItemProperties25: vmslide + 0x1009C2C0,
            GetItemProperties26: vmslide + 0x10098AD0,
            GetItemProperties27: vmslide + 0x10099CF0,
            GetName: vmslide + 0x100985C0,
            GetItemProperties29: vmslide + 0x1009BE20,
            AddCtrlPressedHintText: vmslide + 0x10098F80,
            GetCostHintText: vmslide + 0x1009B610,
        },
        UI: UIOffset {
            DrawItemProperties: vmslide + 0x10091AC0, // GetKeyState(VK_CONTROL)
            BossLifeBar_Call_Units_GetName: vmslide + 0x1009FABB, // BossLifebar:BossName
            MonsterLifeBar_Call_Units_GetName: vmslide + 0x1009F0A8, // game\\hud\\mon-hp-bar
            CheckIsMonsterShouldDisplayLifeBar: vmslide + 0x1009EEF9, // game\\hud\\mon-hp-bar, test    eax, 201h
        },
    });
}

fn init_2_10_2(vmslide: usize) {
    AddressTable.initialize(D2SigmaOffset {
        AutoMap: AutoMapOffset {
            RevealMap: vmslide + 0x100A58C0,
            DrawAutoMap: vmslide + 0x100594A0,
            // DrawAutoMapUnits    : vmslide + 0x00000000,
            DrawUnitBlob: vmslide + 0x100816A0,
        },
        Units: UnitsOffset {
            GetName: vmslide + 0x100CF840,
            DisplayItemProperties: vmslide + 0x1008CBA0,
        },
        Items: ItemsOffset {
            GetItemName: vmslide + 0x1008CED0,
            GetItemNameColor: vmslide + 0x1008D9A0,
            LootFilterCheckUnit: vmslide + 0x100A3260,
        },
        ItemText: ItemTextOffset {
            GetItemPropertiesInit: vmslide + 0x1008C5F0,
            GetItemProperties1: vmslide + 0x10098340,
            GetItemProperties2: vmslide + 0x10096A70,
            GetItemProperties3: vmslide + 0x100967A0,
            GetItemProperties4: vmslide + 0x10096E40,
            GetItemProperties5: vmslide + 0x10094510,
            GetItemProperties6: vmslide + 0x10094F10,
            GetItemProperties7: vmslide + 0x10095040,
            GetItemProperties8: vmslide + 0x100981D0,
            GetItemProperties9: vmslide + 0x10094440,
            GetItemProperties10: vmslide + 0x10097CB0,
            GetItemProperties11: vmslide + 0x10094100,
            GetItemProperties12: vmslide + 0x100956E0,
            GetItemProperties13: vmslide + 0x10098410,
            GetItemProperties14: vmslide + 0x10096FE0,
            GetItemProperties15: vmslide + 0x10098930,
            GetItemProperties16: vmslide + 0x10094C10,
            GetItemProperties17: vmslide + 0x10094300,
            GetItemProperties18: vmslide + 0x10095D50,
            GetItemProperties19: vmslide + 0x100962F0,
            GetItemProperties20: vmslide + 0x10096020,
            GetItemProperties21: vmslide + 0x10095A50,
            GetItemProperties22: vmslide + 0x10095550,
            GetItemProperties23: vmslide + 0x1008E1C0,
            GetItemProperties24: vmslide + 0x1008E430,
            GetItemProperties25: vmslide + 0x10098B90,
            GetItemProperties26: vmslide + 0x100953A0,
            GetItemProperties27: vmslide + 0x100965C0,
            GetName: vmslide + 0x10094E90,
            GetItemProperties29: vmslide + 0x100986F0,
            AddCtrlPressedHintText: vmslide + 0x10095850,
            GetCostHintText: vmslide + 0x10097EE0,
        },
        UI: UIOffset {
            DrawItemProperties: vmslide + 0x1008E380, // GetKeyState(VK_CONTROL)
            BossLifeBar_Call_Units_GetName: vmslide + 0x1009C38B, // BossLifebar:BossName
            MonsterLifeBar_Call_Units_GetName: vmslide + 0x1009B978, // game\\hud\\mon-hp-bar
            CheckIsMonsterShouldDisplayLifeBar: vmslide + 0x1009B7C9, // game\\hud\\mon-hp-bar, test    eax, 201h
        },
    });
}

fn init_2_10(vmslide: usize) {
    AddressTable.initialize(D2SigmaOffset {
        AutoMap: AutoMapOffset {
            RevealMap: vmslide + 0x1009FCD0,
            DrawAutoMap: vmslide + 0x100538C0,
            // DrawAutoMapUnits    : vmslide + 0x00000000,
            DrawUnitBlob: vmslide + 0x1007BAC0,
        },
        Units: UnitsOffset {
            GetName: vmslide + 0x100C9BC0,
            DisplayItemProperties: vmslide + 0x10086FC0,
        },
        Items: ItemsOffset {
            GetItemName: vmslide + 0x100872F0,
            GetItemNameColor: vmslide + 0x10087DC0,
            LootFilterCheckUnit: vmslide + 0x1009D680,
        },
        ItemText: ItemTextOffset {
            GetItemPropertiesInit: vmslide + 0x10086A10,
            GetItemProperties1: vmslide + 0x10092750,
            GetItemProperties2: vmslide + 0x10090E80,
            GetItemProperties3: vmslide + 0x10090BB0,
            GetItemProperties4: vmslide + 0x10091250,
            GetItemProperties5: vmslide + 0x1008E920,
            GetItemProperties6: vmslide + 0x1008F320,
            GetItemProperties7: vmslide + 0x1008F450,
            GetItemProperties8: vmslide + 0x100925E0,
            GetItemProperties9: vmslide + 0x1008E850,
            GetItemProperties10: vmslide + 0x100920C0,
            GetItemProperties11: vmslide + 0x1008E510,
            GetItemProperties12: vmslide + 0x1008FAF0,
            GetItemProperties13: vmslide + 0x10092820,
            GetItemProperties14: vmslide + 0x100913F0,
            GetItemProperties15: vmslide + 0x10092D40,
            GetItemProperties16: vmslide + 0x1008F020,
            GetItemProperties17: vmslide + 0x1008E710,
            GetItemProperties18: vmslide + 0x10090160,
            GetItemProperties19: vmslide + 0x10090700,
            GetItemProperties20: vmslide + 0x10090430,
            GetItemProperties21: vmslide + 0x1008FE60,
            GetItemProperties22: vmslide + 0x1008F960,
            GetItemProperties23: vmslide + 0x100885E0,
            GetItemProperties24: vmslide + 0x10088850,
            GetItemProperties25: vmslide + 0x10092FA0,
            GetItemProperties26: vmslide + 0x1008F7B0,
            GetItemProperties27: vmslide + 0x100909D0,
            GetName: vmslide + 0x1008F2A0,
            GetItemProperties29: vmslide + 0x10092B00,
            AddCtrlPressedHintText: vmslide + 0x1008FC60,
            GetCostHintText: vmslide + 0x100922F0,
        },
        UI: UIOffset {
            DrawItemProperties: vmslide + 0x100887A0, // GetKeyState(VK_CONTROL)
            BossLifeBar_Call_Units_GetName: vmslide + 0x100967AB, // BossLifebar:BossName
            MonsterLifeBar_Call_Units_GetName: vmslide + 0x10095D98, // game\\hud\\mon-hp-bar
            CheckIsMonsterShouldDisplayLifeBar: vmslide + 0x10095BE9, // game\\hud\\mon-hp-bar, test    eax, 201h
        },
    });
}

fn init_2_9_2(vmslide: usize) {
    AddressTable.initialize(D2SigmaOffset {
        AutoMap: AutoMapOffset {
            RevealMap: vmslide + 0x10091A90,
            DrawAutoMap: vmslide + 0x100511D0,
            // DrawAutoMapUnits    : vmslide + 0x10050CD0,
            DrawUnitBlob: vmslide + 0x10076890,
        },
        Units: UnitsOffset {
            GetName: vmslide + 0x100B8A20,
            DisplayItemProperties: vmslide + 0x10080E80,
        },
        Items: ItemsOffset {
            GetItemName: vmslide + 0x100811B0,
            GetItemNameColor: vmslide + 0x10081C80,
            LootFilterCheckUnit: 0,
        },
        ItemText: ItemTextOffset {
            GetItemPropertiesInit: vmslide + 0x10080930,
            GetItemProperties1: vmslide + 0x1008C080,
            GetItemProperties2: vmslide + 0x1008A7B0,
            GetItemProperties3: vmslide + 0x1008A4E0,
            GetItemProperties4: vmslide + 0x1008AB80,
            GetItemProperties5: vmslide + 0x10088250,
            GetItemProperties6: vmslide + 0x10088C50,
            GetItemProperties7: vmslide + 0x10088D80,
            GetItemProperties8: vmslide + 0x1008BF10,
            GetItemProperties9: vmslide + 0x10088180,
            GetItemProperties10: vmslide + 0x1008B9F0,
            GetItemProperties11: vmslide + 0x10087E40,
            GetItemProperties12: vmslide + 0x10089420,
            GetItemProperties13: vmslide + 0x1008C150,
            GetItemProperties14: vmslide + 0x1008AD20,
            GetItemProperties15: vmslide + 0x1008C600,
            GetItemProperties16: vmslide + 0x10088950,
            GetItemProperties17: vmslide + 0x10088040,
            GetItemProperties18: vmslide + 0x10089A90,
            GetItemProperties19: vmslide + 0x1008A030,
            GetItemProperties20: vmslide + 0x10089D60,
            GetItemProperties21: vmslide + 0x10089790,
            GetItemProperties22: vmslide + 0x10089290,
            GetItemProperties23: vmslide + 0x10081F80,
            GetItemProperties24: vmslide + 0x100821B0,
            GetItemProperties25: vmslide + 0x1008C860,
            GetItemProperties26: vmslide + 0x100890E0,
            GetItemProperties27: vmslide + 0x1008A300,
            GetName: vmslide + 0x10088BD0,
            GetItemProperties29: vmslide + 0x1008C3C0,
            AddCtrlPressedHintText: vmslide + 0x10089590,
            GetCostHintText: 0,
        },
        UI: UIOffset {
            DrawItemProperties: vmslide + 0x10082140, // GetKeyState(VK_CONTROL)
            BossLifeBar_Call_Units_GetName: vmslide + 0x1008FFCB, // BossLifebar:BossName
            MonsterLifeBar_Call_Units_GetName: vmslide + 0x1008F5AC, // game\\hud\\mon-hp-bar
            CheckIsMonsterShouldDisplayLifeBar: vmslide + 0x1008F3FD, // game\\hud\\mon-hp-bar, test    eax, 201h
        },
    });
}
