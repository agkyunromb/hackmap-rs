use super::common::*;
use super::HackMap;
use windows_sys::Win32::System::Console::AllocConsole;
use D2Common::D2LevelId;
use D2Common::D2PresetUnit;
use D2Gfx::D2GfxData;
use D2Win::MsgHandler::{StormMsgHandler, StormMsgHandlerParams};

struct Stubs {
    Handle_D2GS_LOADCOMPLETE_04: Option<D2Client::Net::D2GSHandler>,
    D2Client_AutoMap_Init_CurrentAutoMapLayer: Option<extern "stdcall" fn()>,
    D2Client_AutoMap_DrawCells:
        Option<extern "fastcall" fn(cell: &D2AutoMapCellDataEx, arg2: usize)>,
    D2Client_LeaveGameCleanUp: Option<extern "stdcall" fn()>,
}

static mut STUBS: Stubs = Stubs {
    Handle_D2GS_LOADCOMPLETE_04: None,
    D2Client_AutoMap_Init_CurrentAutoMapLayer: None,
    D2Client_AutoMap_DrawCells: None,
    D2Client_LeaveGameCleanUp: None,
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
    pub data: D2Client::AutoMap::D2AutoMapCellData,
    pub cell_type: ExtraCellType,

    _pad: u32,
}

const _: () = assert!(0x2800 % size_of::<D2AutoMapCellDataEx>() == 0);

#[repr(C, packed(1))]
struct D2AutoMapCellBlockEx {
    pub Elements: [D2AutoMapCellDataEx; 0x2800 / std::mem::size_of::<D2AutoMapCellDataEx>()],
    pub NextBlock: *mut D2AutoMapCellBlockEx,
}

extern "fastcall" fn Handle_D2GS_LOADCOMPLETE_04(payload: *const u8) {
    get_stubs().Handle_D2GS_LOADCOMPLETE_04.unwrap()(payload);
    if D2Sigma::initialized() {
        D2Sigma::AutoMap::RevealMap();
    }

    reveal_map_ex();
}

extern "fastcall" fn D2Client_AutoMap_DrawCells(cell: &D2AutoMapCellDataEx, arg2: usize) {
    let D2Client_AutoMap_DrawCells = get_stubs().D2Client_AutoMap_DrawCells.unwrap();

    D2Client_AutoMap_DrawCells(cell, arg2);

    let layer = D2Client::AutoMap::CurrentAutoMapLayer().unwrap();

    let automap_cells_for_layers = HackMap::automap().automap_cells_for_layers();
    let cells = automap_cells_for_layers.get(&layer.nLayerNo);

    if let Some(cells) = cells {
        for cell in cells.iter() {
            D2Client_AutoMap_DrawCells(cell, arg2);
        }
    }
}

extern "stdcall" fn D2Client_LeaveGameCleanUp() {
    let hm = HackMap::get();

    hm.automap.automap_cells.clear();
    hm.image_loader.clear_cache();
    get_stubs().D2Client_LeaveGameCleanUp.unwrap()()
}

fn reveal_map_ex() -> Option<()> {
    let client_player = D2Client::Units::GetClientPlayer()?;
    let drlg_act = client_player.get_drlg_act();

    let drlg = D2Common::Dungeon::GetDrlgFromAct(drlg_act)?;
    let current_act = drlg_act.nAct;
    let max_level_id = D2Common::DataTbls::sgptDataTables().levels_txt_record_count();

    for level_id in 1..max_level_id {
        if D2Common::DrlgDrlg::GetActNoFromLevelId(level_id) != current_act {
            continue;
        }

        let level = match D2Common::DrlgDrlg::GetLevel(drlg, level_id) {
            Some(level) => level,
            None => continue,
        };

        reveal_level_ex(level);
    }

    None
}

fn reveal_level_ex(level: &mut D2Common::D2DrlgLevel) -> Option<()> {
    let mut drlg_room = ptr_to_ref_mut(level.pFirstRoomEx)?;

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
                    add_custom_automap_cell(drlg_room);
                },
            }
        }

        drlg_room = ptr_to_ref_mut(drlg_room.pDrlgRoomNext)?;
    }
}

fn add_custom_automap_cell(drlg_room: &mut D2Common::D2DrlgRoom) -> Option<()> {
    let mut preset_unit = D2Common::DrlgRoom::GetPresetUnits(drlg_room)?;

    let level_id = D2Common::DrlgRoom::GetLevelId(drlg_room);
    let level_def = D2Common::DataTbls::GetLevelDefRecord(level_id).unwrap();
    let layer_id = level_def.dwLayer;

    let levels_txt_record_count = D2Common::DataTbls::sgptDataTables().levels_txt_record_count();

    while let Some(unit) = ptr_to_ref_mut(preset_unit.pNext) {
        if process_preset_unit(unit, drlg_room, layer_id, levels_txt_record_count).is_none() {
            break;
        }
        preset_unit = unit;
    }

    Some(())
}

fn process_preset_unit(
    preset_unit: &mut D2PresetUnit,
    drlg_room: &D2Common::D2DrlgRoom,
    layer_id: u32,
    levels_txt_record_count: i32,
) -> Option<()> {
    let mut x = 0;
    let mut y = 0;
    let preset_unit_index = preset_unit.nIndex as u32;
    let mut cell_type: Option<ExtraCellType> = None;

    match preset_unit.nUnitType {
        D2UnitTypes::Monster => {
            if preset_unit_index == 256 {
                // A4 衣卒尔
                cell_type = Some(ExtraCellType::CellNo(300)); // 红色十字
            }
        }
        D2UnitTypes::Object => {
            match preset_unit_index {
                152 => {
                    // 塔拉夏古墓插杖的地方
                    cell_type = Some(ExtraCellType::CellNo(300)); // 红色十字
                }
                397 => {
                    // 黄金宝箱
                    cell_type = Some(ExtraCellType::CellNo(D2AutoMapCells::QChest as u32));
                }
                460 => {
                    cell_type = Some(ExtraCellType::CellNo(1468));
                }
                _ => {
                    let object_txt = D2Common::DataTbls::GetObjectsTxtRecord(preset_unit_index)?;

                    let cellno = object_txt.dwAutomap;

                    if cellno != 0 {
                        cell_type = Some(ExtraCellType::CellNo(cellno));
                    }
                }
            }
        }
        D2UnitTypes::Tile => {
            let mut room_tiles = ptr_to_ref_mut(drlg_room.pRoomTiles);

            while let Some(tiles) = room_tiles {
                if ptr_to_ref_mut(tiles.pLvlWarpTxtRecord).unwrap().dwLevelId == preset_unit_index {
                    x = 8;
                    y = -0x15;

                    let level_id = D2Common::DrlgRoom::GetLevelId(tiles.pDrlgRoom);

                    if level_id >= levels_txt_record_count {
                        break;
                    }

                    cell_type = Some(ExtraCellType::LevelId(level_id));
                }

                room_tiles = ptr_to_ref_mut(tiles.pNext);
            }
        }
        _ => {}
    }

    let cell_type = cell_type?;

    let (x1, y1) = calculate_positions(drlg_room, preset_unit);
    let (x2, y2) = calculate_pixel_positions(x1, y1);

    let mut cell = D2AutoMapCellDataEx {
        data: D2Client::AutoMap::D2AutoMapCellData {
            fSaved: 0,
            nCellNo: 0,
            xPixel: 0,
            yPixel: 0,
            wWeight: 0,
            pPrev: null_mut(),
            pNext: null_mut(),
        },
        cell_type: ExtraCellType::None,
        _pad: 0,
    };

    match cell_type {
        ExtraCellType::CellNo(cellno) => {
            cell.cell_type = ExtraCellType::None;
            cell.data.nCellNo = cellno as u16;
            cell.data.xPixel = (x2 + x) as u16;
            cell.data.yPixel = (y2 + y) as u16;
        }
        ExtraCellType::LevelId(_) => {
            cell.cell_type = cell_type;
            cell.data.nCellNo = 0;
            cell.data.xPixel = (x2 + x) as u16;
            cell.data.yPixel = (y2 + y) as u16;
        }
        _ => {}
    }

    HackMap::automap()
        .automap_cells
        .entry(layer_id)
        .or_default()
        .push(cell);

    Some(())
}

fn calculate_positions(drlg_room: &D2Common::D2DrlgRoom, preset_unit: &D2PresetUnit) -> (i32, i32) {
    let x1 = drlg_room.nTileXPos * 5 + preset_unit.nXpos;
    let y1 = drlg_room.nTileYPos * 5 + preset_unit.nYpos;
    (x1, y1)
}

fn calculate_pixel_positions(x1: i32, y1: i32) -> (i32, i32) {
    let x2 = ((x1 - y1) * 16) / 10 + 1;
    let y2 = ((x1 + y1) * 8) / 10 - 3;
    (x2, y2)
}

fn new_automap_cell(
    g_cell_block_head: *mut *mut D2AutoMapCellBlockEx,
    g_automap_cell_count: &mut usize,
) -> *mut D2AutoMapCellDataEx {
    unsafe {
        let element_count = &(**g_cell_block_head).Elements.len();
        let block_count = *g_automap_cell_count / element_count;
        let block_index = *g_automap_cell_count % element_count;

        *g_automap_cell_count += 1;

        let mut head: *mut D2AutoMapCellBlockEx = *g_cell_block_head;
        let mut prev: *mut D2AutoMapCellBlockEx = null_mut();

        for _ in 0..block_count {
            prev = head;
            head = (*head).NextBlock;
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
                (*prev).NextBlock = head;
            }
        }

        let cell = &mut (*head).Elements[block_index];
        std::ptr::write_bytes(cell, 0, 1);

        cell.cell_type = ExtraCellType::None;

        cell
    }
}

extern "stdcall" fn NewAutoMapCell() -> *mut D2AutoMapCellDataEx {
    new_automap_cell(
        D2Client::AutoMap::AutoMapCellBlockHead() as *mut *mut D2AutoMapCellBlockEx,
        D2Client::AutoMap::AutoMapCellCount(),
    )
}

extern "stdcall" fn CelDrawClipped(
    data: &D2GfxData,
    x: i32,
    y: i32,
    crop_rect: PVOID,
    draw_mode: D2DrawMode,
) {
    let cell: &mut D2AutoMapCellDataEx = read_at(data as *const D2GfxData as usize - 0x20);

    if !HackMap::automap().draw_automap_cell(cell, x, y) {
        D2Gfx::Texture::CelDrawClipped(data, x, y, crop_rect, draw_mode)
    }
}

pub(super) struct AutoMap {
    automap_cells: HashMap<u32, Vec<D2AutoMapCellDataEx>>,
}

impl AutoMap {
    pub fn new() -> Self {
        Self {
            automap_cells: HashMap::new(),
        }
    }

    pub fn automap_cells_for_layers(&mut self) -> &mut HashMap<u32, Vec<D2AutoMapCellDataEx>> {
        &mut self.automap_cells
    }

    fn draw_automap_cell(&self, cell: &mut D2AutoMapCellDataEx, x: i32, y: i32) -> bool {
        match cell.cell_type {
            ExtraCellType::LevelId(level_id) => {
                self.draw_tile_name(level_id, x, y);

                return true;
            }

            ExtraCellType::ShrineType(_shrine_type) => {}
            _ => {}
        }

        false
    }

    fn draw_tile_name(&self, level_id: D2LevelId, x: i32, y: i32) {
        let data_tables = D2Common::DataTbls::sgptDataTables();
        let level_txt = data_tables.get_levels_txt_record(level_id).unwrap();
        let level_name = level_txt.get_level_name_ptr();

        let player = D2Client::Units::GetClientPlayer().unwrap();
        let drlg_act = ptr_to_ref_mut(player.pDrlgAct).unwrap();
        let color = if D2Common::Dungeon::GetHoradricStaffTombLevelId(drlg_act) == level_id {
            D2StringColorCodes::LightGreen
        } else {
            D2StringColorCodes::White
        };

        D2WinEx::Text::draw_text(level_name, x, y, D2Font::Font6, color);
    }

    pub fn init(&mut self, modules: &D2Modules) -> Result<(), HookError> {
        // unsafe {
        //     std::env::set_var("RUST_BACKTRACE", "1");
        //     ::windows_sys::Win32::System::Console::AllocConsole();
        // }

        D2ClientEx::Game::on_leave_game(|| {
            HackMap::automap().automap_cells_for_layers().clear();
        });

        unsafe {
            STUBS.Handle_D2GS_LOADCOMPLETE_04 = Some(D2Client::Net::SwapD2GSHandler(
                0x04,
                Handle_D2GS_LOADCOMPLETE_04,
            ));

            inline_hook_call(
                modules.D2Client.unwrap(),
                D2RVA::D2Client(0x6FB10E34),
                D2Client_AutoMap_DrawCells as usize,
                Some(&mut STUBS.D2Client_AutoMap_DrawCells),
                None,
            )?;
            inline_hook_jmp::<()>(
                0,
                D2Client::AddressTable.AutoMap.NewAutoMapCell,
                NewAutoMapCell as usize,
                None,
                None,
            )?;
            inline_hook_call::<()>(
                0,
                D2Client::AddressTable.AutoMap.CallDrawAutoMapCell,
                CelDrawClipped as usize,
                None,
                None,
            )?;
        }

        Ok(())
    }
}
