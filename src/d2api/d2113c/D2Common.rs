use super::types;

pub use super::D2RVA;

pub struct StatListOffset {
    pub GetUnitBaseStat: types::FuncAddress,
}

pub struct DataTblsOffset {
    pub sgptDataTbls    : types::FuncAddress,
    pub CompileTxt      : types::FuncAddress,
}

pub struct D2CommonOffset {
    pub DataTbls: DataTblsOffset,
    pub StatList: StatListOffset,
}

pub static AddressTable: types::Holder<D2CommonOffset> = types::Holder::new();

pub mod StatList {
    use super::types::*;
    use super::D2RVA;
    use super::AddressTable;

    pub fn GetUnitBaseStat(unit: PVOID, statId: i32, layer:u16) -> usize {
        addr_to_stdcall(GetUnitBaseStat, AddressTable.StatList.GetUnitBaseStat)(unit, statId, layer)
    }
}

pub mod DataTbls {
    use super::types::*;
    use super::D2RVA;
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

    pub fn CompileTxt(archive: PVOID, name: *const u8, tbl: PVOID, recordCount: *mut i32, recordSize: usize) -> PVOID {
        addr_to_stdcall(CompileTxt, AddressTable.DataTbls.CompileTxt)(archive, name, tbl, recordCount, recordSize)
    }
}

pub fn init(d2common: usize) {
    AddressTable.initialize(D2CommonOffset{
        DataTbls: DataTblsOffset{
            sgptDataTbls    : d2common + D2RVA::D2Common(0x6FDE9E1C),
            CompileTxt      : d2common + D2RVA::D2Common(0x6FDAEF40),
        },
        StatList: StatListOffset{
            GetUnitBaseStat : d2common + D2RVA::D2Common(0x6FD88B70),
        },
    });
}
