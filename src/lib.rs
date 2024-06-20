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

use d2api::*;

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
    use d2api::d2113c::*;
    use hackmap;

    unsafe {
        LdrDisableThreadCalloutsForDll(base_address);
        RtlImageNtHeader(base_address);
    }

    let mut d2modules = d2api::types::D2Modules::default();

    let dlls: &mut [(&mut Option<usize>, Option<fn(usize)>, &str)]  = &mut [
        (&mut d2modules.D2Sigma,  None,                 "D2Sigma2.dll"),
        (&mut d2modules.D2Client, Some(D2Client::init), "D2Client.dll"),
        (&mut d2modules.D2Win,    Some(D2Win::init),    "D2Win.dll"),
        (&mut d2modules.D2Common, Some(D2Common::init), "D2Common.dll"),
        (&mut d2modules.D2Gfx,    Some(D2Gfx::init),    "D2Gfx.dll"),
        (&mut d2modules.D2Multi,  Some(D2Multi::init),  "D2Multi.dll"),
        // (&mut d2modules.Storm,    None,                 "Storm.dll"),
        (&mut d2modules.glide3x,  None,                 "glide3x.dll"),
    ];

    for (dll_base, _, dll_name) in dlls.iter_mut() {
        let base = ldr_load_dll(dll_name);
        if base.is_null() {
            return FALSE;
        }

        **dll_base = Some(base as usize);
    }

    for (&mut dll_base, init_func, _) in dlls.iter() {
        if let Some(init_func) = init_func {
            init_func(dll_base.unwrap() as usize);
        }
    }

    hackmap::init(&d2modules);

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
