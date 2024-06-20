use super::common::*;

struct Stubs {
    DATATBLS_CompileTxt: Option<extern "stdcall" fn(PVOID, PCSTR, PVOID, *mut i32, usize) -> PVOID>,
}

static mut STUBS: Stubs = Stubs{
    DATATBLS_CompileTxt: None,
};

#[allow(static_mut_refs)]
fn get_stubs() -> &'static Stubs {
    unsafe { &STUBS }
}

extern "stdcall" fn DATATBLS_CompileTxt(archive: PVOID, name: PCSTR, tbl: PVOID, recordCount: *mut i32, recordSize: usize) -> PVOID {
    let data = get_stubs().DATATBLS_CompileTxt.unwrap()(archive, name, tbl, recordCount, recordSize);

    if data.is_null() || name.to_str() != "Monstats3" {
        return data;
    }

    let mut specified_colors: [u8; 0x2000] = [0xFE; 0x2000];

    const MONSTATSFLAG_INTOWN   : u32 = 0x400;
    const MONSTATSFLAG_NPC      : u32 = 0x100;
    const MONSTATSFLAG_BOSS     : u32 = 0x40;
    const DefaultColor          : u8 = 0x60;
    const DefaultBossColor      : u8 = 0x9B;
    const DefaultSummonColor    : u8 = 0xD0;
    const DefaultDangerMonster  : u8 = 0x62;

    specified_colors[0x0E3]   = 0xFF;                   // 虫子
    specified_colors[0x3D4]   = DefaultSummonColor;     // 冰图腾
    specified_colors[0x3D5]   = DefaultSummonColor;     // 火图腾
    specified_colors[0x3D7]   = DefaultSummonColor;     // 电图腾
    specified_colors[0x434]   = 0xFF;                   // JJR地面的东西
    specified_colors[0x441]   = 0xFF;                   // JJR地面的东西
    specified_colors[0x442]   = 0xFF;                   // 妹子岛地面的东西
    specified_colors[0x44C]   = 0xFF;                   // JJR地面的东西
    specified_colors[0x63C]   = 0x84;                   // 彼列真身
    specified_colors[0x646]   = 0xFF;                   // 彼列分身
    specified_colors[0xD30]   = DefaultSummonColor;     // 复活的行尸
    specified_colors[2036]    = 0x84;                   // 灵魂收割者
    specified_colors[3556]    = DefaultDangerMonster;   // 马萨伊尔的执政官 lv120
    specified_colors[3563]    = DefaultDangerMonster;   // 马萨伊尔的执政官 lv130
    specified_colors[3558]    = DefaultDangerMonster;   // 伊瑟瑞尔的先锋 lv120
    specified_colors[3565]    = DefaultDangerMonster;   // 伊瑟瑞尔的先锋 lv130
    specified_colors[3559]    = DefaultDangerMonster;   // 英普瑞斯的怒火 lv120
    specified_colors[3566]    = DefaultDangerMonster;   // 英普瑞斯的怒火 lv120

    let mut mon_stats_3 = unsafe { std::slice::from_raw_parts_mut(data as *mut u8, recordSize * *recordCount as usize) };

    let mut mon_stats_txt = D2Common::DataTbls::sgptDataTbls().mon_stats_txt() as usize;
    let mon_stats_txt_record_count = D2Common::DataTbls::sgptDataTbls().mon_stats_txt_record_count();

    for i in 0..mon_stats_txt_record_count {
        let mon_stats_flags: u32 = read_addr(mon_stats_txt + 0x0C);

        match specified_colors[i] {
            0xFF => {},

            0xFE => {
                if (mon_stats_flags & (MONSTATSFLAG_INTOWN | MONSTATSFLAG_NPC)) == 0 {
                    mon_stats_3[0] |= 2;
                    mon_stats_3[4] = if (mon_stats_flags & MONSTATSFLAG_BOSS) != 0 { DefaultBossColor } else { DefaultColor };
                }
            },

            _ => {
                mon_stats_3[0] |= 2;
                mon_stats_3[4] = specified_colors[i];
            },
        }

        mon_stats_txt += 0x1A8;
        mon_stats_3 = &mut mon_stats_3[recordSize..];
    }

    data
}

pub fn init(_modules: &D2Modules) -> Result<(), HookError> {
    unsafe {
        inline_hook_jmp(0, D2Common::AddressTable.DataTbls.CompileTxt, DATATBLS_CompileTxt as usize, Some(&mut STUBS.DATATBLS_CompileTxt), None)?;
    }

    Ok(())
}
