pub use std::ptr::{addr_of, addr_of_mut};
pub use crate::d2api::{*, d2113c::*};
pub use std::{iter::Once, marker::PhantomData, os::raw::c_void, ptr::{null, null_mut}, sync::OnceLock};
pub use ml::hooker::{err::HookError, x86::*};

pub use windows_sys::{
    core::{PCSTR, PCWSTR},
    Win32::{
        UI::WindowsAndMessaging::WM_KEYDOWN,
        UI::Input::KeyboardAndMouse::{GetKeyState, VK_OEM_PLUS, VK_SHIFT, VK_CONTROL},

        Foundation::{HWND, BOOL, FALSE, TRUE, NTSTATUS, UNICODE_STRING},

        System::{
            WindowsProgramming::RtlInitUnicodeString,
            SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
            Diagnostics::Debug::IMAGE_NT_HEADERS32,
        },
    },
};

::windows_targets::link!("ntdll.dll" "system" fn RtlImageNtHeader(Base: PVOID) -> *const IMAGE_NT_HEADERS32);
