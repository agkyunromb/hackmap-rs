use super::super::super::d2consts::*;
use std::ptr::addr_of;

#[repr(C, packed(1))]
pub struct D2LevelDefBin {
    pub dwQuestFlag     : u32,              // 0x0000
    pub dwQuestFlagEx   : u32,              // 0x0004
    pub dwLayer         : u32,              // 0x0008
    pub dwSizeX         : [u32; 3],         // 0x000c
    pub dwSizeY         : [u32; 3],         // 0x0018
    pub dwOffsetX       : u32,              // 0x0024
    pub dwOffsetY       : u32,              // 0x0028
    pub dwDepend        : u32,              // 0x002c
    pub dwDrlgType      : u32,              // 0x0030
    pub dwLevelType     : u32,              // 0x0034
    pub dwSubType       : u32,              // 0x0038
    pub dwSubTheme      : u32,              // 0x003c
    pub dwSubWaypoint   : u32,              // 0x0040
    pub dwSubShrine     : u32,              // 0x0044
    pub dwVis           : [i32; 8],         // 0x0048
    pub dwWarp          : [u32; 8],         // 0x0068
    pub nIntensity      : u8,               // 0x0088
    pub nRed            : u8,               // 0x0089
    pub nGreen          : u8,               // 0x008a
    pub nBlue           : u8,               // 0x008b
    pub dwPortal        : u32,              // 0x008c
    pub dwPosition      : u32,              // 0x0090
    pub dwSaveMonsters  : u32,              // 0x0094
    pub dwLOSDraw       : u32,              // 0x0098
}

#[repr(C, packed(1))]
pub struct D2ObjectsTxt {
    pub szName          : [u8; 0x40],     // 0x0000
    pub wszName         : [u16; 0x40],    // 0x0040
    pub szToken         : [u8; 3],        // 0x00c0
    pub nSpawnMax       : u8,             // 0x00c3
    pub nSelectable     : [u8; 8],        // 0x00c4
    pub nTrapProb       : u8,             // 0x00cc
        pad0xCD         : [u8; 3],        // 0x00cd
    pub dwSizeX         : i32,            // 0x00d0
    pub dwSizeY         : i32,            // 0x00d4
    pub dwFrameCnt      : [u32; 8],       // 0x00d8
    pub wFrameDelta     : [u16; 8],       // 0x00f8
    pub nCycleAnim      : [u8; 8],        // 0x0108
    pub nLit            : [u8; 8],        // 0x0110
    pub nBlocksLight    : [u8; 8],        // 0x0118
    pub nHasCollision   : [u8; 8],        // 0x0120
    pub nIsAttackable0  : u8,             // 0x0128
    pub nStart          : [u8; 8],        // 0x0129
    pub nOrderFlag      : [u8; 8],        // 0x0131
    pub nEnvEffect      : u8,             // 0x0139
    pub nIsDoor         : u8,             // 0x013a
    pub nBlocksVis      : u8,             // 0x013b
    pub nOrientation    : u8,             // 0x013c
    pub nPreOperate     : u8,             // 0x013d
    pub nTrans          : u8,             // 0x013e
    pub nMode           : [u8; 8],        // 0x013f
        pad0x147        : u8,             // 0x0147
    pub dwXOffset       : i32,            // 0x0148
    pub dwYOffset       : i32,            // 0x014c
    pub nDraw           : u8,             // 0x0150
    pub nHD             : u8,             // 0x0151
    pub nTR             : u8,             // 0x0152
    pub nLG             : u8,             // 0x0153
    pub nRA             : u8,             // 0x0154
    pub nLA             : u8,             // 0x0155
    pub nRH             : u8,             // 0x0156
    pub nLH             : u8,             // 0x0157
    pub nSH             : u8,             // 0x0158
    pub nS1             : u8,             // 0x0159
    pub nS2             : u8,             // 0x015a
    pub nS3             : u8,             // 0x015b
    pub nS4             : u8,             // 0x015c
    pub nS5             : u8,             // 0x015d
    pub nS6             : u8,             // 0x015e
    pub nS7             : u8,             // 0x015f
    pub nS8             : u8,             // 0x0160
    pub nTotalPieces    : u8,             // 0x0161
    pub nXSpace         : u8,             // 0x0162
    pub nYSpace         : u8,             // 0x0163
    pub nRed            : u8,             // 0x0164
    pub nGreen          : u8,             // 0x0165
    pub nBlue           : u8,             // 0x0166
    pub nSubClass       : u8,             // 0x0167
    pub dwNameOffset    : u32,            // 0x0168
        pad0x16C        : u8,             // 0x016c
    pub nMonsterOK      : u8,             // 0x016d
    pub nOperateRange   : u8,             // 0x016e
    pub nShrineFunction : u8,             // 0x016f
    pub nAct            : u8,             // 0x0170
    pub nLockable       : u8,             // 0x0171
    pub nGore           : u8,             // 0x0172
    pub nRestore        : u8,             // 0x0173
    pub nRestoreVirgins : u8,             // 0x0174
    pub nSync           : u8,             // 0x0175
        pad0x176        : u16,            // 0x0176
    pub dwParm          : [u32; 8],       // 0x0178
    pub nTgtFX          : u8,             // 0x0198
    pub nTgtFY          : u8,             // 0x0199
    pub nTgtBX          : u8,             // 0x019a
    pub nTgtBY          : u8,             // 0x019b
    pub nDamage         : u8,             // 0x019c
    pub nCollisionSubst : u8,             // 0x019d
        pad0x19E        : u16,            // 0x019e
    pub dwLeft          : u32,            // 0x01a0
    pub dwTop           : u32,            // 0x01a4
    pub dwWidth         : u32,            // 0x01a8
    pub dwHeight        : u32,            // 0x01ac
    pub nBeta           : u8,             // 0x01b0
    pub nInitFn         : u8,             // 0x01b1
    pub nPopulateFn     : u8,             // 0x01b2
    pub nOperateFn      : u8,             // 0x01b3
    pub nClientFn       : u8,             // 0x01b4
    pub nOverlay        : u8,             // 0x01b5
    pub nBlockMissile   : u8,             // 0x01b6
    pub nDrawUnder      : u8,             // 0x01b7
    pub nOpenWarp       : u8,             // 0x01b8
        pad0x1B9        : [u8; 3],        // 0x01b9
    pub dwAutomap       : u32,            // 0x01bc
}

#[repr(C, packed(1))]
pub struct D2LvlWarpTxt {
    pub dwLevelId       : u32,              //0x00
    pub dwSelectX       : u32,              //0x04
    pub dwSelectY       : u32,              //0x08
    pub dwSelectDX      : u32,              //0x0C
    pub dwSelectDY      : u32,              //0x10
    pub dwExitWalkX     : u32,              //0x14
    pub dwExitWalkY     : u32,              //0x18
    pub dwOffsetX       : u32,              //0x1C
    pub dwOffsetY       : u32,              //0x20
    pub dwLitVersion    : u32,              //0x24
    pub dwTiles         : u32,              //0x28
    pub szDirection     : [i8; 4],          //0x2C
}

#[repr(C, packed(4))]
pub struct D2MonStatsTxt
{
    pub nId                         : i16,                  // 0x0000
    pub nBaseId                     : i16,                  // 0x0002
    pub nNextInClass                : i16,                  // 0x0004
    pub wNameStr                    : u16,                  // 0x0006
    pub wDescStr                    : u16,                  // 0x0008
    pub unk0x0A                     : u16,                  // 0x000a
    pub dwMonStatsFlags             : D2MonStatsTxtFlags,   // 0x000c
    pub dwCode                      : u32,                  // 0x0010
    pub wMonSound                   : u16,                  // 0x0014
    pub wUMonSound                  : u16,                  // 0x0016
    pub wMonStatsEx                 : u16,                  // 0x0018
    pub wMonProp                    : u16,                  // 0x001a
    pub wMonType                    : i16,                  // 0x001c
    pub wAI                         : u16,                  // 0x001e
    pub wSpawn                      : u16,                  // 0x0020
    pub nSpawnX                     : u8,                   // 0x0022
    pub nSpawnY                     : u8,                   // 0x0023
    pub nSpawnMode                  : u8,                   // 0x0024
    pub unk0x25                     : u8,                   // 0x0025
    pub wMinion                     : [i16; 2],             // 0x0026
    pub nMonEquipTxtRecordId        : i16,                  // 0x002a
    pub nPartyMin                   : u8,                   // 0x002c
    pub nPartyMax                   : u8,                   // 0x002d
    pub nRarity                     : u8,                   // 0x002e
    pub nMinGrp                     : u8,                   // 0x002f
    pub nMaxGrp                     : u8,                   // 0x0030
    pub nSparsePopulate             : u8,                   // 0x0031
    pub nVelocity                   : i16,                  // 0x0032
    pub nRun                        : i16,                  // 0x0034
    pub nWalkAnimSpeed              : i16,                  // 0x0036
    pub nRunAnimSpeed               : i16,                  // 0x0038
    pub wMissA1                     : u16,                  // 0x003a
    pub wMissA2                     : u16,                  // 0x003c
    pub wMissS1                     : u16,                  // 0x003e
    pub wMissS2                     : u16,                  // 0x0040
    pub wMissS3                     : u16,                  // 0x0042
    pub wMissS4                     : u16,                  // 0x0044
    pub wMissC                      : u16,                  // 0x0046
    pub wMissSQ                     : u16,                  // 0x0048
    pub nMaxChainId                 : u8,                   // 0x004a
    pub nChainId                    : u8,                   // 0x004b
    pub nAlign                      : u8,                   // 0x004c
    pub nTransLvl                   : u8,                   // 0x004d
    pub nThreat                     : u8,                   // 0x004e
    pub nAIdel                      : [u8; 3],              // 0x004f
    pub nAiDist                     : [u8; 3],              // 0x0052
    pub unk0x55                     : u8,                   // 0x0055
    pub wAiParam                    : [[i16; 8]; 3],        // 0x0056
    pub wTreasureClass              : [[u16; 3]; 4],        // 0x0086
    pub nTCQuestId                  : u8,                   // 0x009e
    pub nTCQuestCP                  : u8,                   // 0x009f
    pub nDrain                      : [u8; 3],              // 0x00a0
    pub nToBlock                    : [u8; 3],              // 0x00a3
    pub nCrit                       : u8,                   // 0x00a6
    pub unk0xA7                     : u8,                   // 0x00a7
    pub wSkillDamage                : i16,                  // 0x00a8
    pub nLevel                      : [u16; 3],             // 0x00aa
    pub nMinHP                      : [u16; 3],             // 0x00b0
    pub nMaxHP                      : [u16; 3],             // 0x00b6
    pub nAC                         : [u16; 3],             // 0x00bc
    pub nA1TH                       : [u16; 3],             // 0x00c2
    pub nA2TH                       : [u16; 3],             // 0x00c8
    pub nS1TH                       : [u16; 3],             // 0x00ce
    pub nExp                        : [u16; 3],             // 0x00d4
    pub nA1MinD                     : [u16; 3],             // 0x00da
    pub nA1MaxD                     : [u16; 3],             // 0x00e0
    pub nA2MinD                     : [u16; 3],             // 0x00e6
    pub nA2MaxD                     : [u16; 3],             // 0x00ec
    pub nS1MinD                     : [u16; 3],             // 0x00f2
    pub nS1MaxD                     : [u16; 3],             // 0x00f8
    pub nElMode                     : [u8; 3],              // 0x00fe
    pub nElType                     : [u8; 3],              // 0x0101
    pub nElPct                      : [[u8; 3]; 3],         // 0x0104
    pub unk0x10D                    : u8,                   // 0x010d
    pub nElMinD                     : [[u16; 3]; 3],        // 0x010e
    pub nElMaxD                     : [[u16; 3]; 3],        // 0x0120
    pub nElDur                      : [[u16; 3]; 3],        // 0x0132
    pub nResistances                : [[u16; 6]; 3],        // 0x0144
    pub nColdEffect                 : [i8; 3],              // 0x0168
    pub unk0x16B                    : u8,                   // 0x016b
    pub nSendSkills                 : [u8; 4],              // 0x016c
    pub nSkill                      : [i16; 8],             // 0x0170
    pub nSkillMode                  : [u8; 8],              // 0x0180
    pub nSequence                   : [u16; 8],             // 0x0188
    pub nSkLvl                      : [u8; 8],              // 0x0198
    pub dwDamageRegen               : u32,                  // 0x01a0
    pub nSplEndDeath                : u8,                   // 0x01a4
    pub nSplGetModeChart            : u8,                   // 0x01a5
    pub nSplEndGeneric              : u8,                   // 0x01a6
    pub nSplClientEnd               : u8,                   // 0x01a7
}

#[repr(C, packed(1))]
pub struct D2LevelsTxt {
    pub wLevelNo        : u16,          //0x00
    pub nPal            : u8,           //0x02
    pub nAct            : u8,           //0x03
    pub nTeleport       : u8,           //0x04
    pub nRain           : u8,           //0x05
    pub nMud            : u8,           //0x06
    pub nNoPer          : u8,           //0x07
    pub nIsInside       : u8,           //0x08
    pub nDrawEdges      : u8,           //0x09
    pub unk0x0A         : u16,          //0x0A
    pub dwWarpDist      : u32,          //0x0C
    pub wMonLvl         : [u16; 3],     //0x10
    pub wMonLvlEx       : [u16; 3],     //0x16
    pub dwMonDen        : [u32; 3],     //0x1C
    pub nMonUMin        : [u8; 3],      //0x28
    pub nMonUMax        : [u8; 3],      //0x2B
    pub nMonWndr        : u8,           //0x2E
    pub nMonSpcWalk     : u8,           //0x2F
    pub nQuest          : u8,           //0x30
    pub nRangedSpawn    : u8,           //0x31
    pub nNumMon         : u8,           //0x32
    pub nNumNormMon     : u8,           //0x33
    pub nNumNMon        : u8,           //0x34
    pub nNumUMon        : u8,           //0x35
    pub wMon            : [i16; 25],    //0x36
    pub wNMon           : [i16; 25],    //0x68
    pub wUMon           : [i16; 25],    //0x9A
    pub wCMon           : [u16; 4],     //0xCC
    pub wCPct           : [u16; 4],     //0xD4
    pub wCAmt           : [u16; 4],     //0xDC
    pub nWaypoint       : u8,           //0xE4
    pub nObjGroup       : [u8; 8],      //0xE5
    pub nObjPrb         : [u8; 8],      //0xED
    pub szLevelName     : [i8; 40],     //0xF5
    pub szLevelWarp     : [i8; 40],     //0x11D
    pub szEntryFile     : [i8; 40],     //0x145
    pub pad0x16D        : u8,           //0x16D
    pub wszLevelName    : [u16; 40],    //0x16E
    pub wszLevelWarp    : [u16; 40],    //0x1BE
    pub pad0x20E        : u16,          //0x20E
    pub dwThemes        : u32,          //0x210
    pub dwFloorFilter   : u32,          //0x214
    pub dwBlankScreen   : u32,          //0x218
    pub dwSoundEnv      : u32,          //0x21C
}

impl D2LevelsTxt {
    pub fn get_level_name(&self) -> String {
        unsafe {
            String::from_utf16_lossy(std::slice::from_raw_parts(addr_of!(self.wszLevelName) as *const u16, 40)).trim_end_matches('\0').to_string()
        }
    }

    pub fn get_level_name_ptr(&self) -> *const u16 {
        addr_of!(self.wszLevelName) as *const u16
    }
}
