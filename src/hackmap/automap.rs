use super::common::*;
use super::HackMap;
use windows_sys::Win32::System::Console::AllocConsole;
use D2Common::DrlgRoom::D2LevelId;
use D2Gfx::Texture::D2GfxData;
use D2Win::MsgHandler::{StormMsgHandler, StormMsgHandlerParams};

struct Stubs {
    Handle_D2GS_LOADCOMPLETE_04                 : Option<D2Client::Net::D2GSHandler>,
    D2Client_AutoMap_Init_CurrentAutoMapLayer   : Option<extern "stdcall" fn()>,
    D2Client_AutoMap_DrawCells                  : Option<extern "fastcall" fn(cell: &D2AutoMapCellDataEx, arg2: usize)>,
    D2Client_LeaveGameCleanUp                   : Option<extern "stdcall" fn()>,
}

static mut STUBS: Stubs = Stubs{
    Handle_D2GS_LOADCOMPLETE_04                 : None,
    D2Client_AutoMap_Init_CurrentAutoMapLayer   : None,
    D2Client_AutoMap_DrawCells                  : None,
    D2Client_LeaveGameCleanUp                   : None,
};

#[allow(static_mut_refs)]
fn get_stubs() -> &'static Stubs {
    unsafe { &STUBS }
}

pub enum ExtraCellType {
    None,
    CellNo(u32),
    LevelId(D2LevelId),
    ShrineType(u32),
}

#[repr(C, packed(1))]
pub(super) struct D2AutoMapCellDataEx {
    pub data        : D2Client::AutoMap::D2AutoMapCellData,
    pub cell_type   : ExtraCellType,

    _pad            : u32,
}

const _: () = assert!(0x2800 % size_of::<D2AutoMapCellDataEx>() == 0);

#[repr(C, packed(1))]
struct D2AutoMapCellBlockEx {
    pub Elements    : [D2AutoMapCellDataEx; 0x2800 / std::mem::size_of::<D2AutoMapCellDataEx>()],
    pub NextBlock   : *mut D2AutoMapCellBlockEx,
}

extern "fastcall" fn Handle_D2GS_LOADCOMPLETE_04(payload: *const u8) {
    get_stubs().Handle_D2GS_LOADCOMPLETE_04.unwrap()(payload);
    if D2Sigma::initialized() {
        D2Sigma::AutoMap::RevealMap();
    }

    let _ = reveal_map_ex();
}

extern "fastcall" fn D2Client_AutoMap_DrawCells(cell: &D2AutoMapCellDataEx, arg2: usize) {
    let D2Client_AutoMap_DrawCells = get_stubs().D2Client_AutoMap_DrawCells.unwrap();

    D2Client_AutoMap_DrawCells(cell, arg2);

    let layer = D2Client::AutoMap::CurrentAutoMapLayer().unwrap();

    let automap_cells_for_layers = HackMap::automap().automap_cells_for_layers.as_ref().unwrap();
    let cells = automap_cells_for_layers.get(&layer.nLayerNo);

    if let Some(cells) = cells {
        for cell in cells.iter() {
            D2Client_AutoMap_DrawCells(cell, arg2);
        }
    }
}

extern "stdcall" fn D2Client_LeaveGameCleanUp() {
    HackMap::automap().automap_cells_for_layers.as_mut().unwrap().clear();
    get_stubs().D2Client_LeaveGameCleanUp.unwrap()()
}

fn reveal_map_ex() -> Result<(), ()> {
    let client_player   = D2Client::Units::GetClientPlayer().ok_or(())?;
    let drlg_act        = client_player.get_drlg_act();

    let drlg            = D2Common::Dungeon::GetDrlgFromAct(drlg_act).ok_or(())?;
    let current_act     = drlg_act.nAct;
    let max_level_id    = D2Common::DataTbls::sgptDataTables().levels_txt_record_count();

    for level_id in 1..max_level_id {
        if D2Common::DrlgDrlg::GetActNoFromLevelId(level_id) != current_act {
            continue;
        }

        let level = match D2Common::DrlgDrlg::GetLevel(drlg, level_id) {
            Some(level) => level,
            None => continue,
        };

        let _ = reveal_level_ex(level);
    }

    Ok(())
}

fn reveal_level_ex(level: &mut D2Common::DrlgDrlg::D2DrlgLevel) -> Result<(), ()> {
    let mut drlg_room = ptr_to_ref_mut(level.pFirstRoomEx).ok_or(())?;

    loop {
        if drlg_room.nType == D2DrlgTypes::Preset {
            match D2Common::DrlgPreset::GetLevelPrestIdFromRoomEx(drlg_room) {
                38  |   // Act 1 - Swamp Fill 1
                39  |   // Act 1 - Swamp Fill 2
                401 |   // Act 2 - Desert Fill Bone 1
                402 |   // Act 2 - Desert Fill Bone 2
                403 |   // Act 2 - Desert Fill Wagon 1
                404 |   // Act 2 - Desert Fill Berms 1
                405 |   // Act 2 - Desert Fill Berms 2
                836 |   // Act 4 - Lava X
                863     // Act 5 - Town
                => {},

                _ => {
                    let _ = add_custom_automap_cell(drlg_room);
                },
            }
        }

        drlg_room = ptr_to_ref_mut(drlg_room.pDrlgRoomNext).ok_or(())?;
    }
}

fn add_custom_automap_cell(drlg_room: &mut D2Common::DrlgDrlg::D2DrlgRoom) -> Result<(), ()>  {
    let mut preset_unit = D2Common::DrlgRoom::GetPresetUnits(drlg_room).ok_or(())?;

    let level_id = D2Common::DrlgRoom::GetLevelId(drlg_room);
    let level_def = D2Common::DataTbls::GetLevelDefRecord(level_id).unwrap();
    let layer_id = level_def.dwLayer;

    let levels_txt_record_count = D2Common::DataTbls::sgptDataTables().levels_txt_record_count();

    loop {
        loop {
            let mut x                                   = 0;
            let mut y                                   = 0;
            let preset_unit_index                       = preset_unit.nIndex;
            let mut cell_type: Option<ExtraCellType>    = None;

            match preset_unit.nUnitType {
                D2UnitTypes::Monster => {
                    if preset_unit_index == 256 {
                        // A4 衣卒尔
                        cell_type = Some(ExtraCellType::CellNo(300));   // 红色十字
                    }
                },
                D2UnitTypes::Object => {
                    match preset_unit_index {
                        152 => {
                            // 塔拉夏古墓插杖的地方
                            // orifice, Where you place the Horadric staff

                            cell_type = Some(ExtraCellType::CellNo(300));   // 红色十字
                        },

                        397 => {
                            // 黄金宝箱
                            // sparkly chest
                            cell_type = Some(ExtraCellType::CellNo(D2AutoMapCells::QChest as u32));
                        },

                        460 => {
                            cell_type = Some(ExtraCellType::CellNo(1468));
                        },

                        _ => {
                            let object_txt = match D2Common::DataTbls::GetObjectsTxtRecord(preset_unit_index) {
                                Some(txt) => txt,
                                None => break,
                            };

                            let cellno = object_txt.dwAutomap;

                            if cellno != 0 {
                                cell_type = Some(ExtraCellType::CellNo(cellno));
                            }
                        },
                    }
                },

                D2UnitTypes::Tile => {
                    let mut room_tiles = ptr_to_ref_mut(drlg_room.pRoomTiles);

                    while let Some(tiles) = room_tiles {
                        if ptr_to_ref_mut(tiles.pLvlWarpTxtRecord).unwrap().dwLevelId == preset_unit_index as u32 {
                            x = 8;
                            y = -0x15;

                            let level_id = D2Common::DrlgRoom::GetLevelId(tiles.pDrlgRoom);

                            if level_id >= levels_txt_record_count {
                                break;
                            }

                            // let data_tables = D2Common::DataTbls::sgptDataTables();
                            // let level_txt = data_tables.get_levels_txt_record(level_id).unwrap();
                            // let level_name = level_txt.get_level_name();

                            cell_type = Some(ExtraCellType::LevelId(level_id));
                        }

                        room_tiles = ptr_to_ref_mut(tiles.pNext);
                    }
                },
                _ => {},
            }

            let cell_type = match cell_type {
                Some(t) => t,
                None => break,
            };

            let x1 = drlg_room.nTileXPos * 5 + preset_unit.nXpos;
            let y1 = drlg_room.nTileYPos * 5 + preset_unit.nYpos;

            let x2 = ((x1 - y1) * 16) / 10 + 1;
            let y2 = ((x1 + y1) * 8) / 10 - 3;

            let mut cell = D2AutoMapCellDataEx{
                data        : D2Client::AutoMap::D2AutoMapCellData { fSaved: 0, nCellNo: 0, xPixel: 0, yPixel: 0, wWeight: 0, pPrev: null_mut(), pNext: null_mut() },
                cell_type   : ExtraCellType::None,
                _pad        : 0,
            };

            match cell_type {
                ExtraCellType::CellNo(cellno) => {
                    cell.cell_type = ExtraCellType::None;
                    cell.data.nCellNo = cellno as u16;
                    cell.data.xPixel = (x2 + x) as u16;
                    cell.data.yPixel = (y2 + y) as u16;
                },

                ExtraCellType::LevelId(_) => {
                    cell.cell_type = cell_type;
                    cell.data.nCellNo = 0;
                    cell.data.xPixel = (x2 + x) as u16;
                    cell.data.yPixel = (y2 + y) as u16;
                },

                _ => {},
            }

            // let cell2 = ptr_to_ref_mut(NewAutomapCell()).unwrap();
            // *cell2 = cell;

            HackMap::automap().automap_cells_for_layers.as_mut().unwrap().entry(layer_id).or_insert(vec![]).push(cell);

            // let layer = D2Client::AutoMap::CurrentAutoMapLayer().unwrap();
            // D2Client::AutoMap::AddAutomapCell(&cell2.data, &mut layer.pObjects);

            break;
        }

        preset_unit = ptr_to_ref_mut(preset_unit.pNext).ok_or(())?;
    }
}

fn new_automap_cell(g_cell_block_head: *mut *mut D2AutoMapCellBlockEx, g_automap_cell_count: &mut usize) -> *mut D2AutoMapCellDataEx {
    unsafe {
        let element_count = &(**g_cell_block_head).Elements.len();
        let block_count = *g_automap_cell_count / element_count;
        let block_index = *g_automap_cell_count % element_count;

        *g_automap_cell_count += 1;

        let mut head: *mut D2AutoMapCellBlockEx = *g_cell_block_head;
        let mut prev: *mut D2AutoMapCellBlockEx = null_mut();

        for _ in 0..block_count {
            prev = head;
            head = (&*head).NextBlock;
            if head.is_null() {
                break;
            }
        }

        if head.is_null() {
            head = Fog::Alloc(std::mem::size_of_val(&*head));
            std::ptr::write_bytes(head, 0, 1);
            if prev.is_null() {
                *g_cell_block_head = head;
            } else {
                (&mut *prev).NextBlock = head;
            }
        }

        let cell = &mut (&mut *head).Elements[block_index];
        std::ptr::write_bytes(cell, 0, 1);

        cell.cell_type = ExtraCellType::None;

        cell
    }
}

extern "stdcall" fn NewAutomapCell() -> *mut D2AutoMapCellDataEx {
    new_automap_cell(D2Client::AutoMap::AutoMapCellBlockHead() as *mut *mut D2AutoMapCellBlockEx, D2Client::AutoMap::AutoMapCellCount())
}

extern "stdcall" fn CelDrawClipped(data: &D2GfxData, x: i32, y: i32, crop_rect: PVOID, draw_mode: D2DrawMode) {
    let cell: &mut D2AutoMapCellDataEx = read_at(data as *const D2GfxData as usize - 0x20);

    if HackMap::automap().draw_automap_cell(cell, x, y) == false {
        D2Gfx::Texture::CelDrawClipped(data, x, y, crop_rect, draw_mode)
    }
}

pub(super) struct AutoMap {
    automap_cells_for_layers: Option<HashMap<u32, Vec<D2AutoMapCellDataEx>>>,
}

impl AutoMap {
    pub const fn new() -> Self {
        Self {
            automap_cells_for_layers: None,
        }
    }

    fn draw_automap_cell(&self, cell: &mut D2AutoMapCellDataEx, x: i32, y: i32) -> bool {
        match cell.cell_type {
            ExtraCellType::LevelId(level_id) => {
                self.draw_tile_name(level_id, x, y);

                return true;
            },

            ExtraCellType::ShrineType(_shrine_type) => {},
            _ => {},
        }

        false
    }

    fn draw_tile_name(&self, level_id: D2LevelId, x: i32, y: i32) {
        let data_tables = D2Common::DataTbls::sgptDataTables();
        let level_txt = data_tables.get_levels_txt_record(level_id).unwrap();
        let level_name = level_txt.get_level_name_ptr();

        let player = D2Client::Units::GetClientPlayer().unwrap();
        let drlg_act = ptr_to_ref_mut(player.pDrlgAct).unwrap();
        let color = if D2Common::Dungeon::GetHoradricStaffTombLevelId(drlg_act) == level_id { D2StringColorCodes::LightGreen } else { D2StringColorCodes::White };

        D2WinEx::Text::draw_text(level_name, x, y, D2Font::Font6, color);
    }
}

pub fn init(modules: &D2Modules) -> Result<(), HookError> {
    // unsafe {
    //     std::env::set_var("RUST_BACKTRACE", "1");
    //     ::windows_sys::Win32::System::Console::AllocConsole();
    // }

    HackMap::automap().automap_cells_for_layers = Some(HashMap::new());

    unsafe {
        STUBS.Handle_D2GS_LOADCOMPLETE_04 = Some(D2Client::Net::SwapD2GSHandler(0x04, Handle_D2GS_LOADCOMPLETE_04));

        // patch_memory_value(_modules.D2Client.unwrap(), D2RVA::D2Client(0x6FB11D32), 0x80, 1)?;
        // inline_hook_jmp(_modules.D2Client.unwrap(), D2RVA::D2Client(0x6FB12AF0), D2Client_AutoMap_Init_CurrentAutoMapLayer as usize, Some(&mut STUBS.D2Client_AutoMap_Init_CurrentAutoMapLayer), None)?;

        inline_hook_call(modules.D2Client.unwrap(), D2RVA::D2Client(0x6FAF515D), D2Client_LeaveGameCleanUp as usize, Some(&mut STUBS.D2Client_LeaveGameCleanUp), None)?;
        inline_hook_call(modules.D2Client.unwrap(), D2RVA::D2Client(0x6FB10E34), D2Client_AutoMap_DrawCells as usize, Some(&mut STUBS.D2Client_AutoMap_DrawCells), None)?;
        inline_hook_jmp::<()>(0, D2Client::AddressTable.AutoMap.NewAutomapCell, NewAutomapCell as usize, None, None)?;
        inline_hook_call::<()>(0, D2Client::AddressTable.AutoMap.CallDrawAutoMapCell, CelDrawClipped as usize, None, None)?;
    }

    Ok(())
}
