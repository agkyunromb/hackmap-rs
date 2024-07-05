#[repr(u8)]
pub enum D2ItemActionType
{
    AddToGround               = 0,
    GroundToCursor            = 1,
    DropToGround              = 2,
    OnGround                  = 3,
    PutInContainer            = 4,
    RemoveFromContainer       = 5,
    Equip                     = 6,
    IndirectlySwapBodyItem    = 7,
    Unequip                   = 8,
    SwapBodyItem              = 9,
    AddQuantity               = 10,
    AddToShop                 = 11,
    RemoveFromShop            = 12,
    SwapInContainer           = 13,
    PutInBelt                 = 14,
    RemoveFromBelt            = 15,
    SwapInBelt                = 16,
    AutoUnequip               = 17,
    ToCursor                  = 18,
    ItemInSocket              = 19,
    Unknown0x14               = 20,
    UpdateStats               = 21,
    Unknown0x16               = 22,
    WeaponSwitch              = 23,
}

#[repr(C, packed(1))]
pub struct SCMD_PACKET_16_PIKCUP_ITEM {
    // size: 0x000d
    pub nHeader                 : u8,                   // 0x0000
    pub dwUnitType              : u32,                  // 0x0001
    pub dwUnitGUID              : u32,                  // 0x0005
    pub bCursor                 : u32,                  // 0x0009
}

#[repr(C, packed(1))]
pub struct SCMD_PACKET_17_DROPCURSORITEM {
    // size: 0x0005
    pub nHeader                 : u8,                   // 0x0000
    pub dwItemGUID              : u32,                  // 0x0005
}

#[repr(C, packed(1))]
pub struct SCMD_PACKET_9C_ITEMACTION {
    // size: 0x00fc
    pub nHeader                 : u8,                   // 0x0000
    pub nAction                 : D2ItemActionType,     // 0x0001
    pub nPacketSize             : u8,                   // 0x0002
    pub nComponent              : u8,                   // 0x0003
    pub nItemId                 : u32,                  // 0x0004
    pub pBitstream              : [u8; 244],            // 0x0008
}

#[repr(C, packed(1))]
pub struct SCMD_PACKET_9D_ITEM_OWNED {
    // size: 0x0101
    pub nHeader                  : u8,                  // 0x0000
    pub nAction                  : D2ItemActionType,    // 0x0001
    pub nPacketSize              : u8,                  // 0x0002
    pub nComponent               : u8,                  // 0x0003
    pub nItemId                  : u32,                 // 0x0004
    pub nUnitType                : u8,                  // 0x0008
    pub nUnitId                  : u32,                 // 0x0009
    pub pBitstream               : [u8; 244],           // 0x000d
}
