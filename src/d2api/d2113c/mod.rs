pub mod common;

use super::types as types;

pub use super::d2consts::*;
pub use types::*;
pub mod D2Common;
pub mod D2Client;
pub mod D2Gfx;
pub mod D2Win;
pub mod D2Multi;
pub mod D2CMP;
pub mod Fog;
pub mod Storm;
pub mod D2Sigma;

pub struct D2ImageBase113C;

impl types::D2ImageBase for D2ImageBase113C {
    const D2Client  : usize = 0x6FAB0000;
    const D2Common  : usize = 0x6FD50000;
    const D2Win     : usize = 0x6F8E0000;
    const D2Multi   : usize = 0x6F9D0000;
    const D2Gfx     : usize = 0x6FA80000;
    const D2CMP     : usize = 0x6FE10000;
    const Fog       : usize = 0x6FF50000;
    const Storm     : usize = 0x6FBF0000;
}

pub type D2RVA = types::D2RVA_BASE<D2ImageBase113C>;
