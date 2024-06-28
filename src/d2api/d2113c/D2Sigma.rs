use super::common::*;

pub struct AutoMapOffset {
    pub RevealMap           : FuncAddress,
    pub DrawAutoMap         : FuncAddress,
    pub DrawAutoMapUnits    : FuncAddress,
    pub DrawUnitBlob        : FuncAddress,
}

pub struct UnitsOffset {
    pub Monster_GetName: FuncAddress,
}

pub struct UIOffset {
    pub BossLifeBar_Call_Units_GetName      : FuncAddress,
    pub MonsterLifeBar_Call_Units_GetName   : FuncAddress,
    pub CheckIsMonsterShouldDisplayLifeBar  : FuncAddress,
}

pub struct D2SigmaOffset {
    pub AutoMap : AutoMapOffset,
    pub Units   : UnitsOffset,
    pub UI      : UIOffset,
}

pub static AddressTable: OnceHolder<D2SigmaOffset> = OnceHolder::new();

pub mod AutoMap {
    use super::super::common::*;
    use super::AddressTable;

    pub fn RevealMap() {
        addr_to_stdcall(RevealMap, AddressTable.AutoMap.RevealMap)()
    }

    pub fn DrawUnitBlob(x: i32, y: i32, arg3: i32, color: u8) {
        addr_to_fastcall(DrawUnitBlob, AddressTable.AutoMap.DrawUnitBlob)(x, y, arg3, color)
    }
}

pub mod Units {
    use super::super::D2Common::Units::D2Unit;
    use super::super::common::*;
    use super::AddressTable;

    pub fn GetName(unit: &D2Unit) -> PCWSTR {
        addr_to_fastcall(GetName, AddressTable.Units.Monster_GetName)(unit)
    }
}

pub fn initialized() ->bool {
    // return false;
    AddressTable.initialized()
}

pub fn init(d2sigma: usize) {
    let timestamp = unsafe { (&*RtlImageNtHeader(d2sigma as PVOID)).FileHeader.TimeDateStamp };

    const D2Sigma_BaseAddress: usize = 0x10000000;

    let vmslide = d2sigma - D2Sigma_BaseAddress;

    match timestamp {
        0x663D01B3 => {
            // 2.92

            AddressTable.initialize(D2SigmaOffset{
                AutoMap: AutoMapOffset{
                    RevealMap           : vmslide + 0x10091A90,
                    DrawAutoMap         : vmslide + 0x100511D0,
                    DrawAutoMapUnits    : vmslide + 0x10050CD0,
                    DrawUnitBlob        : vmslide + 0x10076890,
                },
                Units: UnitsOffset{
                    Monster_GetName   : vmslide + 0x100B8A20,
                },
                UI: UIOffset{
                    BossLifeBar_Call_Units_GetName      : vmslide + 0x1008FFCB,   // BossLifebar:BossName
                    MonsterLifeBar_Call_Units_GetName   : vmslide + 0x1008F5AC,   // game\\hud\\mon-hp-bar
                    CheckIsMonsterShouldDisplayLifeBar  : vmslide + 0x1008F3FD,   // game\\hud\\mon-hp-bar, test    eax, 201h
                },
            });
        },

        0x6644F17E => {
            // 2.93

            AddressTable.initialize(D2SigmaOffset{
                AutoMap: AutoMapOffset{
                    RevealMap           : vmslide + 0x10091C10,
                    DrawAutoMap         : 0,
                    DrawAutoMapUnits    : 0,
                    DrawUnitBlob        : 0,
                },
                Units: UnitsOffset{
                    Monster_GetName   : vmslide + 0x100B8D80,
                },
                UI: UIOffset{
                    BossLifeBar_Call_Units_GetName      : vmslide + 0x1009014B,
                    MonsterLifeBar_Call_Units_GetName   : vmslide + 0x1008F72C,
                    CheckIsMonsterShouldDisplayLifeBar  : 0,
                },
            });
        },

        _ => {},
    }
}
