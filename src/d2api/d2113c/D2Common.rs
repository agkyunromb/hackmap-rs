use super::common::*;
mod drlg;
mod units;
mod datatbls;

pub struct StatListOffset {
    pub GetUnitBaseStat         : FuncAddress,
}

pub struct DataTblsOffset {
    pub CompileTxt              : FuncAddress,
    pub GetLevelDefRecord       : FuncAddress,
    pub GetObjectsTxtRecord     : FuncAddress,
    pub sgptDataTables          : FuncAddress,
}

pub struct UnitsOffset {
    pub TestCollisionWithUnit   : FuncAddress,
}

pub struct DrlgDrlgOffset {
    pub GetActNoFromLevelId     : FuncAddress,
    pub GetLevel                : FuncAddress,
}

pub struct DrlgRoomOffset {
    pub GetPresetUnits          : FuncAddress,
    pub GetLevelId              : FuncAddress,
}

pub struct DrlgPresetOffset {
    pub GetLevelPrestIdFromRoomEx: FuncAddress,
}

pub struct DungeonOffset {
    pub GetDrlgFromAct              : FuncAddress,
    pub IsTownLevelId               : FuncAddress,
    pub GetHoradricStaffTombLevelId : FuncAddress,
}

pub struct D2CommonOffset {
    pub DataTbls        : DataTblsOffset,
    pub StatList        : StatListOffset,
    pub Units           : UnitsOffset,
    pub DrlgDrlg        : DrlgDrlgOffset,
    pub DrlgRoom        : DrlgRoomOffset,
    pub DrlgPreset      : DrlgPresetOffset,
    pub Dungeon         : DungeonOffset,
}

pub static AddressTable: OnceHolder<D2CommonOffset> = OnceHolder::new();

pub mod StatList {
    use super::super::common::*;
    use super::AddressTable;

    pub fn GetUnitBaseStat(unit: PVOID, statId: D2ItemStats, layer:u16) -> usize {
        addr_to_stdcall(GetUnitBaseStat, AddressTable.StatList.GetUnitBaseStat)(unit, statId, layer)
    }
}

pub mod DataTbls {
    use std::ptr::null_mut;

    use super::super::common::*;
    use super::AddressTable;
    use super::DrlgPreset::D2LevelId;
    pub use super::datatbls::*;

    pub struct DataTable(usize);

    impl DataTable {
        pub fn mon_stats_txt(&self) -> PVOID {
            unsafe {
                std::ptr::read((self.0 + 0xA78) as *const PVOID)
            }
        }

        pub fn mon_stats_txt_record_count(&self) -> usize {
            unsafe {
                std::ptr::read((self.0 + 0xA80) as *const usize)
            }
        }

        pub fn get_levels_txt_record(&self, level_id: D2LevelId) -> Option<&D2LevelsTxt> {
            let count = self.levels_txt_record_count();
            if level_id >= count {
                return None;
            }

            let level_id = level_id as usize;
            let count = count as usize;

            let levels_txt: *const D2LevelsTxt = read_at(self.0 + 0xC58);
            let levels_txt = unsafe { std::slice::from_raw_parts(levels_txt, count) };

            return Some(&levels_txt[level_id])
        }

        pub fn levels_txt_record_count(&self) -> D2LevelId {
            read_at(self.0 + 0xC5C)
        }
    }

    pub fn sgptDataTables() -> DataTable {
        DataTable(read_at(AddressTable.DataTbls.sgptDataTables))
    }

    pub fn _GetLevelDefRecord(_levelId: D2LevelId) -> *mut D2LevelDefBin { null_mut() }

    pub fn GetLevelDefRecord(levelId: D2LevelId) -> Option<&'static mut D2LevelDefBin> {
        let max_level_id = sgptDataTables().levels_txt_record_count();

        if levelId >= max_level_id {
            return None;
        }

        unsafe {
            Some(&mut *addr_to_fastcall(_GetLevelDefRecord, AddressTable.DataTbls.GetLevelDefRecord)(levelId))
        }
    }

    pub fn _GetObjectsTxtRecord(_objectId: i32) -> *mut D2ObjectsTxt { null_mut() }

    pub fn GetObjectsTxtRecord(objectId: i32) -> Option<&'static mut D2ObjectsTxt> {
        let object_txt = addr_to_stdcall(_GetObjectsTxtRecord, AddressTable.DataTbls.GetObjectsTxtRecord)(objectId);
        ptr_to_ref_mut(object_txt)
    }

    pub fn CompileTxt(archive: PVOID, name: *const u8, tbl: PVOID, recordCount: &mut i32, recordSize: usize) -> PVOID {
        addr_to_stdcall(CompileTxt, AddressTable.DataTbls.CompileTxt)(archive, name, tbl, recordCount, recordSize)
    }
}

pub mod Units {
    use super::super::common::*;
    use super::AddressTable;
    pub use super::units::*;

    pub fn TestCollisionWithUnit(unit1: PVOID, unit2: PVOID, collision_mask: i32) -> BOOL {
        addr_to_stdcall(TestCollisionWithUnit, AddressTable.Units.TestCollisionWithUnit)(unit1, unit2, collision_mask)
    }
}

pub mod DrlgDrlg {
    use super::super::common::*;
    use super::AddressTable;
    pub use super::drlg::*;

    pub fn GetActNoFromLevelId(levelId: D2LevelId) -> u8 {
        addr_to_stdcall(GetActNoFromLevelId, AddressTable.DrlgDrlg.GetActNoFromLevelId)(levelId)
    }

    pub fn _GetLevel(_drlg: &D2Drlg, _levelId: D2LevelId) -> *mut D2DrlgLevel { null_mut() }

    pub fn GetLevel(drlg: &D2Drlg, levelId: D2LevelId) -> Option<&mut D2DrlgLevel> {
        let drlg_level = addr_to_stdcall(_GetLevel, AddressTable.DrlgDrlg.GetLevel)(drlg, levelId);
        ptr_to_ref_mut(drlg_level)
    }
}

pub mod DrlgRoom {
    use super::super::common::*;
    use super::AddressTable;
    pub use super::drlg::*;

    pub fn _GetPresetUnits(_drlgRoom: &D2DrlgRoom) -> *mut D2PresetUnit { null_mut() }

    pub fn GetPresetUnits(drlgRoom: &D2DrlgRoom) -> Option<&'static mut D2PresetUnit> {
        ptr_to_ref_mut(addr_to_fastcall(_GetPresetUnits, AddressTable.DrlgRoom.GetPresetUnits)(drlgRoom))
    }

    pub fn GetLevelId(drlgRoom: *const D2DrlgRoom) -> D2LevelId {
        let levelId: D2LevelId;

        unsafe {
            asm!(
                "mov  eax, {0}",
                "call {1}",
                in(reg) drlgRoom,
                in(reg) AddressTable.DrlgRoom.GetLevelId,
                lateout("eax") levelId,
                options(nostack),
            );
        }

        levelId
    }
}

pub mod Dungeon {
    use super::super::common::*;
    use super::AddressTable;
    pub use super::drlg::*;

    pub fn GetActNoFromLevelId(levelId: D2LevelId) -> u8 {
        addr_to_stdcall(GetActNoFromLevelId, AddressTable.DrlgDrlg.GetActNoFromLevelId)(levelId)
    }

    pub fn _GetDrlgFromAct(_act: &D2DrlgAct) -> *mut D2Drlg { null_mut() }

    pub fn GetDrlgFromAct(act: &D2DrlgAct) -> Option<&mut D2Drlg> {
        let drlg = addr_to_stdcall(_GetDrlgFromAct, AddressTable.Dungeon.GetDrlgFromAct)(act);
        ptr_to_ref_mut(drlg)
    }

    pub fn IsTownLevelId(levelId: D2LevelId) -> BOOL {
        addr_to_stdcall(IsTownLevelId, AddressTable.Dungeon.IsTownLevelId)(levelId)
    }

    pub fn GetHoradricStaffTombLevelId(drlgAct: &D2DrlgAct) -> D2LevelId {
        addr_to_stdcall(GetHoradricStaffTombLevelId, AddressTable.Dungeon.GetHoradricStaffTombLevelId)(drlgAct)
    }
}

pub mod DrlgPreset {
    use super::super::common::*;
    use super::AddressTable;
    pub use super::drlg::*;

    pub fn GetLevelPrestIdFromRoomEx(drlg_room: &D2DrlgRoom) -> i32 {
        addr_to_fastcall(GetLevelPrestIdFromRoomEx, AddressTable.DrlgPreset.GetLevelPrestIdFromRoomEx)(drlg_room)
    }
}

pub fn init(d2common: usize) {
    AddressTable.initialize(D2CommonOffset{
        DataTbls: DataTblsOffset{
            CompileTxt                  : d2common + D2RVA::D2Common(0x6FDAEF40),
            GetLevelDefRecord           : d2common + D2RVA::D2Common(0x6FDBCB20),
            GetObjectsTxtRecord         : d2common + D2RVA::D2Common(0x6FD8E980),
            sgptDataTables              : d2common + D2RVA::D2Common(0x6FDE9E1C),
        },
        StatList: StatListOffset{
            GetUnitBaseStat             : d2common + D2RVA::D2Common(0x6FD88B70),
        },
        Units: UnitsOffset{
            TestCollisionWithUnit       : d2common + D2RVA::D2Common(0x6FD814A0),
        },
        DrlgDrlg: DrlgDrlgOffset{
            GetActNoFromLevelId         : d2common + D2RVA::D2Common(0x6FD7D2C0),
            GetLevel                    : d2common + D2RVA::D2Common(0x6FD7DD80),
        },
        DrlgRoom: DrlgRoomOffset{
            GetPresetUnits              : d2common + D2RVA::D2Common(0x6FD94460),
            GetLevelId                  : d2common + D2RVA::D2Common(0x6FD94690),
        },
        DrlgPreset: DrlgPresetOffset{
            GetLevelPrestIdFromRoomEx   : d2common + D2RVA::D2Common(0x6FD59C20),
        },
        Dungeon: DungeonOffset{
            GetDrlgFromAct              : d2common + D2RVA::D2Common(0x6FD8B270),
            IsTownLevelId               : d2common + D2RVA::D2Common(0x6FD8B230),
            GetHoradricStaffTombLevelId : d2common + D2RVA::D2Common(0x6FD8B080),
        },
    });
}
