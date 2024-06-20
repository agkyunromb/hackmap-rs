pub use crate::d2api::{*, d2113c::*};
pub use std::{iter::Once, marker::PhantomData, os::raw::c_void, ptr::{null, null_mut}, sync::OnceLock};
pub use ml::hooker::{err::HookError, x86::*};

pub use windows_sys::{
    core::{PCSTR, PCWSTR},
    Win32::Foundation::{BOOL, FALSE, TRUE, NTSTATUS, UNICODE_STRING},
    Win32::System::{
        WindowsProgramming::RtlInitUnicodeString,
        SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
        Diagnostics::Debug::IMAGE_NT_HEADERS32,
        // LibraryLoader::*,
    },
};