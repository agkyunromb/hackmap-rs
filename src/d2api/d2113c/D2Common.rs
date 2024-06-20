use super::types;

pub use super::D2RVA;

pub struct StatListOffset {
    pub GetUnitBaseStat: types::FuncAddress,
}

pub struct D2CommonOffset {
    StatList: StatListOffset,
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

pub fn init(d2common: usize) {
    AddressTable.initialize(D2CommonOffset{
        StatList: StatListOffset{
            GetUnitBaseStat: d2common + D2RVA::D2Common(0x6FD88B70),
        },
    });
}
