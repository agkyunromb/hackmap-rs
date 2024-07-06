use super::common::*;
use super::config::{DropNotify, PickupMethod, ConfigRef};
use std::alloc::System;
use std::ops::Add;
use std::time::{SystemTime, Duration};
use super::image_loader;
use super::HackMap;
use super::item_state_monitor::*;
use D2Client::D2ClientOffset;
use D2Common::D2Unit;

const MINIMAP_COLOR_DEFAULT: u8 = 0xFF;
const MINIMAP_COLOR_HIDE: u8    = 0xFE;

struct Stubs {
    ShouldShowUnit: Option<extern "stdcall" fn() -> BOOL>,
    DATATBLS_CompileTxt: Option<extern "stdcall" fn(PVOID, PCSTR, PVOID, &mut i32, usize) -> PVOID>,
    D2Sigma_Items_GetItemName: Option<extern "stdcall" fn(&D2Unit, PWSTR, u32)>,
}

static mut STUBS: Stubs = Stubs{
    ShouldShowUnit              : None,
    DATATBLS_CompileTxt         : None,
    D2Sigma_Items_GetItemName   : None,
};

#[allow(static_mut_refs)]
fn get_stubs() -> &'static Stubs {
    unsafe { &STUBS }
}

extern "stdcall" fn DATATBLS_CompileTxt(archive: PVOID, name: PCSTR, tbl: PVOID, recordCount: &mut i32, recordSize: usize) -> PVOID {
    let data = get_stubs().DATATBLS_CompileTxt.unwrap()(archive, name, tbl, recordCount, recordSize);

    if data.is_null() == false && name.to_str() == "Monstats3" {
        HackMap::unit_color().init_automap_monster_colors(data, *recordCount as usize, recordSize);
    }

    data
}

extern "stdcall" fn d2sigma_automap_draw() {
    if D2Client::UI::GetUIVar(D2UIvars::EscMenu) != 0 || D2Client::UI::GetUIVar(D2UIvars::Config) != 0 {
        return;
    }

    if D2Client::UI::GetUIOpenMode() == 3 {
        return;
    }

    D2Client::AutoMap::DrawAutoMapCells();
    HackMap::unit_color().draw_automap_units();
}

extern "stdcall" fn d2sigma_items_get_item_name(item: &D2Unit, buffer: PWSTR, arg3: u32) {
    get_stubs().D2Sigma_Items_GetItemName.unwrap()(item, buffer, arg3);

    if D2SigmaEx::Items::is_getting_item_properties() {
        return;
    }

    HackMap::unit_color().d2sigma_items_get_item_name(item, buffer);
}

extern "fastcall" fn should_show_unit(unit: &mut D2Unit) -> bool {
    HackMap::unit_color().should_show_unit(unit)
}

fn get_stub_should_show_unit() -> usize {
    get_stubs().ShouldShowUnit.unwrap() as usize
}

global_asm!(
    r#"
.global _naked_should_show_unit
_naked_should_show_unit:

    mov     ecx, esi
    call    {should_show_unit}
    test    al, al
    jz      _naked_should_show_unit_hide_unit

    call    {get_stub_should_show_unit}
    call    eax

_naked_should_show_unit_hide_unit:
    ret
"#,
    should_show_unit            = sym should_show_unit,
    get_stub_should_show_unit   = sym get_stub_should_show_unit,
);

extern "C" {
    fn naked_should_show_unit() -> BOOL;
}

pub(super) struct UnitColor {
    pub cfg                 : ConfigRef,
    pub boss_monster_id     : HashMap<u32, u32>,
    pub glide3x_is_d2sigma  : *mut u8,
    pub items_to_cube       : HashMap<u32, SystemTime>,
}

impl UnitColor {
    pub fn new(cfg: ConfigRef) -> Self{
        Self{
            cfg,
            boss_monster_id     : HashMap::new(),
            glide3x_is_d2sigma  : null_mut(),
            items_to_cube       : HashMap::new(),
        }
    }

    fn init_automap_monster_colors(&mut self, data: PVOID, recordCount: usize, recordSize: usize) {
        let mut mon_stats_3 = unsafe { std::slice::from_raw_parts_mut(data as *mut u8, recordSize * recordCount) };

        // let data_tables = D2Common::DataTbls::sgptDataTables();
        // let mon_stats_txt = data_tables.mon_stats_txt();
        let mon_stats_txt_record_count = D2Common::DataTbls::sgptDataTables().mon_stats_txt_record_count();

        for i in 0..mon_stats_txt_record_count {
            // let mon_stats_flags = mon_stats_txt[i].dwMonStatsFlags;

            if mon_stats_3[0] & 0x01 != 0 || mon_stats_3[1] & 0x02 != 0 {
                self.boss_monster_id.insert(i as u32, 1);
            } else if mon_stats_3[0] & 4 != 0 {
                self.boss_monster_id.insert(i as u32, 1);
            }

            mon_stats_3 = &mut mon_stats_3[recordSize..];
        }
    }

    fn draw_automap_units(&self) -> Option<()> {
        let player = D2Client::Units::GetClientPlayer()?;
        let room = D2Common::Units::GetRoom(player)?;

        let adjacent_rooms = D2Common::Dungeon::GetAdjacentRoomsListFromRoom(room)?;

        for &room in adjacent_rooms.iter() {
            if room.is_null() {
                continue;
            }

            let room = ptr_to_ref_mut(room).unwrap();

            let mut unit_opt = ptr_to_ref_mut(room.pUnitFirst);

            while let Some(unit) = unit_opt {
                self.draw_unit(unit);
                unit_opt = ptr_to_ref_mut(unit.pRoomNext);
            }
        }

        None
    }

    fn draw_unit(&self, unit: &mut D2Unit) {
        let mut x = D2Common::Units::GetClientCoordX(unit);
        let mut y = D2Common::Units::GetClientCoordY(unit);

        let divisor = *D2Client::AutoMap::PointDivisor();
        let rect = D2Client::AutoMap::Rect();

        x = x / divisor - *D2Client::AutoMap::PointOffsetX() + 8;
        y = y / divisor - *D2Client::AutoMap::PointOffsetY() - 8;

        if D2Client::AutoMap::IsMiniMapOn() != FALSE {
            x -= 1;
            y += 5;
        }

        if x < rect.left || x > rect.right || y < rect.top || y > rect.bottom {
            return;
        }

        match unit.dwUnitType {
            D2UnitTypes::Player => {
                self.draw_player(unit, x, y);
            },
            D2UnitTypes::Monster => {
                self.draw_monster(unit, x, y);
            },
            D2UnitTypes::Object => {
                self.draw_object(unit, x, y);
            },

            D2UnitTypes::Missile => {},
            D2UnitTypes::Item => {
                self.draw_item(unit, x, y);
            },

            _ => {},
        }
    }

    fn draw_player(&self, unit: &mut D2Unit, x: i32, y: i32) -> Option<()> {
        let unit_color_config = &self.cfg.borrow().unit_color;
        let player = D2Client::Units::GetClientPlayer()?;
        let color = if player.dwUnitId == unit.dwUnitId { unit_color_config.my_blob_color } else { unit_color_config.party_blob_color };

        self.draw_cell_by_blob_file(x, y, unit_color_config.my_blob_file.as_ref(), color);
        // self.draw_default_cross(x, y, if player.dwUnitId == unit.dwUnitId { 0x97 } else { 0x81 });

        None
    }

    fn draw_monster(&self, unit: &mut D2Unit, x: i32, y: i32) -> Option<()> {
        let class_Id = unit.dwClassId;

        match class_Id {
            179 => return None,   // A1 红牛
            _ => {},
        }

        if D2Client::Units::IsCorpse(unit) {
            return None;
        }

        let unit_color_cfg    = &self.cfg.borrow().unit_color;
        let data_tables       = D2Common::DataTbls::sgptDataTables();
        let monster_data      = unit.get_monster_data()?;
        let mon_stats_txt     = monster_data.get_mon_stats_txt()?;
        let mon_stats_flags   = mon_stats_txt.dwMonStatsFlags;

        if mon_stats_flags & (D2MonStatsTxtFlags::Npc | D2MonStatsTxtFlags::Interact) == (D2MonStatsTxtFlags::Npc | D2MonStatsTxtFlags::Interact) {
            D2WinEx::Text::draw_text(D2Client::Units::GetName(unit), x, y - 8, D2Font::Font6, D2StringColorCodes::DarkGold);
            self.draw_cell_by_blob_file(x, y, unit_color_cfg.npc_blob_file.as_ref(), 0xFF);
            return None;
        }

        if mon_stats_flags.contains(D2MonStatsTxtFlags::InTown | D2MonStatsTxtFlags::Npc) {
            return None;
        }

        let room = D2Common::Units::GetRoom(unit)?;
        let level_txt = data_tables.get_levels_txt_record(D2Common::Dungeon::GetLevelIdFromRoom(room))?;

        for cmon in level_txt.wCMon {
            if cmon == class_Id as u16 {
                return None;
            }
        }

        if D2Client::Units::GetMonsterOwnerID(unit) != u32::MAX {
            return None;
        }

        // println!("class_id: {}", class_Id);

        let type_flag = monster_data.nTypeFlag;
        let mut color: u8;
        let mut show_name = false;
        let mut blob_file: Option<&String>;

        if type_flag.contains(D2MonTypeFlags::SuperUnique) {
            color = unit_color_cfg.super_unique_color;
            blob_file = unit_color_cfg.boss_blob_file.as_ref();
            show_name = true;

        } else if type_flag.contains(D2MonTypeFlags::Unique) {
            color = unit_color_cfg.boss_monster_color;
            blob_file = unit_color_cfg.boss_blob_file.as_ref();

        } else if type_flag.contains(D2MonTypeFlags::Champion) {
            color = unit_color_cfg.champion_monster_color;
            blob_file = unit_color_cfg.monster_blob_file.as_ref();

        } else if type_flag.contains(D2MonTypeFlags::Minion) {
            color = unit_color_cfg.minion_monster_color;
            blob_file = unit_color_cfg.monster_blob_file.as_ref();

        } else if self.boss_monster_id.contains_key(&class_Id) {
            color = unit_color_cfg.super_unique_color;
            blob_file = unit_color_cfg.boss_blob_file.as_ref();
            show_name = true;

        } else if mon_stats_flags.contains(D2MonStatsTxtFlags::Boss) {
            color = unit_color_cfg.champion_monster_color;
            blob_file = unit_color_cfg.monster_blob_file.as_ref();

        } else {
            color = unit_color_cfg.normal_monster_color;
            blob_file = unit_color_cfg.monster_blob_file.as_ref();
        }

        if let Some(c) = unit_color_cfg.monster_color.get(&class_Id) {
            match *c {
                MINIMAP_COLOR_DEFAULT => {
                    return None;
                },

                MINIMAP_COLOR_HIDE => {
                    color = unit_color_cfg.super_unique_color;
                    blob_file = unit_color_cfg.boss_blob_file.as_ref();
                    show_name = true;
                }

                _ => {
                    color = *c;
                },
            }
        }

        self.draw_cell_by_blob_file(x, y, blob_file, color);

        let mut desc = format!("ÿc1");

        if show_name || (type_flag & D2MonTypeFlags::SuperUnique == D2MonTypeFlags::SuperUnique) {
            desc += &format!("ÿc1{}", D2Client::Units::GetName(unit).to_string());

        } else if type_flag == D2MonTypeFlags::Unique && mon_stats_txt.dwMonStatsFlags.contains(D2MonStatsTxtFlags::Boss) && monster_data.wBossHcIdx == 0 {
            desc += &format!("ÿc1{}", D2Client::Units::GetName(unit).to_string());
        }

        if type_flag.contains(D2MonTypeFlags::Unique) {
            let empty_str = String::new();
            for umod in monster_data.nMonUmod {
                match umod {
                    D2MonUMods::None => break,
                    D2MonUMods::MagicResistant  => desc += unit_color_cfg.magic_resistant_desc.as_ref().unwrap_or(&empty_str),
                    D2MonUMods::FireChant       => desc += unit_color_cfg.fire_enchanted_desc.as_ref().unwrap_or(&empty_str),
                    D2MonUMods::LightChant      => desc += unit_color_cfg.lightning_enchanted_desc.as_ref().unwrap_or(&empty_str),
                    D2MonUMods::ColdChant       => desc += unit_color_cfg.cold_enchanted_desc.as_ref().unwrap_or(&empty_str),
                    D2MonUMods::ManaBurn        => desc += unit_color_cfg.mana_burn_desc.as_ref().unwrap_or(&empty_str),
                    _ => {},
                }
            }
        }

        for (stat_id, stat_desc) in [
            (D2ItemStats::DamageResist, &unit_color_cfg.physical_immunity_desc),
            (D2ItemStats::MagicResist,  &unit_color_cfg.magic_immunity_desc),
            (D2ItemStats::FireResist,   &unit_color_cfg.fire_immunity_desc),
            (D2ItemStats::LightResist,  &unit_color_cfg.lightning_immunity_desc),
            (D2ItemStats::ColdResist,   &unit_color_cfg.cold_immunity_desc),
            (D2ItemStats::PoisonResist, &unit_color_cfg.poison_immunity_desc),
        ] {
            let stat_desc = match stat_desc {
                None => continue,
                Some(stat_desc) => stat_desc,
            };

            if stat_desc.is_empty() {
                continue;
            }

            if (D2Common::StatList::GetUnitBaseStat(unit, stat_id, 0) as i32) < 100 {
                continue;
            }

            desc += stat_desc;
        }

        if desc.is_empty() == false {
            D2WinEx::Text::draw_text(desc.to_utf16().as_ptr(), x, y - 10, D2Font::Font16, D2StringColorCodes::White);
        }

        None
    }

    fn draw_object(&self, unit: &mut D2Unit, x: i32, y: i32) -> Option<()> {
        let object_txt = D2Common::DataTbls::GetObjectsTxtRecord(unit.dwClassId)?;

        if object_txt.nSubClass.contains(D2ObjectSubClasses::TownPortal) {
            self.draw_default_cross(x, y, 0x6D);
        }

        None
    }

    fn draw_item(&self, unit: &mut D2Unit, x: i32, y: i32) -> Option<()> {
        let cfg = self.cfg.borrow();
        let item_color = cfg.unit_color.get_color_from_unit(unit)?;
        let minimap_color = item_color.minimap_color?;

        if minimap_color == MINIMAP_COLOR_DEFAULT || minimap_color == MINIMAP_COLOR_HIDE {
            return None;
        }

        self.draw_cell_by_blob_file(x, y, cfg.unit_color.item_blob_file.as_ref(), minimap_color);

        None
    }

    fn draw_cell_by_blob_file(&self, x: i32, y: i32, blob_file: Option<&String>, color: u8) {
        match blob_file {
            None => self.draw_default_cross(x, y, color),
            Some(blob) => {
                let loader = HackMap::image_loader();

                match loader.load_image(blob) {
                    None => self.draw_default_cross(x, y, color),
                    Some(cell) => {
                        self.draw_cell(x, y, cell, color);
                    },
                }
            },
        }
    }

    fn draw_cell(&self, x: i32, y: i32, cell: image_loader::DC6BufferRef, color: u8) {
        if self.glide3x_is_d2sigma.is_null() == false {
            unsafe { self.glide3x_is_d2sigma.write(0); }
        }

        D2GfxEx::Texture::draw_dell(x, y, cell.d2_cell_file_header(), color);

        if self.glide3x_is_d2sigma.is_null() == false {
            unsafe { self.glide3x_is_d2sigma.write(1); }
        }
    }

    fn draw_default_cross(&self, x: i32, y: i32, color: u8) {
        static DefaultUnitShape: &[[i32; 2]] = &[
            [ 0, -2],
            [ 4, -4],
            [ 8, -2],
            [ 4,  0],
            [ 8,  2],
            [ 4,  4],
            [ 0,  2],
            [-4,  4],
            [-8,  2],
            [-4,  0],
            [-8, -2],
            [-4, -4],
            [ 0, -2],
        ];

        for i in 0..DefaultUnitShape.len() - 1 {
            let pt = DefaultUnitShape[i];
            let pt2 = DefaultUnitShape[i + 1];

            D2Gfx::Draw::DrawLine(x + pt[0], y + pt[1], x + pt2[0], y + pt2[1], color, 0xFF)
        }
    }

    fn d2sigma_items_get_item_name(&self, item: &D2Unit, buffer: PWSTR) {
        let mut name = buffer.to_string();

        let socks_num = D2Common::StatList::GetUnitBaseStat(item, D2ItemStats::Item_NumSockets, 0);

        if socks_num != 0 {
            name += &format!("({socks_num}s)");
        }

        let cfg = self.cfg.borrow();

        if cfg.unit_color.item_extra_info {
            let quality = D2Common::Items::GetItemQuality(item);
            let unit_id = item.dwUnitId;
            let class_id = item.dwClassId;

            name = format!("UID:0x{unit_id:X} Q:{quality:?} CID:{class_id}<0x{class_id:X}>\n{name}");
        }

        let _: Option<()> = cfg.unit_color.get_color_from_unit(item).and_then(|item_color| {
            item_color.text_color.and_then(|text_color| {
                if text_color != D2StringColorCodes::Invalid {
                    while name.starts_with("ÿc") {
                        name = name.trim_start_matches("ÿc")[1..].to_string();
                    }

                    name.insert_str(0, text_color.to_str_code());
                }

                None
            })
        });

        // if let Some(item_color) = cfg.unit_color.get_color_from_unit(item) {
        //     if let Some(text_color) = item_color.text_color {
        //         if text_color != D2StringColorCodes::Invalid {
        //             while name.starts_with("ÿc") {
        //                 name = name.trim_start_matches("ÿc")[1..].to_string();
        //             }

        //             name.insert_str(0, &format!("ÿc{}", text_color as u8));
        //         }
        //     }
        // }

        let name = name.to_utf16();

        unsafe {
            name.as_ptr().copy_to_nonoverlapping(buffer, name.len());
        }
    }

    fn should_show_unit(&mut self, unit: &mut D2Unit) -> bool {
        let is_unit_item = D2Common::Inventory::UnitIsItem(unit) != FALSE;

        if is_unit_item == false {
            return true;
        }

        let cfg = Rc::clone(&self.cfg);
        let cfg = cfg.borrow();

        let unit_color = &cfg.unit_color;
        let mut should_auto_pickup = unit_color.auto_pickup;
        let should_hide_items = unit_color.hide_items;

        if should_auto_pickup {
            if let Some(pick) = self.should_auto_pickup_item(unit) {
                should_auto_pickup = pick;
            } else {
                should_auto_pickup = false;
            }
        }

        if should_hide_items == false && should_auto_pickup == false {
            return true;
        }

        let item_cfg = match unit_color.get_color_from_unit(unit) {
            None => return true,
            Some(color) => color,
        };

        if should_auto_pickup {
            self.handle_auto_pickup(unit, item_cfg);
        }

        if should_hide_items == false {
            return true;
        }

        let minimap_color = match item_cfg.minimap_color {
            None => return true,
            Some(color) => color,
        };

        if minimap_color == MINIMAP_COLOR_HIDE {
            unit.dwFlagEx.remove(D2UnitFlagsEx::IsInLos);
            return false;
        }

        unit.dwFlagEx.insert(D2UnitFlagsEx::IsInLos);

        true
    }

    fn on_leave_game(&mut self) {
        self.items_to_cube.clear();
    }

    fn on_post_recv(&mut self, cmd: D2GSCmd, payload: *mut u8) -> Option<()> {
        let mut state = ItemStateMonitor::new();

        state.on_scmd(cmd, payload);

        if state.add_to_ground == false && state.cursor_to_ground == false && state.ground_to_cursor == false {
            return None;
        }

        let item = D2Client::Units::GetClientUnit(state.unit_id, D2UnitTypes::Item)?;

        if state.add_to_ground || state.cursor_to_ground {
            self.handle_dropped_item(item);
        } else if state.ground_to_cursor {
            self.handle_auto_pickup_cube(item);
        }

        None
    }

    fn handle_dropped_item(&mut self, item: &D2Unit) {
        let cfg = Rc::clone(&self.cfg);
        let cfg = cfg.borrow();

        let item_color = match cfg.unit_color.get_color_from_unit(item) {
            None => return,
            Some(c) => c,
        };

        let notify = match item_color.notify {
            None => return,
            Some(n) => n,
        };

        if notify == DropNotify::None {
            return;
        }

        let name_color = D2Sigma::Items::GetItemNameColor(item).to_str_code();

        if let Some(notify_text) = item_color.notify_text.as_ref() {
            D2Client::UI::DisplayGlobalMessage(&format!("{name_color} - {notify_text}"), D2StringColorCodes::Invalid);
            return;
        }

        let quality = D2Common::Items::GetItemQuality(item);
        let name = D2SigmaEx::Items::get_item_name(item);

        let mut name: Vec<&str> = name.split('\n').collect();
        let name_line_count = name.len();
        let item_data_tables = match D2Common::DataTbls::GetItemDataTables() {
            None => return,
            Some(p) => p,
        };

        name.reverse();

        let is_weapon_or_armor = (item.dwClassId as usize) < item_data_tables.nWeaponsTxtRecordCount + item_data_tables.nArmorTxtRecordCount;
        let is_misc = !is_weapon_or_armor;

        let name = if is_misc {
            name.join(" - ")

        } else if notify == DropNotify::Name || quality == D2ItemQualities::Unique {
            name[1..].join(" - ")

        } else {
            name.join(" - ")
        };

        D2Client::UI::DisplayGlobalMessage(&format!("{name_color} - {name}"), D2StringColorCodes::Invalid);

        if notify == DropNotify::Name {
            return;
        }

        let prop = D2SigmaEx::Items::get_item_properties(item, false);
        let prop_lines: Vec<&str> = prop.split("\n").collect();

        for line in prop_lines.iter().skip(name_line_count) {
            D2Client::UI::DisplayGlobalMessage(&format!("    {}", line), D2StringColorCodes::Invalid);
        }
    }

    fn should_auto_pickup_item(&self, item: &D2Unit) -> Option<bool> {
        let player = D2Client::Units::GetClientPlayer()?;
        let item_coord = D2Common::Units::GetCoords(item);
        let distance = D2Common::Units::GetDistanceToCoordinates(player, item_coord.nX, item_coord.nY);

        if distance > 5 {
            return Some(false);
        }

        Some(true)
    }

    fn handle_auto_pickup_cube(&mut self, item: &D2Unit) -> Option<()> {
        let expire_time = self.items_to_cube.get(&item.dwUnitId)?;

        if SystemTime::now() < *expire_time {
            D2ClientEx::Utils::cursor_item_to_cube();
        }

        None
    }

    fn handle_auto_pickup(&mut self, item: &D2Unit, item_cfg: &super::config::ItemColor) -> Option<()> {
        let pickup = item_cfg.pickup?;

        match pickup {
            PickupMethod::None => return None,

            PickupMethod::Inventory => {
                let cmd = D2Common::SCMD_PACKET_16_PIKCUP_ITEM{
                    nHeader       : D2ClientCmd::PICKUP_ITEM as u8,
                    dwUnitType    : D2UnitTypes::Item as u32,
                    dwUnitGUID    : item.dwUnitId,
                    bCursor       : 0,
                };

                D2ClientEx::Net::send_packet(&cmd);
            },

            PickupMethod::Cube => {
                D2ClientEx::Inventory::get_free_position_for_item(item, D2ItemInvPage::Cube)?;

                let cmd = D2Common::SCMD_PACKET_16_PIKCUP_ITEM{
                    nHeader       : D2ClientCmd::PICKUP_ITEM as u8,
                    dwUnitType    : D2UnitTypes::Item as u32,
                    dwUnitGUID    : item.dwUnitId,
                    bCursor       : 1,
                };

                D2ClientEx::Net::send_packet(&cmd);

                self.items_to_cube.insert(item.dwUnitId, SystemTime::now().add(Duration::from_secs(5)));
            },
        }

        None
    }

    pub fn init(&mut self, modules: &D2Modules) -> Result<(), HookError> {
        unsafe {
            let glide3x = modules.glide3x.unwrap();

            match (&*RtlImageNtHeader(modules.glide3x.unwrap() as PVOID)).FileHeader.TimeDateStamp {
                0x6606E04D => {
                    // drawImageHooked
                    HackMap::unit_color().glide3x_is_d2sigma = (glide3x + 0x5BFF3135 - 0x5BD50000) as *mut u8;
                },

                _ => {},
            }

            inline_hook_jmp(0, D2Client::AddressTable.Units.ShouldShowUnit, naked_should_show_unit as usize, Some(&mut STUBS.ShouldShowUnit), None)?;
            inline_hook_jmp(0, D2Common::AddressTable.DataTbls.CompileTxt, DATATBLS_CompileTxt as usize, Some(&mut STUBS.DATATBLS_CompileTxt), None)?;
            inline_hook_jmp::<()>(0, D2Sigma::AddressTable.AutoMap.DrawAutoMap, d2sigma_automap_draw as usize, None, None)?;
            inline_hook_jmp(0, D2Sigma::AddressTable.Items.GetItemName, d2sigma_items_get_item_name as usize, Some(&mut STUBS.D2Sigma_Items_GetItemName), None)?;
        }

        let input = HackMap::input();

        input.reg_toggle("hide_items", |vk| {
            let cfg = HackMap::config();
            let mut cfg = cfg.borrow_mut();

            if vk == cfg.hotkey.hide_items {
                cfg.unit_color.hide_items = !cfg.unit_color.hide_items;
                return (true, cfg.unit_color.hide_items);
            }

            (false, false)
        });

        input.reg_toggle("item_extra_info", |vk| {
            let cfg = HackMap::config();
            let mut cfg = cfg.borrow_mut();

            if vk == cfg.hotkey.item_extra_info {
                cfg.unit_color.item_extra_info = !cfg.unit_color.item_extra_info;
                return (true, cfg.unit_color.item_extra_info)
            }

            (false, false)
        });

        input.reg_toggle("auto_pickup", |vk| {
            let cfg = HackMap::config();
            let mut cfg = cfg.borrow_mut();

            if vk == cfg.hotkey.auto_pickup {
                cfg.unit_color.auto_pickup = !cfg.unit_color.auto_pickup;
                return (true, cfg.unit_color.auto_pickup)
            }

            (false, false)
        });

        D2ClientEx::Net::on_post_recv(|cmd, payload| {
            HackMap::unit_color().on_post_recv(cmd, payload);
        });

        D2ClientEx::Game::on_leave_game(|| {
            HackMap::unit_color().on_leave_game();
        });

        Ok(())
    }

}
