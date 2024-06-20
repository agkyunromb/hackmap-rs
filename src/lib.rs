#![allow(non_snake_case, non_camel_case_types, dead_code, non_upper_case_globals, unused_imports)]

mod d2api;
mod hackmap;

use std::ptr::{null, null_mut};

use windows_sys::{
    core::PCWSTR,
    Win32::Foundation::{BOOL, FALSE, TRUE, NTSTATUS, UNICODE_STRING},
    Win32::System::{
        WindowsProgramming::RtlInitUnicodeString,
        SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
        Diagnostics::Debug::IMAGE_NT_HEADERS32,
        // LibraryLoader::*,
    },
};

use d2api::d2113c::*;

::windows_targets::link!("ntdll.dll" "system" fn LdrDisableThreadCalloutsForDll(DllHandle : PVOID) -> NTSTATUS);
::windows_targets::link!("ntdll.dll" "system" fn RtlImageNtHeader(Base: PVOID) -> *const IMAGE_NT_HEADERS32);
::windows_targets::link!("ntdll.dll" "system" fn LdrLoadDll(PathToFile: PCWSTR, DllCharacteristics: *mut u32, ModuleFileName: *mut UNICODE_STRING, DllHandle: *mut PVOID) -> NTSTATUS);

fn ldr_load_dll(dll_name: &str) -> PVOID {
    let mut module_file: UNICODE_STRING = UNICODE_STRING { Length: 0, MaximumLength: 0, Buffer: null_mut() };
    let mut dll_base: PVOID = null_mut();

    let dll_name_u16: Vec<_> = dll_name.encode_utf16().chain(std::iter::once(0)).collect();

    let status = unsafe {
        RtlInitUnicodeString(&mut module_file, dll_name_u16.as_ptr());
        LdrLoadDll(null(), null_mut(), &mut module_file, &mut dll_base)
    };

    if status < 0 {
        return null_mut();
    }

    dll_base
}

fn init(base_address: PVOID) -> BOOL {
    unsafe {
        LdrDisableThreadCalloutsForDll(base_address);
        RtlImageNtHeader(base_address);
    }

    let mut D2Sigma   : PVOID = null_mut();
    let mut D2Client  : PVOID = null_mut();
    let mut D2Win     : PVOID = null_mut();
    let mut D2Common  : PVOID = null_mut();
    let mut D2Gfx     : PVOID = null_mut();
    let mut D2Multi   : PVOID = null_mut();
    let mut Storm     : PVOID = null_mut();
    let mut glide3x   : PVOID = null_mut();

    let dlls = &mut [
        (&mut D2Sigma,  "D2Sigma.dll"),
        (&mut D2Client, "D2Client.dll"),
        (&mut D2Win,    "D2Win.dll"),
        (&mut D2Common, "D2Common.dll"),
        (&mut D2Gfx,    "D2Gfx.dll"),
        (&mut D2Multi,  "D2Multi.dll"),
        (&mut Storm,    "Storm.dll"),
        (&mut glide3x,  "glide3x.dll"),
    ];

    for (dll_base, dll_name) in dlls.iter_mut() {
        **dll_base = ldr_load_dll(dll_name);
        if dll_base.is_null() {
            return FALSE;
        }
    }

    D2Common::init(D2Common as usize);
    D2Client::init(D2Client as usize);

    D2Client::UI::SetUIVar(0, 0, 0);
    D2Client::Net::SendPacket(null_mut(), 0);
    D2Common::StatList::GetUnitBaseStat(null_mut(), 0, 0);

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
