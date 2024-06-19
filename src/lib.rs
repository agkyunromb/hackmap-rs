#![allow(non_snake_case, non_camel_case_types, dead_code, non_upper_case_globals, unused_imports)]

mod d2api;

use windows_sys::{
    Win32::Foundation::{BOOL, TRUE, NTSTATUS},
    Win32::System::{
        SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
        // LibraryLoader::*,
    },
};

use d2api::d2113c::*;

::windows_targets::link!("ntdll.dll" "system" fn LdrDisableThreadCalloutsForDll(DllHandle : PVOID) -> NTSTATUS);

fn init(_base_address: PVOID) -> BOOL {
    unsafe { LdrDisableThreadCalloutsForDll(_base_address); }

    D2Common::init(0x10000000);
    D2Client::init(0x20000000);

    D2Client::UI::SetUIVar(0, 0, 0);
    D2Client::Net::SendPacket(std::ptr::null_mut(), 0);
    D2Common::StatList::GetUnitBaseStat(std::ptr::null_mut(), 0, 0);

    TRUE
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(BaseAddress: PVOID, Reason: u32, _Reversed: PVOID) -> BOOL {
    match Reason {
        DLL_PROCESS_ATTACH => init(BaseAddress),
        DLL_PROCESS_DETACH => TRUE,
        _ => TRUE
    }
}
