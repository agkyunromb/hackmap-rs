use super::common::*;

pub struct AutoMapOffset {
    pub RevealMap: FuncAddress,
}

pub struct UnitsOffset {
    pub Monster_GetName: FuncAddress,
}

pub struct UIOffset {
    pub BossLifeBar_Call_GetMonsterName     : FuncAddress,
    pub MonsterLifeBar_Call_GetMonsterName  : FuncAddress,
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
}

pub mod Units {
    use super::super::common::*;
    use super::AddressTable;

    pub fn Monster_GetName(unit: PVOID) -> PCWSTR {
        addr_to_fastcall(Monster_GetName, AddressTable.Units.Monster_GetName)(unit)
    }
}

pub fn initialized() ->bool {
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
                    RevealMap         : vmslide + 0x10091A90,
                },
                Units: UnitsOffset{
                    Monster_GetName   : vmslide + 0x100B8A20,
                },
                UI: UIOffset{
                    BossLifeBar_Call_GetMonsterName       : vmslide + 0x1008FFCB,   // BossLifebar:BossName
                    MonsterLifeBar_Call_GetMonsterName    : vmslide + 0x1008F5AC,   // game\\hud\\mon-hp-bar
                },
            });
        },

        0x6644F17E => {
            // 2.93

            AddressTable.initialize(D2SigmaOffset{
                AutoMap: AutoMapOffset{
                    RevealMap         : vmslide + 0x10091C10,
                },
                Units: UnitsOffset{
                    Monster_GetName   : vmslide + 0x100B8D80,
                },
                UI: UIOffset{
                    BossLifeBar_Call_GetMonsterName       : vmslide + 0x1009014B,
                    MonsterLifeBar_Call_GetMonsterName    : vmslide + 0x1008F72C,
                },
            });
        },

        _ => {},
    }
}
