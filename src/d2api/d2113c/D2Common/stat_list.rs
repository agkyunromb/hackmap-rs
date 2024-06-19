use super::types::*;

pub struct StatListOffset {
    pub GetUnitBaseStat: FuncAddress,
}

pub static AddressTable: Holder<StatListOffset> = Holder::new();

pub fn GetUnitBaseStat(unit: PVOID, statId: i32, layer:u16) -> usize {
    addr_to_stdcall(GetUnitBaseStat, AddressTable.GetUnitBaseStat)(unit, statId, layer)
}

pub(super) fn init(d2common: usize) {
    AddressTable.initialize(StatListOffset {
        GetUnitBaseStat: d2common + 0x12345,
    });
}
