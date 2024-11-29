pub use crate::d2api::{d2113c::*, *};
pub use ml::hooker::{err::HookError, x86::*};
pub use std::arch::global_asm;
pub use std::cell::RefCell;
pub use std::collections::HashMap;
pub use std::ptr::{addr_of, addr_of_mut};
pub use std::rc::Rc;
pub use std::{
    iter::Once,
    marker::PhantomData,
    os::raw::c_void,
    ptr::{null, null_mut},
    sync::OnceLock,
};

pub use windows_sys::{
    core::{PCSTR, PCWSTR, PWSTR},
    Win32::{
        Foundation::{BOOL, FALSE, HWND, NTSTATUS, TRUE, UNICODE_STRING},
        System::{
            Diagnostics::Debug::IMAGE_NT_HEADERS32,
            SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
            WindowsProgramming::RtlInitUnicodeString,
        },
        UI::Input::KeyboardAndMouse::{
            GetKeyState, VK_CONTROL, VK_OEM_MINUS, VK_OEM_PLUS, VK_SHIFT,
        },
        UI::WindowsAndMessaging::{MB_OK, WM_KEYDOWN},
    },
};

::windows_targets::link!("ntdll.dll" "system" fn RtlImageNtHeader(Base: PVOID) -> *const IMAGE_NT_HEADERS32);
