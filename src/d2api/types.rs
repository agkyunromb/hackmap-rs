use std::ptr::addr_of;
use std::ops::Deref;
use std::os::raw::c_void;
use std::sync::OnceLock;

pub type FuncAddress = usize;
pub type PVOID = *mut c_void;

// https://github.com/actix/actix-web/blob/66905efd7b02a464f0becff59685c8ce58f243c2/actix-web/src/handler.rs#L89
pub trait Handler<Args>: Clone + 'static {
    type Output;
    type FuncType;

    fn invoke(&self, args: Args) -> Self::Output;
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
    impl<Func, Ret, $($param,)*> Handler<($($param,)*)> for Func
    where
        Func: Fn($($param),*) -> Ret + Clone + 'static,
    {
        type Output = Ret;
        type FuncType = fn($($param),*) -> Ret;

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

pub fn addr_to_fn<F, T>(_: F, addr: usize) -> F::FuncType
where
    F: Handler<T>,
{
    let f: Option<F::FuncType> = None;
    unsafe {
        *(addr_of!(f) as *mut usize) = addr;
    }
    f.unwrap()
}

pub struct Holder<T> {
    inner: OnceLock<T>,
}

impl<T> Holder<T> {
    pub const fn new() -> Self {
        Self {
            inner: OnceLock::new(),
        }
    }

    pub fn initialize(&self, t: T) {
        self.inner.set(t).ok().unwrap();
    }
}

impl<T> Deref for Holder<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner.get().unwrap()
    }
}