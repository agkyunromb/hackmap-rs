use bitflags::bitflags;

#[repr(u8)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum D2StringColorCodes {
    White = 0,
    Red = 1,
    LightGreen = 2,
    Blue = 3,
    DarkGold = 4,
    Grey = 5,
    Black = 6,
    Tan = 7,
    Orange = 8,
    Yellow = 9,
    DarkGreen = 10,
    Purple = 11,
    DarkGreen2 = 12,
    Invalid = 255,
}

impl D2StringColorCodes {
    pub fn as_str_code(&self) -> &'static str {
        // *self as u8 + 0x30

        match self {
            Self::White => "ÿc0",
            Self::Red => "ÿc1",
            Self::LightGreen => "ÿc2",
            Self::Blue => "ÿc3",
            Self::DarkGold => "ÿc4",
            Self::Grey => "ÿc5",
            Self::Black => "ÿc6",
            Self::Tan => "ÿc7",
            Self::Orange => "ÿc8",
            Self::Yellow => "ÿc9",
            Self::DarkGreen => "ÿc:",
            Self::Purple => "ÿc;",
            Self::DarkGreen2 => "ÿc<",
            Self::Invalid => "",
        }
    }
}

#[repr(i32)]
#[derive(PartialEq, Copy, Clone)]
pub enum D2UIvars {
    Game = 0x00,         // 0
    Inventory = 0x01,    // 1
    StatScreen = 0x02,   // 2
    MiniSkill = 0x03,    // 3
    SkillTree = 0x04,    // 4
    ChatBox = 0x05,      // 5
    NewStats = 0x06,     // 6
    NewSkills = 0x07,    // 7
    NpcMenu = 0x08,      // 8
    EscMenu = 0x09,      // 9
    AutoMap = 0x0A,      // 10
    Config = 0x0B,       // 11
    NpcShop = 0x0C,      // 12
    HoldAlt = 0x0D,      // 13
    Anvil = 0x0E,        // 14
    QuestScreen = 0x0F,  // 15
    IniScroll = 0x10,    // 16
    QuestLog = 0x11,     // 17
    Unknown18 = 0x12,    // 18
    HiRicons = 0x13,     // 19
    Waypoint = 0x14,     // 20
    MiniPanel = 0x15,    // 21
    PartyScreen = 0x16,  // 22
    MpTrade = 0x17,      // 23
    MsgLog = 0x18,       // 24
    Stash = 0x19,        // 25
    Cube = 0x1A,         // 26
    SteegStone = 0x1B,   // 27
    GuildVault = 0x1C,   // 28
    Unknown29 = 0x1D,    // 29
    Unknown30 = 0x1E,    // 30
    BeltRows = 0x1F,     // 31
    Unknown32 = 0x20,    // 32
    HelpScreen = 0x21,   // 33
    HelpButton = 0x22,   // 34
    HireIcons = 0x23,    // 35
    MercInv = 0x24,      // 36
    RecipeScroll = 0x25, // 37
}

#[repr(i32)]
#[derive(PartialEq, Copy, Clone)]
pub enum D2ControlTypes {
    Editbox = 1,
    Image = 2,
    Animimage = 3,
    Textbox = 4,
    Scrollbar = 5,
    Button = 6,
    List = 7,
    Timer = 8,
    Smack = 9,
    Progressbar = 10,
    Popup = 11,
    Accountlist = 12,
    Image2 = 13,
}

#[repr(i32)]
#[derive(PartialEq, Copy, Clone)]
pub enum D2ItemStats {
    Invalid = -1,
    Strength = 0,
    Energy = 1,
    Dexterity = 2,
    Vitality = 3,
    StatPts = 4,
    SkillPts = 5,
    HitPoints = 6,
    MaxHp = 7,
    Mana = 8,
    MaxMana = 9,
    Stamina = 10,
    MaxStamina = 11,
    Level = 12,
    Experience = 13,
    Gold = 14,
    GoldBank = 15,
    Item_Armor_Percent = 16,
    Item_MaxDamage_Percent = 17,
    Item_MinDamage_Percent = 18,
    ToHit = 19,
    ToBlock = 20,
    MinDamage = 21,
    MaxDamage = 22,
    Secondary_MinDamage = 23,
    Secondary_MaxDamage = 24,
    DamagePercent = 25,
    ManaRecovery = 26,
    ManaRecoveryBonus = 27,
    StaminaRecoveryBonus = 28,
    LastExp = 29,
    NextExp = 30,
    ArmorClass = 31,
    ArmorClass_Vs_Missile = 32,
    ArmorClass_Vs_Hth = 33,
    Normal_Damage_Reduction = 34,
    Magic_Damage_Reduction = 35,
    DamageResist = 36,
    MagicResist = 37,
    MaxMagicResist = 38,
    FireResist = 39,
    MaxFireResist = 40,
    LightResist = 41,
    MaxLightResist = 42,
    ColdResist = 43,
    MaxColdResist = 44,
    PoisonResist = 45,
    MaxPoisonResist = 46,
    DamageAura = 47,
    FireMinDam = 48,
    FireMaxDam = 49,
    LightMinDam = 50,
    LightMaxDam = 51,
    MagicMinDam = 52,
    MagicMaxDam = 53,
    ColdMinDam = 54,
    ColdMaxDam = 55,
    ColdLength = 56,
    PoisonMinDam = 57,
    PoisonMaxDam = 58,
    PoisonLength = 59,
    LifeDrainMinDam = 60,
    LifeDrainMaxDam = 61,
    ManaDrainMinDam = 62,
    ManaDrainMaxDam = 63,
    StamDrainMinDam = 64,
    StamDrainMaxDam = 65,
    StunLength = 66,
    VelocityPercent = 67,
    AttackRate = 68,
    Other_AnimRate = 69,
    Quantity = 70,
    Value = 71,
    Durability = 72,
    MaxDurability = 73,
    HpRegen = 74,
    Item_MaxDurability_Percent = 75,
    Item_MaxHp_Percent = 76,
    Item_MaxMana_Percent = 77,
    Item_AttackerTakesDamage = 78,
    Item_GoldBonus = 79,
    Item_MagicBonus = 80,
    Item_Knockback = 81,
    Item_TimeDuration = 82,
    Item_AddClassSkills = 83,
    UnsentParam1 = 84,
    Item_AddExperience = 85,
    Item_HealAfterKill = 86,
    Item_ReducedPrices = 87,
    Item_DoubleHerbDuration = 88,
    Item_LightRadius = 89,
    Item_LightColor = 90,
    Item_Req_Percent = 91,
    Item_LevelReq = 92,
    Item_FasterAttackRate = 93,
    Item_LevelReqPct = 94,
    LastBlockFrame = 95,
    Item_FasterMoveVelocity = 96,
    Item_NonClassSkill = 97,
    State = 98,
    Item_FasterGetHitRate = 99,
    Monster_PlayerCount = 100,
    Skill_Poison_Override_Length = 101,
    Item_FasterBlockRate = 102,
    Skill_Bypass_Undead = 103,
    Skill_Bypass_Demons = 104,
    Item_FasterCastRate = 105,
    Skill_Bypass_Beasts = 106,
    Item_SingleSkill = 107,
    Item_RestInPeace = 108,
    Curse_Resistance = 109,
    Item_PoisonLengthResist = 110,
    Item_NormalDamage = 111,
    Item_Howl = 112,
    Item_Stupidity = 113,
    Item_DamageToMana = 114,
    Item_IgnoreTargetAc = 115,
    Item_FractionalTargetAc = 116,
    Item_PreventHeal = 117,
    Item_HalfFreezeDuration = 118,
    Item_ToHit_Percent = 119,
    Item_DamageTargetAc = 120,
    Item_DemonDamage_Percent = 121,
    Item_UndeadDamage_Percent = 122,
    Item_Demon_ToHit = 123,
    Item_Undead_ToHit = 124,
    Item_Throwable = 125,
    Item_ElemSkill = 126,
    Item_AllSkills = 127,
    Item_AttackerTakesLightDamage = 128,
    IronMaiden_Level = 129,
    LifeTap_Level = 130,
    Thorns_Percent = 131,
    BoneArmor = 132,
    BoneArmorMax = 133,
    Item_Freeze = 134,
    Item_OpenWounds = 135,
    Item_CrushingBlow = 136,
    Item_KickDamage = 137,
    Item_ManaAfterKill = 138,
    Item_HealAfterDemonKill = 139,
    Item_ExtraBlood = 140,
    Item_DeadlyStrike = 141,
    Item_AbsorbFire_Percent = 142,
    Item_AbsorbFire = 143,
    Item_AbsorbLight_Percent = 144,
    Item_AbsorbLight = 145,
    Item_AbsorbMagic_Percent = 146,
    Item_AbsorbMagic = 147,
    Item_AbsorbCold_Percent = 148,
    Item_AbsorbCold = 149,
    Item_Slow = 150,
    Item_Aura = 151,
    Item_Indesctructible = 152,
    Item_CannotBeFrozen = 153,
    Item_StaminaDrainPct = 154,
    Item_Reanimate = 155,
    Item_Pierce = 156,
    Item_MagicArrow = 157,
    Item_ExplosiveArrow = 158,
    Item_Throw_MinDamage = 159,
    Item_Throw_MaxDamage = 160,
    Skill_HandOfAthena = 161,
    Skill_StaminaPercent = 162,
    Skill_Passive_StaminaPercent = 163,
    Skill_Concentration = 164,
    Skill_Enchant = 165,
    Skill_Pierce = 166,
    Skill_Conviction = 167,
    Skill_ChillingArmor = 168,
    Skill_Frenzy = 169,
    Skill_Decrepify = 170,
    Skill_Armor_Percent = 171,
    Alignment = 172,
    Target0 = 173,
    Target1 = 174,
    GoldLost = 175,
    Conversion_Level = 176,
    Conversion_MaxHp = 177,
    Unit_DoOverlay = 178,
    Attack_Vs_MonType = 179,
    Damage_Vs_MonType = 180,
    Fade = 181,
    Armor_Override_Percent = 182,
    Unused183 = 183,
    Unused184 = 184,
    Unused185 = 185,
    Unused186 = 186,
    Unused187 = 187,
    Item_AddSkill_Tab = 188,
    Unused189 = 189,
    Unused190 = 190,
    Unused191 = 191,
    Unused192 = 192,
    Unused193 = 193,
    Item_NumSockets = 194,
    Item_SkillOnAttack = 195,
    Item_SkillOnKill = 196,
    Item_SkillOnDeath = 197,
    Item_SkillOnHit = 198,
    Item_SkillOnLevelUp = 199,
    Unused200 = 200,
    Item_SkillOnGetHit = 201,
    Unused202 = 202,
    Unused203 = 203,
    Item_Charged_Skill = 204,
    Unused204 = 205,
    Unused205 = 206,
    Unused206 = 207,
    Unused207 = 208,
    Unused208 = 209,
    Unused209 = 210,
    Unused210 = 211,
    Unused211 = 212,
    Unused212 = 213,
    Item_Armor_PerLevel = 214,
    Item_ArmorPercent_PerLevel = 215,
    Item_Hp_PerLevel = 216,
    Item_Mana_PerLevel = 217,
    Item_MaxDamage_PerLevel = 218,
    Item_MaxDamage_Percent_PerLevel = 219,
    Item_Strength_PerLevel = 220,
    Item_Dexterity_PerLevel = 221,
    Item_Energy_PerLevel = 222,
    Item_Vitality_PerLevel = 223,
    Item_ToHit_PerLevel = 224,
    Item_ToHitPercent_PerLevel = 225,
    Item_Cold_DamageMax_PerLevel = 226,
    Item_Fire_DamageMax_PerLevel = 227,
    Item_Ltng_DamageMax_PerLevel = 228,
    Item_Pois_DamageMax_PerLevel = 229,
    Item_Resist_Cold_PerLevel = 230,
    Item_Resist_Fire_PerLevel = 231,
    Item_Resist_Ltng_PerLevel = 232,
    Item_Resist_Pois_PerLevel = 233,
    Item_Absorb_Cold_PerLevel = 234,
    Item_Absorb_Fire_PerLevel = 235,
    Item_Absorb_Ltng_PerLevel = 236,
    Item_Absorb_Pois_PerLevel = 237,
    Item_Thorns_PerLevel = 238,
    Item_Find_Gold_PerLevel = 239,
    Item_Find_Magic_PerLevel = 240,
    Item_RegenStamina_PerLevel = 241,
    Item_Stamina_PerLevel = 242,
    Item_Damage_Demon_PerLevel = 243,
    Item_Damage_Undead_PerLevel = 244,
    Item_ToHit_Demon_PerLevel = 245,
    Item_ToHit_Undead_PerLevel = 246,
    Item_CrushingBlow_PerLevel = 247,
    Item_OpenWounds_PerLevel = 248,
    Item_Kick_Damage_PerLevel = 249,
    Item_DeadlyStrike_PerLevel = 250,
    Item_Find_Gems_PerLevel = 251,
    Item_Replenish_Durability = 252,
    Item_Replenish_Quantity = 253,
    Item_Extra_Stack = 254,
    Item_Find_Item = 255,
    Item_Slash_Damage = 256,
    Item_Slash_Damage_Percent = 257,
    Item_Crush_Damage = 258,
    Item_Crush_Damage_Percent = 259,
    Item_Thrust_Damage = 260,
    Item_Thrust_Damage_Percent = 261,
    Item_Absorb_Slash = 262,
    Item_Absorb_Crush = 263,
    Item_Absorb_Thrust = 264,
    Item_Absorb_Slash_Percent = 265,
    Item_Absorb_Crush_Percent = 266,
    Item_Absorb_Thrust_Percent = 267,
    Item_Armor_ByTime = 268,
    Item_ArmorPercent_ByTime = 269,
    Item_Hp_ByTime = 270,
    Item_Mana_ByTime = 271,
    Item_MaxDamage_ByTime = 272,
    Item_MaxDamage_Percent_ByTime = 273,
    Item_Strength_ByTime = 274,
    Item_Dexterity_ByTime = 275,
    Item_Energy_ByTime = 276,
    Item_Vitality_ByTime = 277,
    Item_ToHit_ByTime = 278,
    Item_ToHitPercent_ByTime = 279,
    Item_Cold_DamageMax_ByTime = 280,
    Item_Fire_DamageMax_ByTime = 281,
    Item_Ltng_DamageMax_ByTime = 282,
    Item_Pois_DamageMax_ByTime = 283,
    Item_Resist_Cold_ByTime = 284,
    Item_Resist_Fire_ByTime = 285,
    Item_Resist_Ltng_ByTime = 286,
    Item_Resist_Pois_ByTime = 287,
    Item_Absorb_Cold_ByTime = 288,
    Item_Absorb_Fire_ByTime = 289,
    Item_Absorb_Ltng_ByTime = 290,
    Item_Absorb_Pois_ByTime = 291,
    Item_Find_Gold_ByTime = 292,
    Item_Find_Magic_ByTime = 293,
    Item_RegenStamina_ByTime = 294,
    Item_Stamina_ByTime = 295,
    Item_Damage_Demon_ByTime = 296,
    Item_Damage_Undead_ByTime = 297,
    Item_ToHit_Demon_ByTime = 298,
    Item_ToHit_Undead_ByTime = 299,
    Item_CrushingBlow_ByTime = 300,
    Item_OpenWounds_ByTime = 301,
    Item_Kick_Damage_ByTime = 302,
    Item_DeadlyStrike_ByTime = 303,
    Item_Find_Gems_ByTime = 304,
    Item_Pierce_Cold = 305,
    Item_Pierce_Fire = 306,
    Item_Pierce_Ltng = 307,
    Item_Pierce_Pois = 308,
    Item_Damage_Vs_Monster = 309,
    Item_Damage_Percent_Vs_Monster = 310,
    Item_ToHit_Vs_Monster = 311,
    Item_ToHit_Percent_Vs_Monster = 312,
    Item_Ac_Vs_Monster = 313,
    Item_Ac_Percent_Vs_Monster = 314,
    FireLength = 315,
    BurningMin = 316,
    BurningMax = 317,
    Progressive_Damage = 318,
    Progressive_Steal = 319,
    Progressive_Other = 320,
    Progressive_Fire = 321,
    Progressive_Cold = 322,
    Progressive_Lightning = 323,
    Item_Extra_Charges = 324,
    Progressive_ToHit = 325,
    Poison_Count = 326,
    Damage_FrameRate = 327,
    Pierce_Idx = 328,
    Passive_Fire_Mastery = 329,
    Passive_Ltng_Mastery = 330,
    Passive_Cold_Mastery = 331,
    Passive_Pois_Mastery = 332,
    Passive_Fire_Pierce = 333,
    Passive_Ltng_Pierce = 334,
    Passive_Cold_Pierce = 335,
    Passive_Pois_Pierce = 336,
    Passive_Critical_Strike = 337,
    Passive_Dodge = 338,
    Passive_Avoid = 339,
    Passive_Evade = 340,
    Passive_Warmth = 341,
    Passive_Mastery_Melee_Th = 342,
    Passive_Mastery_Melee_Dmg = 343,
    Passive_Mastery_Melee_Crit = 344,
    Passive_Mastery_Throw_Th = 345,
    Passive_Mastery_Throw_Dmg = 346,
    Passive_Mastery_Throw_Crit = 347,
    Passive_WeaponBlock = 348,
    Passive_Summon_Resist = 349,
    ModifierList_Skill = 350,
    ModifierList_Level = 351,
    Last_Sent_Hp_Pct = 352,
    Source_Unit_Type = 353,
    Source_Unit_Id = 354,
    ShortParam1 = 355,
    QuestItemDifficulty = 356,
    Passive_Mag_Mastery = 357,
    Passive_Mag_Pierce = 358,
}

#[repr(i32)]
#[derive(PartialEq, Copy, Clone)]
pub enum D2DrawMode {
    Trans25 = 0,
    Trans50 = 1,
    Trans75 = 2,
    Modulate = 3,
    Burn = 4,
    Normal = 5,
    TransHighLight = 6,
    HighLight = 7,
}

#[repr(i32)]
#[derive(PartialEq, Copy, Clone)]
pub enum D2DrlgTypes {
    Maze = 1,
    Preset = 2,
    Outdoor = 3,
    Max = 4,
}

#[repr(i32)]
#[derive(PartialEq, Copy, Clone)]
pub enum D2UnitTypes {
    Player = 0,
    Monster = 1,
    Object = 2,
    Missile = 3,
    Item = 4,
    Tile = 5,
    Max = 6,
}

#[repr(u32)]
#[derive(PartialEq, Copy, Clone)]
pub enum D2AutoMapCells {
    RedCross = 221,
    Hammer = 302,
    CainCage = 303,
    MephOrb = 305,
    DiabloSeal = 306,
    WayPoint = 307,
    Well = 309,
    Shrine = 310,
    FallcampFlag = 312,
    IniTree = 313,
    CainRock = 314,
    Gidbinn = 315, // 吉得宾
    QHammer = 316,
    BlueCross = 317,
    QChest = 318,
    Stash = 319,
    ArcanePortal = 339,
    RogueFire = 405,
    Book = 427,
    PlaceHolder = 1176,
    BarriTower = 1258,
}

#[repr(u32)]
#[derive(PartialEq, Copy, Clone)]
pub enum D2Font {
    Font8 = 0,
    Font16 = 1,
    Font30 = 2,
    Font42 = 3,
    FontFormal10 = 4,
    FontFormal12 = 5,
    Font6 = 6,
    Font24 = 7,
    FontFormal11 = 8,
    FontExocet10 = 9,
    FontRidiculous = 10,
    FontExocet8 = 11,
    ReallyTheLastSucker = 12,
    FontInGameChat = 13,

    Max,
}

#[repr(u32)]
#[derive(PartialEq, Copy, Clone)]
pub enum D2PlayerModes {
    Death = 0,
    Neutral = 1,
    Walk = 2,
    Run = 3,
    GetHit = 4,
    TNeutral = 5,
    TWalk = 6,
    Attack1 = 7,
    Attack2 = 8,
    Block = 9,
    Cast = 10,
    Throw = 11,
    Kick = 12,
    Special1 = 13,
    Special2 = 14,
    Special3 = 15,
    Special4 = 16,
    Dead = 17,
    Sequence = 18,
    Knockback = 19,

    Max,
}

#[repr(u32)]
#[derive(PartialEq, Copy, Clone)]
pub enum D2MonModes {
    Death = 0,      // 0x00 Death DT
    Neutral = 1,    // 0x01 Neutral NU
    Walk = 2,       // 0x02 Walk WL
    GetHit = 3,     // 0x03 Get Hit GH
    Attack1 = 4,    // 0x04 Melee Attack A1
    Attack2 = 5,    // 0x05 Melee Attack A2
    Block = 6,      // 0x06 Block BL
    Cast = 7,       // 0x07 Spell Cast SC
    Skill1 = 8,     // 0x08 Special S1
    Skill2 = 9,     // 0x09 Special S2
    Skill3 = 10,    // 0x0A Special S3
    Skill4 = 11,    // 0x0B Special S4
    Dead = 12,      // 0x0C Dead DD
    Knockback = 13, // 0x0D Knockback KB
    Sequence = 14,  // 0x0E Sequence SQ
    Run = 15,       // 0x0F Run RN

    Max,
}

#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum D2MonUMods {
    None = 0,
    RndName = 1,
    HpMultiply = 2,
    Light = 3,
    LevelAdd = 4,
    ExtraStrong = 5,
    ExtraFast = 6,
    Cursed = 7,
    MagicResistant = 8,
    FireChant = 9,
    PoisDeath = 10,
    WormDeath = 11,
    BravEnDeath = 12,
    IgnoreAc = 13,
    SpcDamage = 14,
    KillMinionsDeath = 15,
    ChampMods = 16,
    LightChant = 17,
    ColdChant = 18,
    UnusedMercMod = 19,
    ChargedBolts = 20,
    TempSummon = 21,
    QuestMod = 22,
    PoisonField = 23,
    Thief = 24,
    ManaBurn = 25,
    TeleHeal = 26,
    SpectralHit = 27,
    StoneSkin = 28,
    MultiShot = 29,
    AuraChant = 30,
    CorpseBoomDeath = 31,
    FireBoomDeath = 32,
    FreezinDeath = 33,
    SelfResurrect = 34,
    IceShatterDeath = 35,
    ChampStoned = 36,
    ChampStats = 37,
    ChampCurseImmune = 38,
    ChampStats2 = 39,
    PainWormDeath = 40,
    AlwaysRunAi = 41,
    NovaDeath = 42,
}

#[allow(clippy::upper_case_acronyms)]
#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum D2ClientCmd {
    WALK_TO_LOCATION = 0x01,
    WALK_TO_ENTITY = 0x02,
    RUN_TO_LOCATION = 0x03,
    RUN_TO_ENTITY = 0x04,
    LEFT_SKILL_ON_LOCATION = 0x05,
    LEFT_SKILL_ON_ENTITY = 0x06,
    LEFT_SKILL_ON_ENTITYEX = 0x07,
    LEFT_SKILL_ON_LOCATIONEX = 0x08,
    LEFT_SKILL_ON_ENTITYEX2 = 0x09,
    LEFT_SKILL_ON_ENTITYEX3 = 0x0A,
    RIGHT_SKILL_ON_LOCATION = 0x0C,
    RIGHT_SKILL_ON_ENTITY = 0x0D,
    RIGHT_SKILL_ON_ENTITYEX = 0x0E,
    RIGHT_SKILL_ON_LOCATIONEX = 0x0F,
    RIGHT_SKILL_ON_ENTITYEX2 = 0x10,
    RIGHT_SKILL_ON_ENTITYEX3 = 0x11,
    SET_INFERNO_STATE = 0x12,
    INTERACT_WITH_ENTITY = 0x13,
    OVERHEAD_MESSAGE = 0x14,
    CHAT = 0x15,
    PICKUP_ITEM = 0x16,
    DROP_ITEM = 0x17,
    ITEM_TO_BUFFER = 0x18,
    PICKUP_BUFFER_ITEM = 0x19,
    ITEM_TO_BODY = 0x1A,
    SWAP_2_HANDED_ITEM = 0x1B,
    PICKUP_BODY_ITEM = 0x1C,
    SWITCH_BODY_ITEM = 0x1D,
    SWITCH_1H_2H = 0x1E,
    SWITCHINVENTORYITEM = 0x1F,
    USE_ITEM = 0x20,
    STACKI_TEM = 0x21,
    REMOVE_STACK_ITEM = 0x22,
    ITEM_TO_BELT_LOCATION = 0x23,
    REMOVE_BELT_ITEM = 0x24,
    SWITCH_BELT_ITEM = 0x25,
    USE_BELT_ITEM = 0x26,
    IDENTIFY_ITEM = 0x27,
    INSERT_SOCKET_ITEM = 0x28,
    SCROLL_TO_TOME = 0x29,
    ITEM_TO_CUBE = 0x2A,
    NPC_INIT = 0x2F,
    NPC_CANCEL = 0x30,
    QUEST_MESSAGE = 0x31,
    NPC_BUY = 0x32,
    NPC_SELL = 0x33,
    NPC_IDENTIFY_ITEMS = 0x34,
    REPAIR = 0x35,
    HIRE_MERC = 0x36,
    IDENTIFY_GAMBLE = 0x37,
    ENTITY_ACTION = 0x38,
    ADD_STAT = 0x3A,
    ADD_SKILL = 0x3B,
    SELECT_SKILL = 0x3C,
    CLOSE_DOOR = 0x3D,
    ACTIVATE_ITEM = 0x3E,
    CHARACTER_PHRASE = 0x3F,
    UDPATE_QUESTS = 0x40,
    RESURRECT = 0x41,
    STAFF_IN_ORIFICE = 0x44,
    MERC_INTERACT = 0x46,
    MERC_MOVE = 0x47,
    BUSY_STATE_OFF = 0x48,
    WAYPOINT = 0x49,
    REQUEST_ENTITY_UPDATE = 0x4B,
    TRANSMOGRIFY = 0x4C,
    PLAY_NPC_MESSAGE = 0x4D,
    CLICK_BUTTON = 0x4F,
    DROP_GOLD = 0x50,
    BIND_HOTKEY = 0x51,
    STAMINA_ON = 0x53,
    STAMINA_OFF = 0x54,
    QUEST_COMPLETED = 0x58,
    MAKE_ENTITY_MOVE = 0x59,
    SQUELCH_HOSTILE = 0x5D,
    PARTY = 0x5E,
    UPDATE_PLAYER_POS = 0x5F,
    SWAP_WEAPON = 0x60,
    MERC_ITEM = 0x61,
    MERC_RESSURECT = 0x62,
    ITEM_TO_BELT = 0x63,
    WARDEN = 0x66,
    GAME_LOGON_SP = 0x67,
    GAME_LOGON_MULTI = 0x68,
    LEAVE_GAME = 0x69,
    REQUEST_HOSTED_GAMES = 0x6A,
    JOIN_GAME = 0x6B,
    UPLOAD_SAVE = 0x6C,
    PING = 0x6D,
    FINDME_6E = 0x6E,
    FINDME_70 = 0x70,
}

#[allow(clippy::upper_case_acronyms)]
#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum D2GSCmd {
    GAME_LOADING = 0x00,
    GAME_FLAGS = 0x01,
    LOAD_SUCCESSFUL = 0x02,
    LOAD_ACT = 0x03,
    LOAD_COMPLETE = 0x04,
    UNLOAD_COMPLETE = 0x05,
    GAME_EXIT = 0x06,
    MAP_REVEAL = 0x07,
    MAP_HIDE = 0x08,
    ASSIGN_LVL_WARP = 0x09,
    REMOVE_OBJECT = 0x0A,
    GAME_HANDSHAKE = 0x0B,
    NPC_HIT = 0x0C,
    PLAYER_STOP = 0x0D,
    OBJECT_STATE = 0x0E,
    PLAYER_MOVE = 0x0F,
    CHAR_TO_OBJ = 0x10,
    REPORT_KILL = 0x11,
    REASSIGN_PLAYER = 0x15,
    MANY_UNITS_COORDS_UPDATE = 0x16,
    UNKNOWN_17 = 0x17,
    HP_MP_UPDATE2 = 0x18,
    SMALL_GOLDP_ICKUP = 0x19,
    ADD_EXP_BYTE = 0x1A,
    ADD_EXP_WORD = 0x1B,
    ADD_EXP_DWORD = 0x1C,
    SET_ATTR_BYTE = 0x1D,
    SET_ATTR_WORD = 0x1E,
    SET_ATTR_DWORD = 0x1F,
    ATTRIBUTE_UPDATE = 0x20,
    UPDAT_EITEM_OSKILL = 0x21,
    UPDAT_EITEM_SKILL = 0x22,
    SET_SKILL = 0x23,
    CHAT = 0x26,
    NPC_INFO = 0x27,
    PLAYER_QUEST_INFO = 0x28,
    GAME_QUEST_LOG = 0x29,
    NPC_TRANSACTION = 0x2A,
    PLAY_SOUND = 0x2C,
    UPDATE_ITEM_STATS = 0x3E,
    USE_STACKABLE_ITEM = 0x3F,
    ITEM_FLAG_SETTER = 0x40,
    CLEAR_CURSOR = 0x42,
    RELATOR1 = 0x47,
    RELATOR2 = 0x48,
    UNIT_CAST_SKILL_TARGET = 0x4C,
    UNIT_CAST_SKILL_XY = 0x4D,
    MERC_FOR_HIRE = 0x4E,
    CLEAR_MERC_LIST = 0x4F,
    QUEST_SPECIAL = 0x50,
    WORLD_OBJECT = 0x51,
    PLAYER_QUESTLOG = 0x52,
    DARKNESS = 0x53,
    NPC_ENCHANTS = 0x57,
    OPEN_UI = 0x58,
    ASSIGN_PLAYER = 0x59,
    EVENT_MESSAGES = 0x5A,
    PLAYER_JOIN = 0x5B,
    PLAYER_LEAVE = 0x5C,
    QUEST_STATE = 0x5D,
    GAME_QUESTS_AVAILABILITY = 0x5E,
    PORTAL_FLAGS = 0x5F,
    TOWN_PORTAL_STATE = 0x60,
    CAN_GO_TO_ACT = 0x61,
    MAKE_UNIT_TARGETABLE = 0x62,
    WAYPOINT_MENU = 0x63,
    PLAYER_KILL_COUNT = 0x65,
    NPC_MOVE = 0x67,
    NPC_MOVETOENTITY = 0x68,
    NPC_STATE = 0x69,
    NPC_UNKNOWN_0x6A = 0x6A,
    NPC_ACTION = 0x6B,
    NPC_ATTACK = 0x6C,
    NPC_STOP = 0x6D,
    MISSILE_DATA = 0x73,
    PLAYER_CORPSE_ASSIGN = 0x74,
    PLAYER_PARTY_INFO = 0x75,
    PLAYER_IN_PROXIMITY = 0x76,
    TRADE_ACTION = 0x77,
    TRADE_ACCEPTED = 0x78,
    GOLD_IN_TRADE = 0x79,
    SUMMON_LOG = 0x7A,
    ASSIGN_HOTKEY = 0x7B,
    USE_SCROLL = 0x7C,
    SET_ITEM_FLAGS = 0x7D,
    CMNCOF = 0x7E,
    ALL_Y_PARTY_INFO = 0x7F,
    ASSIGN_MERC = 0x81,
    PORTAL_OWNERSHIP = 0x82,
    UNIQUE_EVENTS = 0x89,
    NPC_WANTS_INTERACT = 0x8A,
    PLAYER_RELATIONSHIP = 0x8B,
    RELATIONSHIP_UPDATE = 0x8C,
    ASSIGN_PLAYER_TO_PARTY = 0x8D,
    CORPSE_ASSIGN = 0x8E,
    PONG = 0x8F,
    PARTY_AUTO_MAP_INFO = 0x90,
    NPC_GOSSIP = 0x91,
    REMOVE_ITEMS_DISPLAY = 0x92,
    UNKNOWN_UNIT_SKILL_0x93 = 0x93,
    SKILLS_LIST = 0x94,
    HP_MP_UPDATE = 0x95,
    WALK_VERIFY = 0x96,
    WEAPON_SWITCH = 0x97,
    EVILHUT = 0x98,
    UNIT_SKILL_CAST_TARGET = 0x99,
    UNIT_SKILL_CAST_XY = 0x9A,
    MERC_REVIVE_COST = 0x9B,
    ITEM_ACTION = 0x9C,
    ITEM_OWNED = 0x9D,
    MERC_ATTRIBUTE_BYTE = 0x9E,
    MERC_ATTRIBUTE_WORD = 0x9F,
    MERC_ATTRIBUTE_DWORD = 0xA0,
    MERC_ADD_EXP_BYTE = 0xA1,
    MERC_ADD_EXP_WORD = 0xA2,
    SKILL_AURA_STAT = 0xA3,
    BAAL_WAVES = 0xA4,
    STATE_SKILL_MOVE = 0xA5,
    RUNES_TXT = 0xA6,
    DELAY_STATE = 0xA7,
    SET_STATE = 0xA8,
    END_STATE = 0xA9,
    MULTI_STATES = 0xAA,
    NPC_HEAL = 0xAB,
    MONSTER_PACKET = 0xAC,
    WARDEN = 0xAE,
    START_LOGON = 0xAF,
    CONNECTION_TERMINATED = 0xB0,
    GAMES_INFO = 0xB2,
    DOWNLOAD_SAVE = 0xB3,
    CONNECTION_REFUSED = 0xB4,
}

#[repr(i32)]
#[derive(PartialEq, Copy, Clone)]
pub enum D2ItemTypes {
    None1 = 0,            // 0x00
    None2 = 1,            // 0x01
    Shield = 2,           // 0x02
    Armor = 3,            // 0x03
    Gold = 4,             // 0x04
    BowQuiver = 5,        // 0x05
    CrossbowQuiver = 6,   // 0x06
    PlayerBodyPart = 7,   // 0x07
    Herb = 8,             // 0x08
    Potion = 9,           // 0x09
    Ring = 10,            // 0x0A
    Elixir = 11,          // 0x0B
    Amulet = 12,          // 0x0C
    Charm = 13,           // 0x0D
    None3 = 14,           // 0x0E
    Boots = 15,           // 0x0F
    Gloves = 16,          // 0x10
    None4 = 17,           // 0x11
    Book = 18,            // 0x12
    Belt = 19,            // 0x13
    Gem = 20,             // 0x14
    Torch = 21,           // 0x15
    Scroll = 22,          // 0x16
    None5 = 23,           // 0x17
    Scepter = 24,         // 0x18
    Wand = 25,            // 0x19
    Staff = 26,           // 0x1A
    Bow = 27,             // 0x1B
    Axe = 28,             // 0x1C
    Club = 29,            // 0x1D
    Sword = 30,           // 0x1E
    Hammer = 31,          // 0x1F
    Knife = 32,           // 0x20
    Spear = 33,           // 0x21
    Polearm = 34,         // 0x22
    Crossbow = 35,        // 0x23
    Mace = 36,            // 0x24
    Helm = 37,            // 0x25
    MissilePotion = 38,   // 0x26
    Quest = 39,           // 0x27
    BodyPart = 40,        // 0x28
    Key = 41,             // 0x29
    ThrowingKnife = 42,   // 0x2A
    ThrowingAxe = 43,     // 0x2B
    Javelin = 44,         // 0x2C
    Weapon = 45,          // 0x2D
    MeleeWeapon = 46,     // 0x2E
    MissileWeapon = 47,   // 0x2F
    ThrownWeapon = 48,    // 0x30
    ComboWeapon = 49,     // 0x31
    AnyArmor = 50,        // 0x32
    AnyShield = 51,       // 0x33
    Miscellaneous = 52,   // 0x34
    SocketFiller = 53,    // 0x35
    SecondHand = 54,      // 0x36
    StavesAndRods = 55,   // 0x37
    Missile = 56,         // 0x38
    Blunt = 57,           // 0x39
    Jewel = 58,           // 0x3A
    ClassSpecific = 59,   // 0x3B
    AmazonItem = 60,      // 0x3C
    BarbarianItem = 61,   // 0x3D
    NecromancerItem = 62, // 0x3E
    PaladinItem = 63,     // 0x3F
    SorceressItem = 64,   // 0x40
    AssassinItem = 65,    // 0x41
    DruidItem = 66,       // 0x42
    HandToHand = 67,      // 0x43
    Orb = 68,             // 0x44
    VoodooHeads = 69,     // 0x45
    AuricShields = 70,    // 0x46
    PrimalHelm = 71,      // 0x47
    Pelt = 72,            // 0x48
    Cloak = 73,           // 0x49
    Rune = 74,            // 0x4A
    Circlet = 75,         // 0x4B
    HealingPotion = 76,   // 0x4C
    ManaPotion = 77,      // 0x4D
    RejuvPotion = 78,     // 0x4E
    StaminaPotion = 79,   // 0x4F
    AntidotePotion = 80,  // 0x50
    ThawingPotion = 81,   // 0x51
    SmallCharm = 82,      // 0x52
    MediumCharm = 83,     // 0x53
    LargeCharm = 84,      // 0x54
    AmazonBow = 85,       // 0x55
    AmazonSpear = 86,     // 0x56
    AmazonJavelin = 87,   // 0x57
    HandToHand2 = 88,     // 0x58
    MagicBowQuiv = 89,    // 0x59
    Unk = 90,             // 0x5A
    ChippedGem = 91,      // 0x5B
    FlawedGem = 92,       // 0x5C
    StandardGem = 93,     // 0x5D
    FlawlessGem = 94,     // 0x5E
    PerfectGem = 95,      // 0x5F
    Amethyst = 96,        // 0x60
    Diamond = 97,         // 0x61
    Emerald = 98,         // 0x62
    Ruby = 99,            // 0x63
    Sapphire = 100,       // 0x64
    Topaz = 101,          // 0x65
    Skull = 102,          // 0x66
    Tome = 103,           // 0x67
    ClueScroll = 104,     // 0x68
}

#[repr(i32)]
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub enum D2ItemQualities {
    Inferior = 1,
    Normal = 2,
    Superior = 3,
    Magic = 4,
    Set = 5,
    Rare = 6,
    Unique = 7,
    Craft = 8,
    Tempered = 9,
}

#[repr(u8)]
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub enum D2ItemInvPage {
    Inventory = 0,
    Equip = 1,
    Trade = 2,
    Cube = 3,
    Stash = 4,
    Belt = 5,
    Null = 255,
}

#[repr(i32)]
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub enum D2UnitAlignment {
    Evil = 0,
    Neutral = 1,
    Good = 2,
    Max = 3,
    UnAssigned = 4,
}

pub struct D2ItemCodes {}
impl D2ItemCodes {
    pub const Cube: u32 = u32::from_le_bytes(*b"box ");
}

bitflags! {
    #[derive(PartialEq, Copy, Clone)]
    pub struct D2MonStatsTxtFlags : u32 {
        const IsSpawn           = 0x00000001;
        const IsMelee           = 0x00000002;
        const NoRatio           = 0x00000004;
        const OpenDoors         = 0x00000008;
        const SetBoss           = 0x00000010;
        const BossXfer          = 0x00000020;
        const Boss              = 0x00000040;
        const PrimeEvil         = 0x00000080;
        const Npc               = 0x00000100;
        const Interact          = 0x00000200;
        const InTown            = 0x00000400;
        const LUndead           = 0x00000800;
        const HUndead           = 0x00001000;
        const Demon             = 0x00002000;
        const Flying            = 0x00004000;
        const Killable          = 0x00008000;
        const SwitchAi          = 0x00010000;
        const NoMultiShot       = 0x00020000;
        const NeverCount        = 0x00040000;
        const PetIgnore         = 0x00080000;
        const DeathDmg          = 0x00100000;
        const GenericSpawn      = 0x00200000;
        const Zoo               = 0x00400000;
        const PlaceSpawn        = 0x00800000;
        const Inventory         = 0x01000000;
        const Enabled           = 0x02000000;
        const NoShldBlock       = 0x04000000;
        const NoAura            = 0x08000000;
        const RangedType        = 0x10000000;
    }

    #[derive(PartialEq, Copy, Clone)]
    pub struct D2UnitFlags : u32 {
        const DoUpdate          = 0x00000001;           // tells to update the unit
        const Targetable        = 0x00000002;           // whenever the unit can be selected or not
        const CanBeAttacked     = 0x00000004;           // whenever the unit can be attacked
        const IsValidTarget     = 0x00000008;           // used to check if unit is a valid target
        const InitSeedSet       = 0x00000010;           // tells whenever the unit seed has been initialized
        const DrawShadow        = 0x00000020;           // tells whenver to draw a shadow or not (client only)
        const SkSrvDoFunc       = 0x00000040;           // set when skill srvdofunc is executed
        const ObjPreOperate     = 0x00000080;           // unknown, used by objects with pre-operate disabled
        const HasTxtMsg         = 0x00000100;           // whenever this unit has a text message attached to it
        const IsMerc            = 0x00000200;           // is mercenary unit
        const HasEventSound     = 0x00000400;           // does this unit have an event-sound attached to it (server)
        const Summoner          = 0x00000800;           // set for the summoner only
        const SendRefreshMsg    = 0x00001000;           // used by items to send a refresh message when it drops on ground
        const IsLinkRefreshMsg  = 0x00002000;           // tells whenever this unit is linked to an update message chain
        const SqGfxChange       = 0x00004000;           // tells whenever to load new anim for skill SQ
        const UpgLifeNHitClass  = 0x00008000;           // updates life% and hitclass on client
        const IsDead            = 0x00010000;           // unit is dead
        const NoTc              = 0x00020000;           // disables treasureclass drops
        const MonModeIsChanging = 0x00080000;           // set when monmode changes
        const PreDraw           = 0x00100000;           // pre-draw this unit (like floor tiles, client only)
        const IsAsync           = 0x00200000;           // is async unit (critters)
        const IsClientUnit      = 0x00400000;           // is client unit
        const IsInit            = 0x01000000;           // set when unit has been initialized
        const IsResurrect       = 0x02000000;           // set for resurrected units and items on floor
        const NoXp              = 0x04000000;           // no xp gain from killing this unit
        const Automap           = 0x10000000;           // automap stuff
        const Automap2          = 0x20000000;           // automap stuff
        const PetIgnore         = 0x40000000;           // ignored by pets
        const IsRevive          = 0x80000000;           // is revived monster
    }

    #[derive(PartialEq, Copy, Clone)]
    pub struct D2UnitFlagsEx : u32 {
        const HasInv            = 0x00000001;           // unit has inventory attached to it
        const UpdateInv         = 0x00000002;           // tells to update inventory content
        const IsVendorItem      = 0x00000004;           // set for vendor shop items
        const IsShapeshifted    = 0x00000008;           // unit is shapeshifted
        const ItemInit          = 0x00000010;           // set for items, related to init
        const IsInLos           = 0x00000080;           // unit is in client's line of sight
        const HasBeenDeleted    = 0x00000100;           // unit has been deleted but not free'd yet
        const StoreOwnerInfo    = 0x00000400;           // unit stores info about owner
        const IsCorpse          = 0x00001000;           // unit is a corpse (use UNITFLAG_ISDEAD instead)
        const UnkPathRelated    = 0x00002000;           // related to path
        const Teleported        = 0x00010000;           // unit has been teleported, needs resync
        const StoreLastAttacker = 0x00020000;           // unit stores info about last attacker
        const NoDraw            = 0x00040000;           // don't draw this unit
        const IsExpansion       = 0x02000000;           // is expansion unit
        const ServerUnit        = 0x04000000;           // is server-side unit
    }

    #[derive(PartialEq, Copy, Clone)]
    pub struct D2MonTypeFlags : u8 {
        const Other             = 0x00000001;
        const SuperUnique       = 0x00000002;
        const Champion          = 0x00000004;
        const Unique            = 0x00000008;
        const Minion            = 0x00000010;
        const Possessed         = 0x00000020;
        const Ghostly           = 0x00000040;
        const MultiShot         = 0x00000080;
    }

    #[derive(PartialEq, Copy, Clone)]
    pub struct D2ItemFlags : u32
    {
        const NewItem           = 0x00000001;
        const Target            = 0x00000002;
        const Targeting         = 0x00000004;
        const Deleted           = 0x00000008;
        const Identified        = 0x00000010;
        const Quantity          = 0x00000020;
        const SwitchIn          = 0x00000040;
        const SwitchOut         = 0x00000080;
        const Broken            = 0x00000100;
        const Repaired          = 0x00000200;
        const Unk1              = 0x00000400;
        const Socketed          = 0x00000800;
        const NoSell            = 0x00001000;
        const InStore           = 0x00002000;
        const NoEquip           = 0x00004000;
        const Named             = 0x00008000;
        const IsEar             = 0x00010000;
        const StartItem         = 0x00020000;
        const Unk2              = 0x00040000;
        const Init              = 0x00080000;
        const Unk3              = 0x00100000;
        const CompactSave       = 0x00200000;
        const Ethereal          = 0x00400000;
        const JustSaved         = 0x00800000;
        const Personalized      = 0x01000000;
        const LowQuality        = 0x02000000;
        const Runeword          = 0x04000000;
        const Item              = 0x08000000;
    }

    #[derive(PartialEq, Copy, Clone)]
    pub struct D2ObjectSubClasses : u8 {
        const Shrine        = 0x01;
        const Obelisk       = 0x02;
        const TownPortal    = 0x04;
        const Chest         = 0x08;
        const Portal        = 0x10;
        const Well          = 0x20;
        const WayPoint      = 0x40;
        const Door          = 0x80;
    }
}
