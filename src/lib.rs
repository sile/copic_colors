#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub number: &'static str,
    pub name: &'static str,
    pub hue: Hue,
    pub saturation: Saturation,
    pub brightness: Brightness,
    pub rgb: Rgb,
}

impl Color {
    const fn new(
        number: &'static str,
        name: &'static str,
        hue: Hue,
        saturation: Saturation,
        brightness: Brightness,
        rgb: Rgb,
    ) -> Self {
        Self {
            number,
            name,
            hue,
            saturation,
            brightness,
            rgb,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// Hue (aka. Dominant Color Family)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hue {
    BlueViolet,
    Violet,
    RedViolet,
    Red,
    YellowRed,
    Yellow,
    YellowGreen,
    Green,
    BlueGreen,
    Blue,
    Earth,
    CoolGray,
    NeutralGray,
    TonerGray,
    WarmGray,
    Achromatic,
    Flourescent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Saturation {
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    Undefined,
}

impl Saturation {
    fn to_u8(self) -> Option<u8> {
        match self {
            Saturation::S0 => Some(0),
            Saturation::S1 => Some(1),
            Saturation::S2 => Some(2),
            Saturation::S3 => Some(3),
            Saturation::S4 => Some(4),
            Saturation::S5 => Some(5),
            Saturation::S6 => Some(6),
            Saturation::S7 => Some(7),
            Saturation::S8 => Some(8),
            Saturation::S9 => Some(9),
            Saturation::Undefined => None,
        }
    }
}

impl PartialOrd for Saturation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let n = self.to_u8()?;
        let m = other.to_u8()?;
        Some(n.cmp(&m))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Brightness {
    B000,
    B00,
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
    Undefined,
}

impl Brightness {
    fn to_u8(self) -> Option<u8> {
        match self {
            Brightness::B000 => Some(0),
            Brightness::B00 => Some(1),
            Brightness::B0 => Some(2),
            Brightness::B1 => Some(3),
            Brightness::B2 => Some(4),
            Brightness::B3 => Some(5),
            Brightness::B4 => Some(6),
            Brightness::B5 => Some(7),
            Brightness::B6 => Some(8),
            Brightness::B7 => Some(9),
            Brightness::B8 => Some(10),
            Brightness::B9 => Some(11),
            Brightness::Undefined => None,
        }
    }
}

impl PartialOrd for Brightness {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let n = self.to_u8()?;
        let m = other.to_u8()?;
        Some(n.cmp(&m))
    }
}

// TODO: all_colors()

pub const COLOR_BV0000: Color = Color::new(
    "BV0000",
    "Pale Thistle",
    Hue::BlueViolet,
    Saturation::S0,
    Brightness::B000,
    Rgb::new(238, 236, 245),
);

pub const COLOR_BV000: Color = Color::new(
    "BV000",
    "Iridescent Mauve",
    Hue::BlueViolet,
    Saturation::S0,
    Brightness::B00,
    Rgb::new(238, 231, 241),
);

pub const COLOR_BV00: Color = Color::new(
    "BV00",
    "Mauve Shadow",
    Hue::BlueViolet,
    Saturation::S0,
    Brightness::B0,
    Rgb::new(233, 227, 240),
);

pub const COLOR_BV01: Color = Color::new(
    "BV01",
    "Viola",
    Hue::BlueViolet,
    Saturation::S0,
    Brightness::B1,
    Rgb::new(200, 196, 223),
);

pub const COLOR_BV02: Color = Color::new(
    "BV02",
    "Prune",
    Hue::BlueViolet,
    Saturation::S0,
    Brightness::B2,
    Rgb::new(190, 196, 223),
);

pub const COLOR_BV04: Color = Color::new(
    "BV04",
    "Blue Berry",
    Hue::BlueViolet,
    Saturation::S0,
    Brightness::B4,
    Rgb::new(146, 164, 206),
);

pub const COLOR_BV08: Color = Color::new(
    "BV08",
    "Blue Violet",
    Hue::BlueViolet,
    Saturation::S0,
    Brightness::B8,
    Rgb::new(176, 140, 185),
);

pub const COLOR_BV11: Color = Color::new(
    "BV11",
    "Soft Violet",
    Hue::BlueViolet,
    Saturation::S1,
    Brightness::B1,
    Rgb::new(224, 220, 236),
);

pub const COLOR_BV13: Color = Color::new(
    "BV13",
    "Hydrangea Blue",
    Hue::BlueViolet,
    Saturation::S1,
    Brightness::B3,
    Rgb::new(152, 158, 201),
);

pub const COLOR_BV17: Color = Color::new(
    "BV17",
    "Deep Reddish Blue",
    Hue::BlueViolet,
    Saturation::S1,
    Brightness::B7,
    Rgb::new(126, 144, 189),
);

pub const COLOR_BV20: Color = Color::new(
    "BV20",
    "Dull Lavender",
    Hue::BlueViolet,
    Saturation::S2,
    Brightness::B0,
    Rgb::new(220, 227, 242),
);

pub const COLOR_BV23: Color = Color::new(
    "BV23",
    "Grayish Lavender",
    Hue::BlueViolet,
    Saturation::S2,
    Brightness::B3,
    Rgb::new(196, 205, 225),
);

pub const COLOR_BV25: Color = Color::new(
    "BV25",
    "Grayish Violet",
    Hue::BlueViolet,
    Saturation::S2,
    Brightness::B5,
    Rgb::new(144, 143, 172),
);

pub const COLOR_BV29: Color = Color::new(
    "BV29",
    "Slate",
    Hue::BlueViolet,
    Saturation::S2,
    Brightness::B9,
    Rgb::new(46, 66, 86),
);

pub const COLOR_BV31: Color = Color::new(
    "BV31",
    "Pale Lavender",
    Hue::BlueViolet,
    Saturation::S3,
    Brightness::B1,
    Rgb::new(230, 232, 244),
);

pub const COLOR_BV34: Color = Color::new(
    "BV34",
    "Bluebell",
    Hue::BlueViolet,
    Saturation::S3,
    Brightness::B4,
    Rgb::new(127, 146, 189),
);

pub const COLOR_V0000: Color = Color::new(
    "V0000",
    "Rose Quartz",
    Hue::Violet,
    Saturation::S0,
    Brightness::B000,
    Rgb::new(243, 241, 248),
);

pub const COLOR_V000: Color = Color::new(
    "V000",
    "Pale Heath",
    Hue::Violet,
    Saturation::S0,
    Brightness::B00,
    Rgb::new(247, 242, 247),
);

pub const COLOR_V01: Color = Color::new(
    "V01",
    "Heath",
    Hue::Violet,
    Saturation::S0,
    Brightness::B1,
    Rgb::new(237, 204, 222),
);

pub const COLOR_V04: Color = Color::new(
    "V04",
    "Lilac",
    Hue::Violet,
    Saturation::S0,
    Brightness::B4,
    Rgb::new(237, 185, 209),
);

pub const COLOR_V05: Color = Color::new(
    "V05",
    "Azalea",
    Hue::Violet,
    Saturation::S0,
    Brightness::B5,
    Rgb::new(236, 180, 206),
);

pub const COLOR_V06: Color = Color::new(
    "V06",
    "Lavender",
    Hue::Violet,
    Saturation::S0,
    Brightness::B6,
    Rgb::new(219, 165, 198),
);

pub const COLOR_V09: Color = Color::new(
    "V09",
    "Violet",
    Hue::Violet,
    Saturation::S0,
    Brightness::B9,
    Rgb::new(151, 89, 154),
);

pub const COLOR_V12: Color = Color::new(
    "V12",
    "Pale Lilac",
    Hue::Violet,
    Saturation::S1,
    Brightness::B2,
    Rgb::new(242, 223, 235),
);

pub const COLOR_V15: Color = Color::new(
    "V15",
    "Mallow",
    Hue::Violet,
    Saturation::S1,
    Brightness::B5,
    Rgb::new(224, 182, 209),
);

pub const COLOR_V17: Color = Color::new(
    "V17",
    "Amethyst",
    Hue::Violet,
    Saturation::S1,
    Brightness::B7,
    Rgb::new(179, 161, 199),
);

pub const COLOR_V20: Color = Color::new(
    "V20",
    "Wisteria",
    Hue::Violet,
    Saturation::S2,
    Brightness::B0,
    Rgb::new(213, 204, 214),
);

pub const COLOR_V22: Color = Color::new(
    "V22",
    "Ash Lavender",
    Hue::Violet,
    Saturation::S2,
    Brightness::B2,
    Rgb::new(123, 111, 144),
);

pub const COLOR_V25: Color = Color::new(
    "V25",
    "Pale Blackberry",
    Hue::Violet,
    Saturation::S2,
    Brightness::B5,
    Rgb::new(182, 174, 198),
);

pub const COLOR_V28: Color = Color::new(
    "V28",
    "Eggplant",
    Hue::Violet,
    Saturation::S2,
    Brightness::B8,
    Rgb::new(120, 122, 160),
);

pub const COLOR_V91: Color = Color::new(
    "V91",
    "Pale Grape",
    Hue::Violet,
    Saturation::S9,
    Brightness::B1,
    Rgb::new(239, 208, 216),
);

pub const COLOR_V93: Color = Color::new(
    "V93",
    "Early Grape",
    Hue::Violet,
    Saturation::S9,
    Brightness::B3,
    Rgb::new(237, 205, 223),
);

pub const COLOR_V95: Color = Color::new(
    "V95",
    "Light Grape",
    Hue::Violet,
    Saturation::S9,
    Brightness::B5,
    Rgb::new(197, 137, 170),
);

pub const COLOR_V99: Color = Color::new(
    "V99",
    "Aubergine",
    Hue::Violet,
    Saturation::S9,
    Brightness::B9,
    Rgb::new(79, 51, 77),
);

pub const ALL_COLORS: [Color; 368] = [
    COLOR_BV0000,
    COLOR_BV000,
    COLOR_BV00,
    COLOR_BV01,
    COLOR_BV02,
    COLOR_BV04,
    COLOR_BV08,
    COLOR_BV11,
    COLOR_BV13,
    COLOR_BV17,
    COLOR_BV20,
    COLOR_BV23,
    COLOR_BV25,
    COLOR_BV29,
    COLOR_BV31,
    COLOR_BV34,
    COLOR_V0000,
    COLOR_V000,
    COLOR_V01,
    COLOR_V04,
    COLOR_V05,
    COLOR_V06,
    COLOR_V09,
    COLOR_V12,
    COLOR_V15,
    COLOR_V17,
    COLOR_V20,
    COLOR_V22,
    COLOR_V25,
    COLOR_V28,
    COLOR_V91,
    COLOR_V93,
    COLOR_V95,
    COLOR_V99,
    // RV0000, RV000, RV00, RV02, RV04, RV06, RV09, RV10, RV11, RV13, RV14, RV17, RV19, RV21,
    // RV23, RV25, RV29, RV32, RV34, RV42, RV52, RV55, RV63, RV66, RV69, RV91, RV93, RV95, RV99,
    // R0000, R000, R00, R01, R02, R05, R08, R11, R12, R14, R17, R20, R21, R22, R24, R27, R29, R30,
    // R32, R35, R37, R39, R43, R46, R56, R59, R81, R83, R85, R89, YR0000, YR000, YR00, YR01, YR02,
    // YR04, YR07, YR09, YR12, YR14, YR15, YR16, YR18, YR20, YR21, YR23, YR24, YR27, YR30, YR31, YR61,
    // YR65, YR68, YR82, Y0000, Y000, Y00, Y02, Y04, Y06, Y08, Y11, Y13, Y15, Y17, Y18, Y19, Y21, Y23,
    // Y26, Y28, Y32, Y35, Y38, YG0000, YG00, YG01, YG03, YG05, YG06, YG07, YG09, YG11, YG13, YG17,
    // YG21, YG23, YG25, YG41, YG45, YG61, YG63, YG91, YG93, YG95, YG97, YG99, G0000, G000, G00, G02,
    // G03, G05, G07, G09, G12, G14, G16, G17, G19, G20, G21, G24, G28, G40, G43, G46, G82, G85, G94,
    // G99, BG0000, BG000, BG01, BG02, BG05, BG07, BG09, BG10, BG11, BG13, BG15, BG18, BG23, BG32,
    // BG34, BG45, BG49, BG53, BG57, BG70, BG72, BG75, BG78, BG90, BG93, BG96, BG99, B0000, B000, B00,
    // B01, B02, B04, B05, B06, B12, B14, B16, B18, B21, B23, B24, B26, B28, B29, B32, B34, B37, B39,
    // B41, B45, B52, B60, B63, B66, B69, B79, B91, B93, B95, B97, B99, E0000, E000, E00, E01, E02,
    // E04, E07, E08, E09, E11, E13, E15, E17, E18, E19, E21, E23, E25, E27, E29, E30, E31, E33, E34,
    // E35, E37, E39, E40, E41, E42, E43, E44, E47, E49, E50, E51, E53, E55, E57, E59, E70, E71, E74,
    // E77, E79, E81, E84, E87, E89, E93, E95, E97, E99, C00, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9,
    // C10, N0, N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
    // W00, W0, W1, W2, W3, W4, W5, W6, W7, W8, W9, W10, // 0, 100, 110
    // FV, FRV, FYR, FY, FYG, FG, FBG, FB,
];
