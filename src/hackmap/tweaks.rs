use D2Common::Units::D2Unit;

use super::common::*;
use super::HackMap;
use super::config::ConfigRef;

struct Stubs {
    UI_HandleUIVars: Option<extern "stdcall" fn(PVOID)>,
}

static mut STUBS: Stubs = Stubs{
    UI_HandleUIVars: None,
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

extern "stdcall" fn D2Common_Units_TestCollisionWithUnit(unit1: PVOID, unit2: PVOID, collision_mask: i32) -> BOOL {
    let (success, hide) = HackMap::tweaks().should_hide_unit(unit2);

    if success == false {
        return D2Common::Units::TestCollisionWithUnit(unit1, unit2, collision_mask);
    }

    if hide { TRUE } else { FALSE }
}

extern "fastcall" fn D2Sigma_Units_GetName(unit: &D2Unit) -> PCWSTR {

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

    let monster_name = if unsafe { GetKeyState(VK_SHIFT as i32) < 0 } {
        format!("{name}({class_id}, 0x{class_id:X}) ÿc7{dr} ÿc8{mr} ÿc1{fr} ÿc9{lr} ÿc3{cr} ÿc2{pr}")
    } else {
        let percent = (hp * 100.0 / max_hp) as usize;
        format!("{name}({percent}%%) ÿc7{dr} ÿc8{mr} ÿc1{fr} ÿc9{lr} ÿc3{cr} ÿc2{pr}")
    };

    let hm = HackMap::tweaks();
    hm.current_monster_name = monster_name.to_utf16();
    hm.current_monster_name.as_ptr()
}

fn is_player_in_town() -> bool {
    let player = match D2Client::Units::GetClientPlayer() {
        Some(p) => p,
        None => return false,
    };

    let active_room = D2Common::Units::GetRoom(&player).unwrap();

    D2Common::Dungeon::IsRoomInTown(active_room) != FALSE
}

extern "fastcall" fn D2Client_IsPlayerRunning(is_running: BOOL, arg2: u32) -> u32 {
    let mut flags: u32;

    unsafe {
        std::arch::asm!(
            "",
            out("eax") flags,
        );
    }

    if is_running != FALSE || is_player_in_town() {
        flags |= 8
    }

    unsafe {
        std::arch::asm!(
            "",
            in("edx") arg2,
        )
    }

    flags
}

extern "fastcall" fn D2Client_IsPlayerRunning2(arg1: usize, is_running: BOOL) -> u32 {
    let mut flags: u32;

    unsafe {
        std::arch::asm!(
            "",
            out("eax") flags,
        );
    }

    if is_running != FALSE || is_player_in_town() {
        flags |= 8
    }

    unsafe {
        std::arch::asm!(
            "",
            in("ecx") arg1,
        )
    }

    flags
}

pub(super) struct Tweaks {
    pub cfg: super::config::ConfigRef,
    pub current_monster_name: Vec<u16>,
}

impl Tweaks {
    pub fn new(cfg: ConfigRef) -> Self{
        Self{
            cfg,
            current_monster_name: vec![],
        }
    }

    fn on_key_down(&self, vk: u16) -> bool {
        let mut cfg = self.cfg.borrow_mut();

        if vk == 'Y' as u16 {
            cfg.tweaks.perm_show_items_toggle = !cfg.tweaks.perm_show_items_toggle;
        }

        false
    }

    fn should_hide_unit(&self, _unit: PVOID) -> (bool, bool) {
        let success = true;
        let hide = false;

        (success, hide)
    }

    fn handle_perm_show_items(&self, obj: PVOID) {
        let UI_HandleUIVars = get_stubs().UI_HandleUIVars.unwrap();

        if self.cfg.borrow_mut().tweaks.perm_show_items_toggle == false || D2Client::UI::GetUIVar(D2UIvars::HoldAlt) != 0 {
            UI_HandleUIVars(obj);
            return;
        }

        D2Client::UI::SetUIVar(D2UIvars::HoldAlt, 0, 0);
        UI_HandleUIVars(obj);
        D2Client::UI::SetUIVar(D2UIvars::HoldAlt, 1, 0);
    }
}

pub fn init(modules: &D2Modules) -> Result<(), HookError> {
    HackMap::input().on_key_down(|vk| -> bool {
        HackMap::tweaks().on_key_down(vk)
    });

    let D2Client = modules.D2Client.unwrap();

    unsafe {
        // 永久显示地面物品
        let glide3x = &*RtlImageNtHeader(modules.glide3x.unwrap() as PVOID);

        inline_hook_call(0, D2Client::AddressTable.UI.HandleUIVars, HandleUIVars as usize, Some(&mut STUBS.UI_HandleUIVars), None)?;
        patch_memory_value(D2Client, D2RVA::D2Client(0x6FB0948B), 0xEB, 1)?;

        // HDText_drawFramedText_is_alt_clicked
        match glide3x.FileHeader.TimeDateStamp {
            0x6606E04D => {
                patch_memory_value(modules.glide3x.unwrap(), 0x55F2E, 0x80, 1)?;
            },

            _ => {},
        }

        // 去除阴影
        inline_hook_jmp::<()>(D2Client, D2RVA::D2Client(0x6FB59A20), MISC_CalculateShadowRGBA as usize, None, None)?;

        // 透视
        inline_hook_call::<()>(D2Client, D2RVA::D2Client(0x6FB16695), D2Common_Units_TestCollisionWithUnit as usize, None, None)?;

        // 在城里默认跑步
        inline_hook_call::<()>(D2Client, D2RVA::D2Client(0x6FAF27D7), D2Client_IsPlayerRunning as usize, None, None)?;
        inline_hook_call::<()>(D2Client, D2RVA::D2Client(0x6FAF4930), D2Client_IsPlayerRunning2 as usize, None, None)?;

        // 显示抗性
        if D2Sigma::initialized() {
            inline_hook_call::<()>(0, D2Sigma::AddressTable.UI.BossLifeBar_Call_Units_GetName, D2Sigma_Units_GetName as usize, None, None)?;
            inline_hook_call::<()>(0, D2Sigma::AddressTable.UI.MonsterLifeBar_Call_Units_GetName, D2Sigma_Units_GetName as usize, None, None)?;
            patch_memory_value(0, D2Sigma::AddressTable.UI.CheckIsMonsterShouldDisplayLifeBar, 0x80, 1)?;
        }
    }

    Ok(())
}
