use super::common::*;
use super::image_loader;
use super::HackMap;
use super::config::ConfigRef;
use D2Common::Units::D2Unit;

struct Stubs {
    DATATBLS_CompileTxt: Option<extern "stdcall" fn(PVOID, PCSTR, PVOID, &mut i32, usize) -> PVOID>,
}

static mut STUBS: Stubs = Stubs{
    DATATBLS_CompileTxt: None,
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

extern "stdcall" fn D2Sigma_AutoMap_Draw() {
    if D2Client::UI::GetUIVar(D2UIvars::EscMenu) != 0 || D2Client::UI::GetUIVar(D2UIvars::Config) != 0 {
        return;
    }

    if D2Client::UI::GetUIOpenMode() == 3 {
        return;
    }

    D2Client::AutoMap::DrawAutoMapCells();
    let _ = HackMap::unit_color().draw_automap_units();
}

extern "stdcall" fn D2Sigma_AutoMap_DrawUnits() {
    let _ = HackMap::unit_color().draw_automap_units();
}

pub(super) struct UnitColor {
    pub cfg                 : ConfigRef,
    pub boss_monster_id     : HashMap<u32, u32>,
    pub glide3x_is_d2sigma  : *mut u8,
}

impl UnitColor {
    pub fn new(cfg: ConfigRef) -> Self{
        Self{
            cfg,
            boss_monster_id     : HashMap::new(),
            glide3x_is_d2sigma  : null_mut(),
        }
    }

    fn init_automap_monster_colors(&mut self, data: PVOID, recordCount: usize, recordSize: usize) {
        // let mut specified_colors: [u8; 0x2000] = [0xFE; 0x2000];

        // const MONSTATSFLAG_INTOWN   : u32 = 0x400;
        // const MONSTATSFLAG_NPC      : u32 = 0x100;
        // const MONSTATSFLAG_BOSS     : u32 = 0x40;
        // const DefaultColor          : u8 = 0x60;
        // const DefaultBossColor      : u8 = 0x9B;
        // const DefaultSummonColor    : u8 = 0xD0;
        // const DefaultDangerMonster  : u8 = 0x62;

        // specified_colors[0x0E3]   = 0xFF;                   // 虫子
        // specified_colors[0x3D4]   = DefaultSummonColor;     // 冰图腾
        // specified_colors[0x3D5]   = DefaultSummonColor;     // 火图腾
        // specified_colors[0x3D7]   = DefaultSummonColor;     // 电图腾
        // specified_colors[0x434]   = 0xFF;                   // JJR地面的东西
        // specified_colors[0x441]   = 0xFF;                   // JJR地面的东西
        // specified_colors[0x442]   = 0xFF;                   // 妹子岛地面的东西
        // specified_colors[0x44C]   = 0xFF;                   // JJR地面的东西
        // specified_colors[0x63C]   = 0x84;                   // 彼列真身
        // specified_colors[0x646]   = 0xFF;                   // 彼列分身
        // specified_colors[0xD30]   = DefaultSummonColor;     // 复活的行尸
        // specified_colors[2036]    = 0x84;                   // 灵魂收割者
        // specified_colors[3556]    = DefaultDangerMonster;   // 马萨伊尔的执政官 lv120
        // specified_colors[3563]    = DefaultDangerMonster;   // 马萨伊尔的执政官 lv130
        // specified_colors[3558]    = DefaultDangerMonster;   // 伊瑟瑞尔的先锋 lv120
        // specified_colors[3565]    = DefaultDangerMonster;   // 伊瑟瑞尔的先锋 lv130
        // specified_colors[3559]    = DefaultDangerMonster;   // 英普瑞斯的怒火 lv120
        // specified_colors[3566]    = DefaultDangerMonster;   // 英普瑞斯的怒火 lv120

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

            // match specified_colors[i] {
            //     0xFF => {},

            //     0xFE => {
            //         if mon_stats_flags.contains(D2MonStatsTxtFlags::InTown | D2MonStatsTxtFlags::Npc) {
            //             mon_stats_3[0] |= 2;
            //             mon_stats_3[4] = if mon_stats_flags.contains(D2MonStatsTxtFlags::Boss) { DefaultBossColor } else { DefaultColor };
            //         }
            //     },

            //     _ => {
            //         mon_stats_3[0] |= 2;
            //         mon_stats_3[4] = specified_colors[i];
            //     },
            // }

            mon_stats_3 = &mut mon_stats_3[recordSize..];
        }
    }

    fn draw_automap_units(&self) -> Result<(), ()> {
        let player = D2Client::Units::GetClientPlayer().ok_or(())?;
        let room = D2Common::Units::GetRoom(player).ok_or(())?;

        let adjacent_rooms = D2Common::Dungeon::GetAdjacentRoomsListFromRoom(room).ok_or(())?;

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

        Ok(())
    }

    fn draw_unit(&self, unit: &mut D2Unit) {
        let mut x = D2Common::Units::GetClientCoordX(unit);
        let mut y = D2Common::Units::GetClientCoordY(unit);

        let divisor = *D2Client::AutoMap::PointDivisor();
        let rect = D2Client::AutoMap::Rect();

        x = x / divisor - *D2Client::AutoMap::PointOffsetX() + 8;
        y = y / divisor - *D2Client::AutoMap::PointOffsetY() - 8;

        if x < rect.left || x > rect.right || y < rect.top || y > rect.bottom {
            return;
        }

        match unit.dwUnitType {
            D2UnitTypes::Player => {
                let _ = self.draw_player(unit, x, y);
            },
            D2UnitTypes::Monster => {
                let _ = self.draw_monster(unit, x, y);
            },

            D2UnitTypes::Missile => {},
            D2UnitTypes::Item => {},

            _ => {},
        }
    }

    fn draw_player(&self, unit: &mut D2Unit, x: i32, y: i32) -> Result<(), ()> {
        let unit_color_config = &self.cfg.borrow().unit_color;
        let player = D2Client::Units::GetClientPlayer().ok_or(())?;

        self.draw_cell_by_blob_file(x, y, unit_color_config.my_blob_file.as_ref(), if player.dwUnitId == unit.dwUnitId { 0x97 } else { 0x81 });

        Ok(())
    }

    fn draw_monster(&self, unit: &mut D2Unit, x: i32, y: i32) -> Result<(), ()> {
        let class_Id = unit.dwClassId;

        match class_Id {
            179 => return Ok(()),   // A1 红牛
            _ => {},
        }

        if D2Client::Units::IsCorpse(unit) {
            return Ok(());
        }

        let unit_color_cfg    = &self.cfg.borrow().unit_color;
        let data_tables       = D2Common::DataTbls::sgptDataTables();
        let monster_data      = unit.get_monster_data().ok_or(())?;
        let mon_stats_txt     = monster_data.get_mon_stats_txt().ok_or(())?;
        let mon_stats_flags   = mon_stats_txt.dwMonStatsFlags;

        if mon_stats_flags & (D2MonStatsTxtFlags::Npc | D2MonStatsTxtFlags::Interact) == (D2MonStatsTxtFlags::Npc | D2MonStatsTxtFlags::Interact) {
            D2WinEx::Text::draw_text(D2Client::Units::GetName(unit), x, y - 8, D2Font::Font6, D2StringColorCodes::DarkGold);
            self.draw_cell_by_blob_file(x, y, unit_color_cfg.npc_blob_file.as_ref(), 0xFF);
            return Ok(());
        }

        if mon_stats_flags.contains(D2MonStatsTxtFlags::InTown | D2MonStatsTxtFlags::Npc) {
            return Ok(());
        }

        let room = D2Common::Units::GetRoom(unit).ok_or(())?;
        let level_txt = data_tables.get_levels_txt_record(D2Common::Dungeon::GetLevelIdFromRoom(room)).ok_or(())?;

        for cmon in level_txt.wCMon {
            if cmon == class_Id as u16 {
                return Ok(());
            }
        }

        if D2Client::Units::GetMonsterOwnerID(unit) != u32::MAX {
            return Ok(());
        }

        // println!("class_id: {}", class_Id);

        let type_flag = monster_data.nTypeFlag;
        let mut color: u8;
        let mut show_name = false;
        let blob_file: Option<&String>;

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

        } else {
            color = unit_color_cfg.normal_monster_color;
            blob_file = unit_color_cfg.monster_blob_file.as_ref();
        }

        if let Some(c) = unit_color_cfg.monster_color.get(&class_Id) {
            if *c == 0xFF {
                return Ok(());
            }

            color = *c;
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

        Ok(())
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

}

pub fn init(modules: &D2Modules) -> Result<(), HookError> {
    unsafe {
        let glide3x = modules.glide3x.unwrap();

        match (&*RtlImageNtHeader(modules.glide3x.unwrap() as PVOID)).FileHeader.TimeDateStamp {
            0x6606E04D => {
                // drawImageHooked
                HackMap::unit_color().glide3x_is_d2sigma = (glide3x + 0x5BFF3135 - 0x5BD50000) as *mut u8;
            },

            _ => {},
        }

        inline_hook_jmp(0, D2Common::AddressTable.DataTbls.CompileTxt, DATATBLS_CompileTxt as usize, Some(&mut STUBS.DATATBLS_CompileTxt), None)?;
        // inline_hook_jmp::<()>(0, D2Sigma::AddressTable.AutoMap.DrawAutoMapUnits, D2Sigma_AutoMap_DrawUnits as usize, None, None)?;
        inline_hook_jmp::<()>(0, D2Sigma::AddressTable.AutoMap.DrawAutoMap, D2Sigma_AutoMap_Draw as usize, None, None)?;
    }

    Ok(())
}
