use std::marker::PhantomData;
use std::ptr::addr_of;
use std::ops::Deref;
use std::os::raw::c_void;
use std::sync::OnceLock;

pub type FuncAddress = usize;
pub type PVOID = *mut c_void;

pub trait CStringToStr {
    fn to_str(self) -> &'static str;
}

impl CStringToStr for *const u8 {
    fn to_str(self) -> &'static str {
        if self.is_null() {
            return "";
        }

        unsafe {
            std::ffi::CStr::from_ptr(self as *const i8).to_str().unwrap()
        }
    }
}

impl CStringToStr for *const i8 {
    fn to_str(self) -> &'static str {
        (self as *const u8).to_str()
    }
}

pub trait UTF16ToString {
    fn to_string(self) -> String;
}

impl UTF16ToString for *const u16 {
    fn to_string(self) -> String {
        if self.is_null() {
            return String::new();
        }

        let mut len = 0;
        unsafe {
            while *self.offset(len) != 0 {
                len += 1;
            }

            let slice = std::slice::from_raw_parts(self, len as usize);
            String::from_utf16_lossy(slice)
        }
    }
}

impl UTF16ToString for *mut u16 {
    fn to_string(self) -> String {
        (self as *const u16).to_string()
    }
}

pub trait StrToUTF16 {
    fn to_utf16(&self) -> Vec<u16>;
}

impl StrToUTF16 for &str {
    fn to_utf16(&self) -> Vec<u16> {
        self.encode_utf16().chain(std::iter::once(0)).collect()
    }
}

impl StrToUTF16 for String {
    fn to_utf16(&self) -> Vec<u16> {
        self.as_str().to_utf16()
    }
}

pub fn read_at<R>(addr: usize) -> R {
    unsafe {
        (addr as *const R).read()
    }
}

pub struct D2Modules {
    pub D2Sigma     : Option<usize>,
    pub D2Client    : Option<usize>,
    pub D2Win       : Option<usize>,
    pub D2Common    : Option<usize>,
    pub D2Gfx       : Option<usize>,
    pub D2CMP       : Option<usize>,
    pub D2Multi     : Option<usize>,
    pub Fog         : Option<usize>,
    pub Storm       : Option<usize>,
    pub glide3x     : Option<usize>,
}

impl Default for D2Modules {
    fn default() -> Self {
        Self {
            D2Sigma   : None,
            D2Client  : None,
            D2Win     : None,
            D2Common  : None,
            D2Gfx     : None,
            D2CMP     : None,
            D2Multi   : None,
            Fog       : None,
            Storm     : None,
            glide3x   : None,
        }
    }
}

// https://github.com/actix/actix-web/blob/66905efd7b02a464f0becff59685c8ce58f243c2/actix-web/src/handler.rs#L89
pub trait Handler<Args>: Clone + 'static {
    type Output;
    type FuncType;
    type FastCall;
    type StdCall;
    type Cdecl;

    fn invoke(&self, args: Args) -> Self::Output;
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
    impl<Func, Ret, $($param,)*> Handler<($($param,)*)> for Func
    where
        Func: Fn($($param),*) -> Ret + Clone + 'static,
    {
        type Output = Ret;
        type FuncType = fn($($param),*) -> Ret;
        type FastCall = extern "fastcall" fn($($param),*) -> Ret;
        type StdCall = extern "stdcall" fn($($param),*) -> Ret;
        type Cdecl = extern "cdecl" fn($($param),*) -> Ret;

        #[inline]
        #[allow(non_snake_case)]
        fn invoke(&self, ($($param,)*): ($($param,)*)) -> Self::Output {
            (self)($($param,)*)
        }
    }
});


/*
for i in range(11):
    args = ' '.join(['Arg%d' % (n + 1) for n in range(i)])
    print(f'factory_tuple! {{ {args} }}')
*/

factory_tuple! { }
factory_tuple! { Arg1 }
factory_tuple! { Arg1 Arg2 }
factory_tuple! { Arg1 Arg2 Arg3 }
factory_tuple! { Arg1 Arg2 Arg3 Arg4 }
factory_tuple! { Arg1 Arg2 Arg3 Arg4 Arg5 }
factory_tuple! { Arg1 Arg2 Arg3 Arg4 Arg5 Arg6 }
factory_tuple! { Arg1 Arg2 Arg3 Arg4 Arg5 Arg6 Arg7 }
factory_tuple! { Arg1 Arg2 Arg3 Arg4 Arg5 Arg6 Arg7 Arg8 }
factory_tuple! { Arg1 Arg2 Arg3 Arg4 Arg5 Arg6 Arg7 Arg8 Arg9 }
factory_tuple! { Arg1 Arg2 Arg3 Arg4 Arg5 Arg6 Arg7 Arg8 Arg9 Arg10 }

pub fn addr_to_stdcall<F, T>(_: F, addr: usize) -> F::StdCall
where
    F: Handler<T>,
{
    unsafe {
        std::mem::transmute_copy(&addr)
    }
}

pub fn addr_to_fastcall<F, T>(_: F, addr: usize) -> F::FastCall
where
    F: Handler<T>,
{
    unsafe {
        std::mem::transmute_copy(&addr)
    }
}

pub fn addr_to_cdecl<F, T>(_: F, addr: usize) -> F::Cdecl
where
    F: Handler<T>,
{
    unsafe {
        std::mem::transmute_copy(&addr)
    }
}

pub fn ptr_to_ref_mut<T>(ptr: *mut T) -> Option<&'static mut T> {
    if ptr.is_null() { None } else { unsafe { Some(&mut *ptr) } }
}

pub struct OnceHolder<T> {
    inner       : OnceLock<T>,
    initialized : OnceLock<bool>,
}

impl<T> OnceHolder<T> {
    pub const fn new() -> Self {
        Self {
            inner       : OnceLock::new(),
            initialized : OnceLock::new(),
        }
    }

    pub fn initialize(&self, t: T) {
        self.inner.set(t).ok().unwrap();
        self.initialized.set(true).ok().unwrap();
    }

    pub fn initialized(&self) -> bool {
        *self.initialized.get().unwrap_or(&false)
    }
}

impl<T> Deref for OnceHolder<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner.get().unwrap()
    }
}

pub trait D2ImageBase {
    const D2Client  : usize;
    const D2Common  : usize;
    const D2Win     : usize;
    const D2Multi   : usize;
    const D2Gfx     : usize;
    const D2CMP     : usize;
    const Fog       : usize;
    const Storm     : usize;
}

pub(crate) struct D2RVA_BASE<T: D2ImageBase>(PhantomData<T>);

impl<T: D2ImageBase> D2RVA_BASE<T> {
    pub fn D2Client(va: usize) -> usize {
        va - T::D2Client
    }

    pub fn D2Common(va: usize) -> usize {
        va - T::D2Common
    }

    pub fn D2Win(va: usize) -> usize {
        va - T::D2Win
    }

    pub fn D2Multi(va: usize) -> usize {
        va - T::D2Multi
    }

    pub fn D2Gfx(va: usize) -> usize {
        va - T::D2Gfx
    }

    pub fn D2CMP(va: usize) -> usize {
        va - T::D2CMP
    }

    pub fn Fog(va: usize) -> usize {
        va - T::Fog
    }

    pub fn Storm(va: usize) -> usize {
        va - T::Storm
    }
}
