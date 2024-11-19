use super::common::*;
mod drlg;
mod units;
mod datatbls;
mod packet;

pub use units::*;
pub use datatbls::*;
pub use drlg::*;
pub use packet::*;

pub struct StatListOffset {
    pub GetUnitBaseStat                 : FuncAddress,
    pub GetStatListFromUnitStateAndFlag : FuncAddress,
    pub GetUnitAlignment                : FuncAddress,
}

pub struct DataTblsOffset {
    pub CompileTxt                          : FuncAddress,
    pub GetLevelDefRecord                   : FuncAddress,
    pub GetObjectsTxtRecord                 : FuncAddress,
    pub GetItemDataTables                   : FuncAddress,
    pub GetNextHirelingTxtRecordFromClassId : FuncAddress,
    pub sgptDataTables                      : FuncAddress,
}

pub struct UnitsOffset {
    pub TestCollisionWithUnit   : FuncAddress,
    pub GetRoom                 : FuncAddress,
    pub GetNearestTestedUnit    : FuncAddress,
    pub GetClientCoordX         : FuncAddress,
    pub GetClientCoordY         : FuncAddress,
    pub GetDistanceToCoordinates: FuncAddress,
    pub GetCoords               : FuncAddress,
    pub GetInventoryRecordId    : FuncAddress,
    pub GetPlayerName           : FuncAddress,
}

pub struct ItemsOffset {
    pub GetItemType             : FuncAddress,
    pub GetItemQuality          : FuncAddress,
    pub GetInvPage              : FuncAddress,
    pub GetBaseCode             : FuncAddress,
    pub CheckItemTypeId         : FuncAddress,
    pub CheckItemFlag           : FuncAddress,
}

pub struct InventoryOffset {
    pub UnitIsItem              : FuncAddress,
    pub GetFirstItem            : FuncAddress,
    pub GetNextItem             : FuncAddress,
    pub GetCursorItem           : FuncAddress,
    pub GetFreePosition         : FuncAddress,
    pub GetFreeBeltSlot         : FuncAddress,
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
    pub GetDrlgFromAct                  : FuncAddress,
    pub IsTownLevelId                   : FuncAddress,
    pub IsRoomInTown                    : FuncAddress,
    pub GetHoradricStaffTombLevelId     : FuncAddress,
    pub GetRoomFromAct                  : FuncAddress,
    pub GetAdjacentRoomsListFromRoom    : FuncAddress,
    pub GetLevelIdFromRoom              : FuncAddress,
}

pub struct D2CommonOffset {
    pub DataTbls        : DataTblsOffset,
    pub StatList        : StatListOffset,
    pub Units           : UnitsOffset,
    pub Items           : ItemsOffset,
    pub Inventory       : InventoryOffset,
    pub DrlgDrlg        : DrlgDrlgOffset,
    pub DrlgRoom        : DrlgRoomOffset,
    pub DrlgPreset      : DrlgPresetOffset,
    pub Dungeon         : DungeonOffset,
}

pub static AddressTable: OnceHolder<D2CommonOffset> = OnceHolder::new();

pub mod StatList {
    use super::*;

    pub fn GetUnitBaseStat(unit: &D2Unit, statId: D2ItemStats, layer: u16) -> usize {
        addr_to_stdcall(GetUnitBaseStat, AddressTable.StatList.GetUnitBaseStat)(unit, statId, layer)
    }

    pub fn GetStatListFromUnitStateAndFlag(unit: &D2Unit, state: i32, flag: u32) -> usize {
        addr_to_stdcall(GetStatListFromUnitStateAndFlag, AddressTable.StatList.GetStatListFromUnitStateAndFlag)(unit, state, flag)
    }

    pub fn GetUnitAlignment(unit: &D2Unit) -> D2UnitAlignment {
        addr_to_stdcall(GetUnitAlignment, AddressTable.StatList.GetUnitAlignment)(unit)
    }
}

pub mod DataTbls {
    use std::ptr::null_mut;
    use super::*;

    pub struct DataTable(usize);

    impl DataTable {
        pub fn mon_stats_txt(&self) -> &mut [D2MonStatsTxt] {
            unsafe {
                let addr = (self.0 + 0xA78) as *const usize;
                std::slice::from_raw_parts_mut(addr.read() as *mut D2MonStatsTxt, self.mon_stats_txt_record_count())
            }
        }

        pub fn mon_stats_txt_record_count(&self) -> usize {
            unsafe {
                ((self.0 + 0xA80) as *const usize).read()
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

        pub fn objects_txt_record_count(&self) -> usize {
            read_at(self.0 + 0xCC0)
        }
    }

    pub fn sgptDataTables() -> DataTable {
        DataTable(read_at(AddressTable.DataTbls.sgptDataTables))
    }

    fn _GetLevelDefRecord(_levelId: D2LevelId) -> *mut D2LevelDefBin { null_mut() }

    pub fn GetLevelDefRecord(levelId: D2LevelId) -> Option<&'static mut D2LevelDefBin> {
        let max_level_id = sgptDataTables().levels_txt_record_count();

        if levelId >= max_level_id {
            return None;
        }

        unsafe {
            Some(&mut *addr_to_fastcall(_GetLevelDefRecord, AddressTable.DataTbls.GetLevelDefRecord)(levelId))
        }
    }

    fn _GetObjectsTxtRecord(_objectId: u32) -> *mut D2ObjectsTxt { null_mut() }

    pub fn GetObjectsTxtRecord(objectId: u32) -> Option<&'static mut D2ObjectsTxt> {
        if objectId as usize >= sgptDataTables().objects_txt_record_count() {
            return None;
        }

        let object_txt = addr_to_stdcall(_GetObjectsTxtRecord, AddressTable.DataTbls.GetObjectsTxtRecord)(objectId);
        ptr_to_ref_mut(object_txt)
    }

    fn _GetItemDataTables() -> *mut D2ItemDataTbl { null_mut() }

    pub fn GetItemDataTables() -> Option<&'static mut D2ItemDataTbl> {
        ptr_to_ref_mut(addr_to_stdcall(_GetItemDataTables, AddressTable.DataTbls.GetItemDataTables)())
    }

    fn _GetNextHirelingTxtRecordFromClassId(_bExpansion: BOOL, _nClass: u32, _pOldRecord: PVOID) -> *mut D2HirelingTxt { null_mut() }

    pub fn GetNextHirelingTxtRecordFromClassId(expansion: BOOL, classId: u32, oldRecord: PVOID) -> Option<&'static mut D2HirelingTxt> {
        ptr_to_ref_mut(addr_to_stdcall(_GetNextHirelingTxtRecordFromClassId, AddressTable.DataTbls.GetNextHirelingTxtRecordFromClassId)(expansion, classId, oldRecord))
    }

    pub fn CompileTxt(archive: PVOID, name: *const u8, tbl: PVOID, recordCount: &mut i32, recordSize: usize) -> PVOID {
        addr_to_stdcall(CompileTxt, AddressTable.DataTbls.CompileTxt)(archive, name, tbl, recordCount, recordSize)
    }
}

pub mod Units {
    use super::*;

    pub fn TestCollisionWithUnit(unit1: PVOID, unit2: PVOID, collision_mask: i32) -> BOOL {
        addr_to_stdcall(TestCollisionWithUnit, AddressTable.Units.TestCollisionWithUnit)(unit1, unit2, collision_mask)
    }

    fn _GetRoom(_unit: &D2Unit) -> *mut D2ActiveRoom { null_mut() }
    pub fn GetRoom(unit: &D2Unit) -> Option<&mut D2ActiveRoom> {
        ptr_to_ref_mut(addr_to_stdcall(_GetRoom, AddressTable.Units.GetRoom)(unit))
    }

    pub fn GetClientCoordX(unit: &D2Unit) -> i32 {
        addr_to_stdcall(GetClientCoordX, AddressTable.Units.GetClientCoordX)(unit)
    }

    pub fn GetClientCoordY(unit: &D2Unit) -> i32 {
        addr_to_stdcall(GetClientCoordY, AddressTable.Units.GetClientCoordY)(unit)
    }

    pub fn GetDistanceToCoordinates(unit: &D2Unit, x: i32, y: i32) -> i32 {
        addr_to_stdcall(GetDistanceToCoordinates, AddressTable.Units.GetDistanceToCoordinates)(unit, x, y)
    }

    fn _GetCoords(_unit: &D2Unit, _coord: &mut D2Coord) {}

    pub fn GetCoords(unit: &D2Unit) -> D2Coord {
        let mut coord =  D2Coord { nX: 0, nY: 0 };
        addr_to_stdcall(_GetCoords, AddressTable.Units.GetCoords)(unit, &mut coord);
        coord
    }

    pub fn GetInventoryRecordId(unit: &D2Unit, invPage: D2ItemInvPage, isLod: BOOL) -> i32 {
        addr_to_stdcall(GetInventoryRecordId, AddressTable.Units.GetInventoryRecordId)(unit, invPage, isLod)
    }

    fn _GetPlayerName(_unit: &D2Unit) -> PCSTR { null_mut() }

    pub fn GetPlayerName(unit: &D2Unit) -> Option<String> {
        let name = addr_to_stdcall(_GetPlayerName, AddressTable.Units.GetPlayerName)(unit);

        if name.is_null() {
            return None;
        }

        Some(name.to_str().to_string())
    }

}

pub mod Items {
    use super::*;

    pub fn GetItemType(item: &D2Unit) -> BOOL {
        addr_to_stdcall(GetItemType, AddressTable.Items.GetItemType)(item)
    }

    pub fn GetItemQuality(item: &D2Unit) -> D2ItemQualities {
        addr_to_stdcall(GetItemQuality, AddressTable.Items.GetItemQuality)(item)
    }

    pub fn GetInvPage(item: &D2Unit) -> D2ItemInvPage {
        addr_to_stdcall(GetInvPage, AddressTable.Items.GetInvPage)(item)
    }

    pub fn GetBaseCode(item: &D2Unit) -> u32 {
        addr_to_stdcall(GetBaseCode, AddressTable.Items.GetBaseCode)(item)
    }

    pub fn CheckItemTypeId(item: &D2Unit, itemType: i32) -> BOOL {
        addr_to_stdcall(CheckItemTypeId, AddressTable.Items.CheckItemTypeId)(item, itemType)
    }

    fn _CheckItemFlag(_item: &D2Unit, _flags: D2ItemFlags, _line: usize, _file: usize) -> BOOL { FALSE }

    pub fn CheckItemFlag(item: &D2Unit, flags: D2ItemFlags) -> BOOL {
        addr_to_stdcall(_CheckItemFlag, AddressTable.Items.CheckItemFlag)(item, flags, 0, 0)
    }
}

pub mod Inventory {
    use super::*;

    fn _UnitIsItem(_unit: &D2Unit) -> *mut D2Unit { null_mut() }

    pub fn UnitIsItem(unit: &D2Unit) -> BOOL {
        if addr_to_stdcall(_UnitIsItem, AddressTable.Inventory.UnitIsItem)(unit).is_null() { FALSE } else { TRUE }
    }

    fn _GetFirstItem(_inventory: &D2Inventory) -> *mut D2Unit { null_mut() }

    pub fn GetFirstItem(inventory: &D2Inventory) -> Option<&mut D2Unit> {
        ptr_to_ref_mut(addr_to_stdcall(_GetFirstItem, AddressTable.Inventory.GetFirstItem)(inventory))
    }

    fn _GetNextItem(_item: &D2Unit) -> *mut D2Unit { null_mut() }

    pub fn GetNextItem(item: &D2Unit) -> Option<&mut D2Unit> {
        ptr_to_ref_mut(addr_to_stdcall(_GetNextItem, AddressTable.Inventory.GetNextItem)(item))
    }

    fn _GetCursorItem(_inventory: &D2Inventory) -> *mut D2Unit { null_mut() }

    pub fn GetCursorItem(inventory: &D2Inventory) -> Option<&mut D2Unit> {
        ptr_to_ref_mut(addr_to_stdcall(_GetCursorItem, AddressTable.Inventory.GetCursorItem)(inventory))
    }

    fn _GetFreePosition(_inventory: &D2Inventory, _item: &D2Unit, _inventoryRecordId: i32, _x: *mut i32, _y: *mut i32, _page: D2ItemInvPage) -> BOOL { FALSE }

    pub fn GetFreePosition(inventory: &D2Inventory, item: &D2Unit, inventoryRecordId: i32, page: D2ItemInvPage) -> Option<(i32, i32)> {
        let mut x = 0;
        let mut y = 0;

        if addr_to_stdcall(_GetFreePosition, AddressTable.Inventory.GetFreePosition)(inventory, item, inventoryRecordId, &mut x, &mut y, page) == FALSE {
            return None;
        }

        Some((x, y))
    }

    fn _GetFreeBeltSlot(_inventory: &D2Inventory, _item: &D2Unit, _freeSlotId: *mut i32) -> BOOL { FALSE }

    pub fn GetFreeBeltSlot(inventory: &D2Inventory, item: &D2Unit) -> Option<i32> {
        let mut freeSlotId: i32 = 0;

        if addr_to_stdcall(_GetFreeBeltSlot, AddressTable.Inventory.GetFreeBeltSlot)(inventory, item, &mut freeSlotId) == FALSE {
            return None;
        }

        Some(freeSlotId)
    }

}

pub mod DrlgDrlg {
    use super::*;

    pub fn GetActNoFromLevelId(levelId: D2LevelId) -> u8 {
        addr_to_stdcall(GetActNoFromLevelId, AddressTable.DrlgDrlg.GetActNoFromLevelId)(levelId)
    }

    fn _GetLevel(_drlg: &D2Drlg, _levelId: D2LevelId) -> *mut D2DrlgLevel { null_mut() }

    pub fn GetLevel(drlg: &D2Drlg, levelId: D2LevelId) -> Option<&mut D2DrlgLevel> {
        let drlg_level = addr_to_stdcall(_GetLevel, AddressTable.DrlgDrlg.GetLevel)(drlg, levelId);
        ptr_to_ref_mut(drlg_level)
    }
}

pub mod DrlgRoom {
    use super::*;

    fn _GetPresetUnits(_drlgRoom: &D2DrlgRoom) -> *mut D2PresetUnit { null_mut() }

    pub fn GetPresetUnits(drlgRoom: &D2DrlgRoom) -> Option<&'static mut D2PresetUnit> {
        ptr_to_ref_mut(addr_to_fastcall(_GetPresetUnits, AddressTable.DrlgRoom.GetPresetUnits)(drlgRoom))
    }

    pub fn GetLevelId(drlgRoom: *const D2DrlgRoom) -> D2LevelId {
        let levelId: D2LevelId;

        unsafe {
            asm!(
                "call ecx",
                in("eax") drlgRoom,
                in("ecx") AddressTable.DrlgRoom.GetLevelId,
                lateout("eax") levelId,
            );
        }

        levelId
    }
}

pub mod Dungeon {
    use super::super::common::*;
    use super::datatbls::D2LevelDefBin;
    use super::AddressTable;
    pub use super::drlg::*;

    fn _GetDrlgFromAct(_act: &D2DrlgAct) -> *mut D2Drlg { null_mut() }

    pub fn GetDrlgFromAct(act: &D2DrlgAct) -> Option<&mut D2Drlg> {
        let drlg = addr_to_stdcall(_GetDrlgFromAct, AddressTable.Dungeon.GetDrlgFromAct)(act);
        ptr_to_ref_mut(drlg)
    }

    pub fn IsTownLevelId(levelId: D2LevelId) -> BOOL {
        addr_to_stdcall(IsTownLevelId, AddressTable.Dungeon.IsTownLevelId)(levelId)
    }

    pub fn IsRoomInTown(activeRoom: &D2ActiveRoom) -> BOOL {
        addr_to_stdcall(IsRoomInTown, AddressTable.Dungeon.IsRoomInTown)(activeRoom)
    }

    pub fn GetHoradricStaffTombLevelId(drlgAct: &D2DrlgAct) -> D2LevelId {
        addr_to_stdcall(GetHoradricStaffTombLevelId, AddressTable.Dungeon.GetHoradricStaffTombLevelId)(drlgAct)
    }

    pub fn GetRoomFromAct(drlgAct: &D2DrlgAct) -> Option<&mut D2ActiveRoom> {
        addr_to_stdcall(GetRoomFromAct, AddressTable.Dungeon.GetRoomFromAct)(drlgAct)
    }

    fn _GetAdjacentRoomsListFromRoom(_activeRoom: &D2ActiveRoom, _roomList: *mut *mut *mut D2ActiveRoom, _roomCount: *mut usize) {}

    pub fn GetAdjacentRoomsListFromRoom(activeRoom: &D2ActiveRoom) -> Option<&[*mut D2ActiveRoom]> {
        let mut rooms: *mut *mut D2ActiveRoom = null_mut();
        let mut room_count = 0_usize;

        addr_to_stdcall(_GetAdjacentRoomsListFromRoom, AddressTable.Dungeon.GetAdjacentRoomsListFromRoom)(&activeRoom, &mut rooms, &mut room_count);

        if room_count == 0 {
            return None;
        }

        unsafe {
            let s = std::slice::from_raw_parts_mut(rooms, room_count);

            Some(s)
        }
    }

    pub fn GetLevelIdFromRoom(room: &D2ActiveRoom) -> D2LevelId {
        addr_to_stdcall(GetLevelIdFromRoom, AddressTable.Dungeon.GetLevelIdFromRoom)(room)
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
            CompileTxt                          : d2common + D2RVA::D2Common(0x6FDAEF40),
            GetLevelDefRecord                   : d2common + D2RVA::D2Common(0x6FDBCB20),
            GetObjectsTxtRecord                 : d2common + D2RVA::D2Common(0x6FD8E980),
            GetItemDataTables                   : d2common + D2RVA::D2Common(0x6FDC1A40),
            GetNextHirelingTxtRecordFromClassId : d2common + D2RVA::D2Common(0x6FDA3190),
            sgptDataTables                      : d2common + D2RVA::D2Common(0x6FDE9E1C),
        },
        StatList: StatListOffset{
            GetUnitBaseStat                     : d2common + D2RVA::D2Common(0x6FD88B70),
            GetStatListFromUnitStateAndFlag     : d2common + D2RVA::D2Common(0x6FD87EC0),
            GetUnitAlignment                    : d2common + D2RVA::D2Common(0x6FD891F0),
        },
        Units: UnitsOffset{
            TestCollisionWithUnit               : d2common + D2RVA::D2Common(0x6FD814A0),
            GetRoom                             : d2common + D2RVA::D2Common(0x6FD7FE10),
            GetNearestTestedUnit                : d2common + D2RVA::D2Common(0x6FD62330),
            GetClientCoordX                     : d2common + D2RVA::D2Common(0x6FD80290),
            GetClientCoordY                     : d2common + D2RVA::D2Common(0x6FD80240),
            GetDistanceToCoordinates            : d2common + D2RVA::D2Common(0x6FDCF5E0),
            GetCoords                           : d2common + D2RVA::D2Common(0x6FD80050),
            GetInventoryRecordId                : d2common + D2RVA::D2Common(0x6FD7FB60),
            GetPlayerName                       : d2common + D2RVA::D2Common(0x6FD7EBB0),
        },
        Items: ItemsOffset{
            GetItemType                         : d2common + D2RVA::D2Common(0x6FD730F0),
            GetItemQuality                      : d2common + D2RVA::D2Common(0x6FD73B40),
            GetInvPage                          : d2common + D2RVA::D2Common(0x6FD737C0),
            GetBaseCode                         : d2common + D2RVA::D2Common(0x6FD73290),
            CheckItemTypeId                     : d2common + D2RVA::D2Common(0x6FD74430),
            CheckItemFlag                       : d2common + D2RVA::D2Common(0x6FD73940),
        },
        Inventory: InventoryOffset{
            UnitIsItem                          : d2common + D2RVA::D2Common(0x6FD6E400),
            GetFirstItem                        : d2common + D2RVA::D2Common(0x6FD6E190),
            GetNextItem                         : d2common + D2RVA::D2Common(0x6FD6E8F0),
            GetCursorItem                       : d2common + D2RVA::D2Common(0x6FD6DFB0),
            GetFreePosition                     : d2common + D2RVA::D2Common(0x6FD708E0),
            GetFreeBeltSlot                     : d2common + D2RVA::D2Common(0x6FD700D0),
        },
        DrlgDrlg: DrlgDrlgOffset{
            GetActNoFromLevelId                 : d2common + D2RVA::D2Common(0x6FD7D2C0),
            GetLevel                            : d2common + D2RVA::D2Common(0x6FD7DD80),
        },
        DrlgRoom: DrlgRoomOffset{
            GetPresetUnits                      : d2common + D2RVA::D2Common(0x6FD94460),
            GetLevelId                          : d2common + D2RVA::D2Common(0x6FD94690),
        },
        DrlgPreset: DrlgPresetOffset{
            GetLevelPrestIdFromRoomEx           : d2common + D2RVA::D2Common(0x6FD59C20),
        },
        Dungeon: DungeonOffset{
            GetDrlgFromAct                      : d2common + D2RVA::D2Common(0x6FD8B270),
            IsTownLevelId                       : d2common + D2RVA::D2Common(0x6FD8B230),
            IsRoomInTown                        : d2common + D2RVA::D2Common(0x6FD8C390),
            GetHoradricStaffTombLevelId         : d2common + D2RVA::D2Common(0x6FD8B080),
            GetRoomFromAct                      : d2common + D2RVA::D2Common(0x6FD8B550),
            GetAdjacentRoomsListFromRoom        : d2common + D2RVA::D2Common(0x6FD8BA20),
            GetLevelIdFromRoom                  : d2common + D2RVA::D2Common(0x6FD8C000),
        },
    });
}
