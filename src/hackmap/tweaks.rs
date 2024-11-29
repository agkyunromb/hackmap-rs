use D2Common::D2Unit;

use super::common::*;
use super::config::ConfigRef;
use super::HackMap;

struct Stubs {
    UI_HandleUIVars: Option<extern "stdcall" fn(PVOID)>,
    continue_attacking_after_target_dead_left: Option<extern "stdcall" fn()>,
    continue_attacking_after_target_dead_right: Option<extern "stdcall" fn()>,
    MPQLoadFile: Option<
        extern "stdcall" fn(*const u8, *mut u8, usize, *mut usize, *const u8, usize, usize) -> BOOL,
    >,
}

static mut STUBS: Stubs = Stubs {
    UI_HandleUIVars: None,
    continue_attacking_after_target_dead_left: None,
    continue_attacking_after_target_dead_right: None,
    MPQLoadFile: None,
};

#[allow(static_mut_refs)]
fn get_stubs() -> &'static Stubs {
    unsafe { &STUBS }
}

extern "stdcall" fn HandleUIVars(obj: PVOID) {
    HackMap::tweaks().handle_perm_show_items(obj);
}

extern "stdcall" fn MISC_CalculateShadowRGBA(r: &mut u8, g: &mut u8, b: &mut u8, a: &mut u8) {
    *a = 0xFF;
    *r = 0xFF;
    *g = 0xFF;
    *b = 0xFF;
}

extern "stdcall" fn D2Common_Units_TestCollisionWithUnit(
    _unit1: PVOID,
    _unit2: PVOID,
    _collision_mask: i32,
) -> BOOL {
    FALSE
    // let (success, hide) = HackMap::tweaks().should_hide_unit(unit2);

    // if success == false {
    //     return D2Common::Units::TestCollisionWithUnit(unit1, unit2, collision_mask);
    // }

    // if hide { TRUE } else { FALSE }
}

fn D2Sigma_Units_GetName(unit: &D2Unit, is_boss: bool) -> PCWSTR {
    let name = D2Sigma::Units::GetName(unit).to_string();

    let dr = D2Common::StatList::GetUnitBaseStat(unit, D2ItemStats::DamageResist, 0) as i32;
    let mr = D2Common::StatList::GetUnitBaseStat(unit, D2ItemStats::MagicResist, 0) as i32;
    let fr = D2Common::StatList::GetUnitBaseStat(unit, D2ItemStats::FireResist, 0) as i32;
    let lr = D2Common::StatList::GetUnitBaseStat(unit, D2ItemStats::LightResist, 0) as i32;
    let cr = D2Common::StatList::GetUnitBaseStat(unit, D2ItemStats::ColdResist, 0) as i32;
    let pr = D2Common::StatList::GetUnitBaseStat(unit, D2ItemStats::PoisonResist, 0) as i32;
    let hp = D2Common::StatList::GetUnitBaseStat(unit, D2ItemStats::HitPoints, 0) as f64;
    let max_hp = D2Common::StatList::GetUnitBaseStat(unit, D2ItemStats::MaxHp, 0) as f64;

    let class_id: u32 = unit.dwClassId;
    let cfg = HackMap::config();
    let cfg = cfg.borrow();

    let monster_name = if cfg.tweaks.show_monster_id {
        format!(
            "{name}({class_id}, 0x{class_id:X}) ÿc7{dr} ÿc8{mr} ÿc1{fr} ÿc9{lr} ÿc3{cr} ÿc2{pr}"
        )
    } else if is_boss {
        let percent = (hp * 100.0 / max_hp) as usize;
        format!("{name}({percent}%) ÿc7{dr} ÿc8{mr} ÿc1{fr} ÿc9{lr} ÿc3{cr} ÿc2{pr}")
    } else {
        let percent = (hp * 100.0 / max_hp) as usize;
        format!("{name}({percent}%%) ÿc7{dr} ÿc8{mr} ÿc1{fr} ÿc9{lr} ÿc3{cr} ÿc2{pr}")
    };

    let tweaks = HackMap::tweaks();
    tweaks.current_monster_name = monster_name.to_utf16();
    tweaks.current_monster_name.as_ptr()
}

extern "fastcall" fn D2Sigma_Units_GetNameForMonster(unit: &D2Unit) -> PCWSTR {
    D2Sigma_Units_GetName(unit, false)
}

extern "fastcall" fn D2Sigma_Units_GetNameForBoss(unit: &D2Unit) -> PCWSTR {
    D2Sigma_Units_GetName(unit, true)
}

extern "C" fn is_player_in_town() -> bool {
    let player = match D2Client::Units::GetClientPlayer() {
        Some(p) => p,
        None => return false,
    };

    let active_room = D2Common::Units::GetRoom(player).unwrap();

    D2Common::Dungeon::IsRoomInTown(active_room) != FALSE
}

global_asm!(
    r#"
.global _naked_is_player_running_1
_naked_is_player_running_1:

    test    ecx, ecx
    jz      _naked_is_player_running_1_NOT_RUNNING

    or      eax, 8
    ret

_naked_is_player_running_1_NOT_RUNNING:
    push    edx
    push    eax
    call    {is_player_in_town}
    mov     edx, [esp]
    or      edx, 8
    test    eax, eax
    pop     eax
    cmovne  eax, edx
    pop     edx
    ret
"#,
    is_player_in_town = sym is_player_in_town,
);

global_asm!(
    r#"
.global _naked_is_player_running_2
_naked_is_player_running_2:
    test    edx, edx
    jz      _naked_is_player_running_2_NOT_RUNNING

    or      eax, 8
    ret

_naked_is_player_running_2_NOT_RUNNING:
    push    ecx
    push    eax
    call    {is_player_in_town}
    mov     ecx, [esp]
    or      ecx, 8
    test    eax, eax
    pop     eax
    cmovne  eax, ecx
    pop     ecx
    ret
"#,
    is_player_in_town = sym is_player_in_town,
);

extern "C" {
    fn naked_is_player_running_1();
    fn naked_is_player_running_2();
}

fn continue_attacking_after_target_dead() {
    D2Client::UI::SetAttackWithLeftButton(0);

    if !HackMap::config()
        .borrow()
        .tweaks
        .continue_attacking_after_target_dead
    {
        D2Client::UI::SetAttackWithRightButton(0);
    }
}

extern "stdcall" fn MPQLoadFile(
    fileInfo: *const u8,
    buffer: *mut u8,
    bufferSize: usize,
    mut fileSize: *mut usize,
    eventInfo: *const u8,
    arg6: usize,
    arg7: usize,
) -> BOOL {
    let mut file_size: usize = 0;

    if fileSize.is_null() {
        fileSize = &mut file_size;
    }

    let file_name = ((fileInfo as usize + 8) as *const u8)
        .to_str()
        .to_lowercase();
    let file_name = file_name.as_str();

    // println!("load {file_name}");

    if HackMap::config()
        .borrow()
        .tweaks
        .excluded_dc6
        .contains(file_name)
    {
        return FALSE;
    }

    let success = get_stubs().MPQLoadFile.unwrap()(
        fileInfo, buffer, bufferSize, fileSize, eventInfo, arg6, arg7,
    );

    while success != FALSE {
        if true {
            break;
        }

        if file_name == "(attributes)" {
            break;
        }

        let dump_path = std::path::Path::new("MPQDumped").join(file_name);
        std::fs::create_dir_all(dump_path.parent().unwrap()).unwrap();

        let content = unsafe { std::slice::from_raw_parts(buffer, *fileSize) };

        std::fs::write(dump_path, content).unwrap();

        break;
    }

    success
}

pub(super) struct Tweaks {
    pub cfg: ConfigRef,
    pub current_monster_name: Vec<u16>,
}

impl Tweaks {
    pub const MAX_GLOBAL_MESSAGE_COUNT: i32 = 30;
    pub const MAX_QUICK_MESSAGE_COUNT: i32 = 20;

    pub fn new(cfg: ConfigRef) -> Self {
        Self {
            cfg,
            current_monster_name: vec![],
        }
    }

    fn should_hide_unit(&self, _unit: PVOID) -> (bool, bool) {
        let success = true;
        let hide = false;

        (success, hide)
    }

    fn handle_perm_show_items(&self, obj: PVOID) {
        let UI_HandleUIVars = get_stubs().UI_HandleUIVars.unwrap();

        if !self.cfg.borrow_mut().tweaks.perm_show_items
            || D2Client::UI::GetUIVar(D2UIvars::HoldAlt) != 0
        {
            UI_HandleUIVars(obj);
            return;
        }

        D2Client::UI::SetUIVar(D2UIvars::HoldAlt, 0, 0);
        UI_HandleUIVars(obj);
        D2Client::UI::SetUIVar(D2UIvars::HoldAlt, 1, 0);
    }

    pub fn init(&mut self, modules: &D2Modules) -> Result<(), HookError> {
        let D2Client = modules.D2Client.unwrap();

        unsafe {
            inline_hook_jmp(
                0,
                Storm::AddressTable.MPQLoadFile,
                MPQLoadFile as usize,
                Some(&mut STUBS.MPQLoadFile),
                None,
            )?;

            // 永久显示地面物品
            let glide3x = &*RtlImageNtHeader(modules.glide3x.unwrap() as PVOID);

            inline_hook_call(
                0,
                D2Client::AddressTable.UI.CallHandleUIVars,
                HandleUIVars as usize,
                Some(&mut STUBS.UI_HandleUIVars),
                None,
            )?;
            patch_memory_value(D2Client, D2RVA::D2Client(0x6FB0948B), 0xEB, 1)?;

            // global message 上限
            patch_memory_value(
                D2Client,
                D2RVA::D2Client(0x6FB2D9B2),
                Self::MAX_GLOBAL_MESSAGE_COUNT as u64,
                1,
            )?;

            // quick message 上限
            patch_memory_value(
                D2Client,
                D2RVA::D2Client(0x6FB2D7E3),
                Self::MAX_QUICK_MESSAGE_COUNT as u64,
                1,
            )?;

            // HDText_drawFramedText_is_alt_clicked
            match glide3x.FileHeader.TimeDateStamp {
                0x6606E04D => {
                    patch_memory_value(modules.glide3x.unwrap(), 0x55F2E, 0x80, 1)?;
                }

                0x6727FC35 => {
                    patch_memory_value(modules.glide3x.unwrap(), 0x54EC9, 0xEB, 1)?;
                }

                _ => {}
            }

            // 去除阴影
            if HackMap::config().borrow().tweaks.remove_shadow {
                inline_hook_jmp::<()>(
                    D2Client,
                    D2RVA::D2Client(0x6FB59A20),
                    MISC_CalculateShadowRGBA as usize,
                    None,
                    None,
                )?;
            }

            // 透视
            inline_hook_call::<()>(
                D2Client,
                D2RVA::D2Client(0x6FB16695),
                D2Common_Units_TestCollisionWithUnit as usize,
                None,
                None,
            )?;

            // 在城里默认跑步
            inline_hook_call::<()>(
                D2Client,
                D2RVA::D2Client(0x6FAF27D7),
                naked_is_player_running_1 as usize,
                None,
                None,
            )?;
            inline_hook_call::<()>(
                D2Client,
                D2RVA::D2Client(0x6FAF4930),
                naked_is_player_running_2 as usize,
                None,
                None,
            )?;

            // 显示抗性
            if D2Sigma::initialized() {
                inline_hook_call::<()>(
                    0,
                    D2Sigma::AddressTable.UI.BossLifeBar_Call_Units_GetName,
                    D2Sigma_Units_GetNameForBoss as usize,
                    None,
                    None,
                )?;
                inline_hook_call::<()>(
                    0,
                    D2Sigma::AddressTable.UI.MonsterLifeBar_Call_Units_GetName,
                    D2Sigma_Units_GetNameForMonster as usize,
                    None,
                    None,
                )?;
                patch_memory_value(
                    0,
                    D2Sigma::AddressTable.UI.CheckIsMonsterShouldDisplayLifeBar,
                    0x80,
                    1,
                )?;
            }

            // 目标死亡后不松开鼠标
            inline_hook_call::<()>(
                D2Client,
                D2RVA::D2Client(0x6FAF2AE1),
                continue_attacking_after_target_dead as usize,
                None,
                None,
            )?;
            inline_hook_call::<()>(
                D2Client,
                D2RVA::D2Client(0x6FAF2AE6),
                continue_attacking_after_target_dead as usize,
                None,
                None,
            )?;
        }

        HackMap::input().reg_toggle("show_monster_id", |vk| {
            let cfg = HackMap::config();
            let mut cfg = cfg.borrow_mut();

            if vk == cfg.hotkey.show_monster_id {
                cfg.tweaks.show_monster_id = !cfg.tweaks.show_monster_id;
                return (true, cfg.tweaks.show_monster_id);
            }

            (false, false)
        });

        HackMap::input().reg_toggle("perm_show_items", |vk| {
            let cfg = HackMap::config();
            let mut cfg = cfg.borrow_mut();

            if vk == cfg.hotkey.perm_show_items {
                cfg.tweaks.perm_show_items = !cfg.tweaks.perm_show_items;
                return (true, cfg.tweaks.perm_show_items);
            }

            (false, false)
        });

        Ok(())
    }
}
