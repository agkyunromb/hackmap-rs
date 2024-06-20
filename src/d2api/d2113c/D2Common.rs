use super::common::*;

pub struct StatListOffset {
    pub GetUnitBaseStat: FuncAddress,
}

pub struct DataTblsOffset {
    pub sgptDataTbls    : FuncAddress,
    pub CompileTxt      : FuncAddress,
}

pub struct UnitsOffset {
    pub TestCollisionWithUnit: FuncAddress,
}

pub struct D2CommonOffset {
    pub DataTbls    : DataTblsOffset,
    pub StatList    : StatListOffset,
    pub Units       : UnitsOffset,
}

pub static AddressTable: OnceHolder<D2CommonOffset> = OnceHolder::new();

pub mod StatList {
    use super::super::common::*;
    use super::AddressTable;

    pub fn GetUnitBaseStat(unit: PVOID, statId: i32, layer:u16) -> usize {
        addr_to_stdcall(GetUnitBaseStat, AddressTable.StatList.GetUnitBaseStat)(unit, statId, layer)
    }
}

pub mod DataTbls {
    use super::super::common::*;
    use super::AddressTable;

    pub struct DataTable(PVOID);

    impl DataTable {
        pub fn mon_stats_txt(&self) -> PVOID {
            unsafe {
                std::ptr::read((self.0 as usize + 0xA78) as *const PVOID)
            }
        }

        pub fn mon_stats_txt_record_count(&self) -> usize {
            unsafe {
                std::ptr::read((self.0 as usize + 0xA80) as *const usize)
            }
        }
    }

    pub fn sgptDataTbls() -> DataTable {
        unsafe {
            DataTable(std::ptr::read(AddressTable.DataTbls.sgptDataTbls as *const PVOID))
        }
    }

    pub fn CompileTxt(archive: PVOID, name: *const u8, tbl: PVOID, recordCount: &mut i32, recordSize: usize) -> PVOID {
        addr_to_stdcall(CompileTxt, AddressTable.DataTbls.CompileTxt)(archive, name, tbl, recordCount, recordSize)
    }
}

pub mod Units {
    use super::super::common::*;
    use super::AddressTable;

    pub fn TestCollisionWithUnit(unit1: PVOID, unit2: PVOID, collision_mask: i32) -> BOOL {
        addr_to_stdcall(TestCollisionWithUnit, AddressTable.Units.TestCollisionWithUnit)(unit1, unit2, collision_mask)
    }
}

pub fn init(d2common: usize) {
    AddressTable.initialize(D2CommonOffset{
        DataTbls: DataTblsOffset{
            sgptDataTbls            : d2common + D2RVA::D2Common(0x6FDE9E1C),
            CompileTxt              : d2common + D2RVA::D2Common(0x6FDAEF40),
        },
        StatList: StatListOffset{
            GetUnitBaseStat         : d2common + D2RVA::D2Common(0x6FD88B70),
        },
        Units: UnitsOffset{
            TestCollisionWithUnit   : d2common + D2RVA::D2Common(0x6FD814A0),
        }
    });
}
