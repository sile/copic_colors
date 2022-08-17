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

pub const COLOR_RV0000: Color = Color::new(
    "RV0000",
    "Evening Primrose",
    Hue::RedViolet,
    Saturation::S0,
    Brightness::B000,
    Rgb::new(246, 239, 246),
);

pub const COLOR_RV000: Color = Color::new(
    "RV000",
    "Pale Purple",
    Hue::RedViolet,
    Saturation::S0,
    Brightness::B00,
    Rgb::new(247, 233, 241),
);

pub const COLOR_RV00: Color = Color::new(
    "RV00",
    "Water Lily",
    Hue::RedViolet,
    Saturation::S0,
    Brightness::B0,
    Rgb::new(243, 211, 233),
);

pub const COLOR_RV02: Color = Color::new(
    "RV02",
    "Sugared Almond Pink",
    Hue::RedViolet,
    Saturation::S0,
    Brightness::B2,
    Rgb::new(252, 222, 233),
);

pub const COLOR_RV04: Color = Color::new(
    "RV04",
    "Shock Pink",
    Hue::RedViolet,
    Saturation::S0,
    Brightness::B4,
    Rgb::new(249, 176, 197),
);

pub const COLOR_RV06: Color = Color::new(
    "RV06",
    "Cerise",
    Hue::RedViolet,
    Saturation::S0,
    Brightness::B6,
    Rgb::new(238, 134, 174),
);

pub const COLOR_RV09: Color = Color::new(
    "RV09",
    "Fuchsia",
    Hue::RedViolet,
    Saturation::S0,
    Brightness::B9,
    Rgb::new(232, 124, 172),
);

pub const COLOR_RV10: Color = Color::new(
    "RV10",
    "Pale Pink",
    Hue::RedViolet,
    Saturation::S1,
    Brightness::B0,
    Rgb::new(254, 241, 245),
);

pub const COLOR_RV11: Color = Color::new(
    "RV11",
    "Pink",
    Hue::RedViolet,
    Saturation::S1,
    Brightness::B1,
    Rgb::new(252, 223, 227),
);

pub const COLOR_RV13: Color = Color::new(
    "RV13",
    "Tender Pink",
    Hue::RedViolet,
    Saturation::S1,
    Brightness::B3,
    Rgb::new(252, 211, 222),
);

pub const COLOR_RV14: Color = Color::new(
    "RV14",
    "Begonia Pink",
    Hue::RedViolet,
    Saturation::S1,
    Brightness::B4,
    Rgb::new(248, 160, 188),
);

pub const COLOR_RV17: Color = Color::new(
    "RV17",
    "Deep Magenta",
    Hue::RedViolet,
    Saturation::S1,
    Brightness::B7,
    Rgb::new(229, 139, 181),
);

pub const COLOR_RV19: Color = Color::new(
    "RV19",
    "Red Violet",
    Hue::RedViolet,
    Saturation::S1,
    Brightness::B9,
    Rgb::new(222, 115, 167),
);

pub const COLOR_RV21: Color = Color::new(
    "RV21",
    "Light Pink",
    Hue::RedViolet,
    Saturation::S2,
    Brightness::B1,
    Rgb::new(254, 238, 237),
);

pub const COLOR_RV23: Color = Color::new(
    "RV23",
    "Pure Pink",
    Hue::RedViolet,
    Saturation::S2,
    Brightness::B3,
    Rgb::new(251, 198, 207),
);

pub const COLOR_RV25: Color = Color::new(
    "RV25",
    "Dog Rose Flower",
    Hue::RedViolet,
    Saturation::S2,
    Brightness::B5,
    Rgb::new(248, 161, 193),
);

pub const COLOR_RV29: Color = Color::new(
    "RV29",
    "Crimson",
    Hue::RedViolet,
    Saturation::S2,
    Brightness::B9,
    Rgb::new(241, 74, 129),
);

pub const COLOR_RV32: Color = Color::new(
    "RV32",
    "Shadow Pink",
    Hue::RedViolet,
    Saturation::S3,
    Brightness::B2,
    Rgb::new(253, 220, 215),
);

pub const COLOR_RV34: Color = Color::new(
    "RV34",
    "Dark Pink",
    Hue::RedViolet,
    Saturation::S3,
    Brightness::B4,
    Rgb::new(251, 188, 184),
);

pub const COLOR_RV42: Color = Color::new(
    "RV42",
    "Salmon Pink",
    Hue::RedViolet,
    Saturation::S4,
    Brightness::B2,
    Rgb::new(251, 199, 191),
);

pub const COLOR_RV52: Color = Color::new(
    "RV52",
    "Cotton Candy",
    Hue::RedViolet,
    Saturation::S5,
    Brightness::B2,
    Rgb::new(220, 156, 182),
);

pub const COLOR_RV55: Color = Color::new(
    "RV55",
    "Hollyhock",
    Hue::RedViolet,
    Saturation::S5,
    Brightness::B5,
    Rgb::new(217, 97, 150),
);

pub const COLOR_RV63: Color = Color::new(
    "RV63",
    "Begonia",
    Hue::RedViolet,
    Saturation::S6,
    Brightness::B3,
    Rgb::new(219, 138, 181),
);

pub const COLOR_RV66: Color = Color::new(
    "RV66",
    "Raspberry",
    Hue::RedViolet,
    Saturation::S6,
    Brightness::B6,
    Rgb::new(141, 55, 101),
);

pub const COLOR_RV69: Color = Color::new(
    "RV69",
    "Peony",
    Hue::RedViolet,
    Saturation::S6,
    Brightness::B9,
    Rgb::new(156, 98, 118),
);

pub const COLOR_RV91: Color = Color::new(
    "RV91",
    "Grayish Cherry",
    Hue::RedViolet,
    Saturation::S9,
    Brightness::B1,
    Rgb::new(237, 222, 232),
);

pub const COLOR_RV93: Color = Color::new(
    "RV93",
    "Smoky Purple",
    Hue::RedViolet,
    Saturation::S9,
    Brightness::B3,
    Rgb::new(238, 195, 210),
);

pub const COLOR_RV95: Color = Color::new(
    "RV95",
    "Baby Blossoms",
    Hue::RedViolet,
    Saturation::S9,
    Brightness::B5,
    Rgb::new(198, 148, 170),
);

pub const COLOR_RV99: Color = Color::new(
    "RV99",
    "Argyle Purple",
    Hue::RedViolet,
    Saturation::S9,
    Brightness::B9,
    Rgb::new(110, 87, 100),
);

pub const COLOR_R0000: Color = Color::new(
    "R0000",
    "Pink Beryl",
    Hue::Red,
    Saturation::S0,
    Brightness::B000,
    Rgb::new(252, 246, 242),
);

pub const COLOR_R000: Color = Color::new(
    "R000",
    "Cherry White",
    Hue::Red,
    Saturation::S0,
    Brightness::B00,
    Rgb::new(255, 246, 242),
);

pub const COLOR_R00: Color = Color::new(
    "R00",
    "Pinkish White",
    Hue::Red,
    Saturation::S0,
    Brightness::B0,
    Rgb::new(255, 245, 239),
);

pub const COLOR_R01: Color = Color::new(
    "R01",
    "Pinkish Vanilla",
    Hue::Red,
    Saturation::S0,
    Brightness::B1,
    Rgb::new(253, 224, 218),
);

pub const COLOR_R02: Color = Color::new(
    "R02",
    "Rose Salmon",
    Hue::Red,
    Saturation::S0,
    Brightness::B2,
    Rgb::new(254, 220, 208),
);

pub const COLOR_R05: Color = Color::new(
    "R05",
    "Salmon Red",
    Hue::Red,
    Saturation::S0,
    Brightness::B5,
    Rgb::new(248, 157, 133),
);

pub const COLOR_R08: Color = Color::new(
    "R08",
    "Vermilion",
    Hue::Red,
    Saturation::S0,
    Brightness::B8,
    Rgb::new(244, 109, 86),
);

pub const COLOR_R11: Color = Color::new(
    "R11",
    "Pale Cherry Pink",
    Hue::Red,
    Saturation::S1,
    Brightness::B1,
    Rgb::new(255, 232, 221),
);

pub const COLOR_R12: Color = Color::new(
    "R12",
    "Light Tea Rose",
    Hue::Red,
    Saturation::S1,
    Brightness::B2,
    Rgb::new(254, 220, 203),
);

pub const COLOR_R14: Color = Color::new(
    "R14",
    "Light Rouge",
    Hue::Red,
    Saturation::S1,
    Brightness::B4,
    Rgb::new(249, 168, 157),
);

pub const COLOR_R17: Color = Color::new(
    "R17",
    "Lipstick Orange",
    Hue::Red,
    Saturation::S1,
    Brightness::B7,
    Rgb::new(247, 143, 114),
);

pub const COLOR_R20: Color = Color::new(
    "R20",
    "Blush",
    Hue::Red,
    Saturation::S2,
    Brightness::B0,
    Rgb::new(254, 224, 217),
);

pub const COLOR_R21: Color = Color::new(
    "R21",
    "Sardonyx",
    Hue::Red,
    Saturation::S2,
    Brightness::B1,
    Rgb::new(252, 182, 172),
);

pub const COLOR_R22: Color = Color::new(
    "R22",
    "Light Prawn",
    Hue::Red,
    Saturation::S2,
    Brightness::B2,
    Rgb::new(255, 153, 153),
);

pub const COLOR_R24: Color = Color::new(
    "R24",
    "Prawn",
    Hue::Red,
    Saturation::S2,
    Brightness::B4,
    Rgb::new(245, 127, 130),
);

pub const COLOR_R27: Color = Color::new(
    "R27",
    "Cadmium Red",
    Hue::Red,
    Saturation::S2,
    Brightness::B7,
    Rgb::new(242, 83, 100),
);

pub const COLOR_R29: Color = Color::new(
    "R29",
    "Lipstick Red",
    Hue::Red,
    Saturation::S2,
    Brightness::B9,
    Rgb::new(239, 0, 71),
);

pub const COLOR_R30: Color = Color::new(
    "R30",
    "Pale Yellowish Pink",
    Hue::Red,
    Saturation::S3,
    Brightness::B0,
    Rgb::new(254, 234, 229),
);

pub const COLOR_R32: Color = Color::new(
    "R32",
    "Peach",
    Hue::Red,
    Saturation::S3,
    Brightness::B2,
    Rgb::new(252, 204, 196),
);

pub const COLOR_R35: Color = Color::new(
    "R35",
    "Coral",
    Hue::Red,
    Saturation::S3,
    Brightness::B5,
    Rgb::new(245, 122, 138),
);

pub const COLOR_R37: Color = Color::new(
    "R37",
    "Carmine",
    Hue::Red,
    Saturation::S3,
    Brightness::B7,
    Rgb::new(237, 120, 125),
);

pub const COLOR_R39: Color = Color::new(
    "R39",
    "Garnet",
    Hue::Red,
    Saturation::S3,
    Brightness::B9,
    Rgb::new(214, 83, 130),
);

pub const COLOR_R43: Color = Color::new(
    "R43",
    "Bougainvillaea",
    Hue::Red,
    Saturation::S4,
    Brightness::B3,
    Rgb::new(241, 143, 150),
);

pub const COLOR_R46: Color = Color::new(
    "R46",
    "Strong Red",
    Hue::Red,
    Saturation::S4,
    Brightness::B6,
    Rgb::new(230, 80, 109),
);

pub const COLOR_R56: Color = Color::new(
    "R56",
    "Currant",
    Hue::Red,
    Saturation::S5,
    Brightness::B6,
    Rgb::new(195, 102, 121),
);

pub const COLOR_R59: Color = Color::new(
    "R59",
    "Cardinal",
    Hue::Red,
    Saturation::S5,
    Brightness::B9,
    Rgb::new(197, 95, 124),
);

pub const COLOR_R81: Color = Color::new(
    "R81",
    "Rose Pink",
    Hue::Red,
    Saturation::S8,
    Brightness::B1,
    Rgb::new(246, 212, 220),
);

pub const COLOR_R83: Color = Color::new(
    "R83",
    "Rose Mist",
    Hue::Red,
    Saturation::S8,
    Brightness::B3,
    Rgb::new(245, 170, 191),
);

pub const COLOR_R85: Color = Color::new(
    "R85",
    "Rose Red",
    Hue::Red,
    Saturation::S8,
    Brightness::B5,
    Rgb::new(222, 118, 154),
);

pub const COLOR_R89: Color = Color::new(
    "R89",
    "Dark Red",
    Hue::Red,
    Saturation::S8,
    Brightness::B9,
    Rgb::new(145, 53, 77),
);

pub const COLOR_YR0000: Color = Color::new(
    "YR0000",
    "Pale Chiffon",
    Hue::YellowRed,
    Saturation::S0,
    Brightness::B000,
    Rgb::new(252, 246, 235),
);

pub const COLOR_YR000: Color = Color::new(
    "YR000",
    "Silk",
    Hue::YellowRed,
    Saturation::S0,
    Brightness::B00,
    Rgb::new(255, 241, 225),
);

pub const COLOR_YR00: Color = Color::new(
    "YR00",
    "Powder Pink",
    Hue::YellowRed,
    Saturation::S0,
    Brightness::B0,
    Rgb::new(254, 223, 199),
);

pub const COLOR_YR01: Color = Color::new(
    "YR01",
    "Peach Puff",
    Hue::YellowRed,
    Saturation::S0,
    Brightness::B1,
    Rgb::new(253, 214, 183),
);

pub const COLOR_YR02: Color = Color::new(
    "YR02",
    "Light Orange",
    Hue::YellowRed,
    Saturation::S0,
    Brightness::B2,
    Rgb::new(255, 228, 207),
);

pub const COLOR_YR04: Color = Color::new(
    "YR04",
    "Chrome Orange",
    Hue::YellowRed,
    Saturation::S0,
    Brightness::B4,
    Rgb::new(255, 207, 109),
);

pub const COLOR_YR07: Color = Color::new(
    "YR07",
    "Cadmium Orange",
    Hue::YellowRed,
    Saturation::S0,
    Brightness::B7,
    Rgb::new(246, 120, 51),
);

pub const COLOR_YR09: Color = Color::new(
    "YR09",
    "Chinese Orange",
    Hue::YellowRed,
    Saturation::S0,
    Brightness::B7,
    Rgb::new(243, 87, 0),
);

pub const COLOR_YR12: Color = Color::new(
    "YR12",
    "Loquat",
    Hue::YellowRed,
    Saturation::S1,
    Brightness::B2,
    Rgb::new(254, 227, 145),
);

pub const COLOR_YR14: Color = Color::new(
    "YR14",
    "Caramel",
    Hue::YellowRed,
    Saturation::S1,
    Brightness::B4,
    Rgb::new(255, 211, 76),
);

pub const COLOR_YR15: Color = Color::new(
    "YR15",
    "Pumpkin Yellow",
    Hue::YellowRed,
    Saturation::S1,
    Brightness::B5,
    Rgb::new(255, 209, 152),
);

pub const COLOR_YR16: Color = Color::new(
    "YR16",
    "Apricot",
    Hue::YellowRed,
    Saturation::S1,
    Brightness::B6,
    Rgb::new(255, 196, 23),
);

pub const COLOR_YR18: Color = Color::new(
    "YR18",
    "Sanguine",
    Hue::YellowRed,
    Saturation::S1,
    Brightness::B8,
    Rgb::new(245, 115, 53),
);

pub const COLOR_YR20: Color = Color::new(
    "YR20",
    "Yellowish Shade",
    Hue::YellowRed,
    Saturation::S2,
    Brightness::B0,
    Rgb::new(255, 233, 201),
);

pub const COLOR_YR21: Color = Color::new(
    "YR21",
    "Cream",
    Hue::YellowRed,
    Saturation::S2,
    Brightness::B1,
    Rgb::new(248, 229, 187),
);

pub const COLOR_YR23: Color = Color::new(
    "YR23",
    "Yellow Ochre",
    Hue::YellowRed,
    Saturation::S2,
    Brightness::B3,
    Rgb::new(242, 219, 152),
);

pub const COLOR_YR24: Color = Color::new(
    "YR24",
    "Pale Sepia",
    Hue::YellowRed,
    Saturation::S2,
    Brightness::B4,
    Rgb::new(246, 220, 107),
);

pub const COLOR_YR27: Color = Color::new(
    "YR27",
    "Tuscan Orange",
    Hue::YellowRed,
    Saturation::S2,
    Brightness::B7,
    Rgb::new(216, 103, 69),
);

pub const COLOR_YR30: Color = Color::new(
    "YR30",
    "Macadamia nut",
    Hue::YellowRed,
    Saturation::S3,
    Brightness::B0,
    Rgb::new(247, 237, 201),
);

pub const COLOR_YR31: Color = Color::new(
    "YR31",
    "Light Reddish Yellow",
    Hue::YellowRed,
    Saturation::S3,
    Brightness::B1,
    Rgb::new(255, 230, 178),
);

pub const COLOR_YR61: Color = Color::new(
    "YR61",
    "Spring Orange",
    Hue::YellowRed,
    Saturation::S6,
    Brightness::B1,
    Rgb::new(254, 226, 204),
);

pub const COLOR_YR65: Color = Color::new(
    "YR65",
    "Atoll",
    Hue::YellowRed,
    Saturation::S6,
    Brightness::B5,
    Rgb::new(254, 187, 101),
);

pub const COLOR_YR68: Color = Color::new(
    "YR68",
    "Orange",
    Hue::YellowRed,
    Saturation::S6,
    Brightness::B8,
    Rgb::new(246, 119, 0),
);

pub const COLOR_YR82: Color = Color::new(
    "YR82",
    "Mellow Peach",
    Hue::YellowRed,
    Saturation::S8,
    Brightness::B2,
    Rgb::new(255, 209, 152),
);

pub const COLOR_Y0000: Color = Color::new(
    "Y0000",
    "Yellow Fluorite",
    Hue::Yellow,
    Saturation::S0,
    Brightness::B000,
    Rgb::new(255, 254, 247),
);

pub const COLOR_Y000: Color = Color::new(
    "Y000",
    "Pale Lemon",
    Hue::Yellow,
    Saturation::S0,
    Brightness::B00,
    Rgb::new(255, 255, 246),
);

pub const COLOR_Y00: Color = Color::new(
    "Y00",
    "Barium Yellow",
    Hue::Yellow,
    Saturation::S0,
    Brightness::B0,
    Rgb::new(255, 254, 229),
);

pub const COLOR_Y02: Color = Color::new(
    "Y02",
    "Canary Yellow",
    Hue::Yellow,
    Saturation::S0,
    Brightness::B2,
    Rgb::new(249, 245, 160),
);

pub const COLOR_Y04: Color = Color::new(
    "Y04",
    "Acacia",
    Hue::Yellow,
    Saturation::S0,
    Brightness::B4,
    Rgb::new(243, 233, 83),
);

pub const COLOR_Y06: Color = Color::new(
    "Y06",
    "Yellow",
    Hue::Yellow,
    Saturation::S0,
    Brightness::B6,
    Rgb::new(255, 247, 114),
);

pub const COLOR_Y08: Color = Color::new(
    "Y08",
    "Acid Yellow",
    Hue::Yellow,
    Saturation::S0,
    Brightness::B8,
    Rgb::new(255, 245, 0),
);

pub const COLOR_Y11: Color = Color::new(
    "Y11",
    "Pale Yellow",
    Hue::Yellow,
    Saturation::S1,
    Brightness::B1,
    Rgb::new(255, 252, 211),
);

pub const COLOR_Y13: Color = Color::new(
    "Y13",
    "Lemon Yellow",
    Hue::Yellow,
    Saturation::S1,
    Brightness::B3,
    Rgb::new(252, 249, 183),
);

pub const COLOR_Y15: Color = Color::new(
    "Y15",
    "Cadmium Yellow",
    Hue::Yellow,
    Saturation::S1,
    Brightness::B5,
    Rgb::new(255, 238, 114),
);

pub const COLOR_Y17: Color = Color::new(
    "Y17",
    "Golden Yellow",
    Hue::Yellow,
    Saturation::S1,
    Brightness::B7,
    Rgb::new(255, 234, 85),
);

pub const COLOR_Y18: Color = Color::new(
    "Y18",
    "Lightning Yellow",
    Hue::Yellow,
    Saturation::S1,
    Brightness::B8,
    Rgb::new(255, 242, 87),
);

pub const COLOR_Y19: Color = Color::new(
    "Y19",
    "Napoli Yellow",
    Hue::Yellow,
    Saturation::S1,
    Brightness::B9,
    Rgb::new(255, 238, 57),
);

pub const COLOR_Y21: Color = Color::new(
    "Y21",
    "Buttercup Yellow",
    Hue::Yellow,
    Saturation::S2,
    Brightness::B1,
    Rgb::new(255, 242, 201),
);

pub const COLOR_Y23: Color = Color::new(
    "Y23",
    "Yellowish Beige",
    Hue::Yellow,
    Saturation::S2,
    Brightness::B3,
    Rgb::new(253, 234, 190),
);

pub const COLOR_Y26: Color = Color::new(
    "Y26",
    "Mustard",
    Hue::Yellow,
    Saturation::S2,
    Brightness::B6,
    Rgb::new(246, 229, 111),
);

pub const COLOR_Y28: Color = Color::new(
    "Y28",
    "Lionet Gold",
    Hue::Yellow,
    Saturation::S2,
    Brightness::B8,
    Rgb::new(213, 181, 110),
);

pub const COLOR_Y32: Color = Color::new(
    "Y32",
    "Cashmere",
    Hue::Yellow,
    Saturation::S3,
    Brightness::B2,
    Rgb::new(251, 230, 202),
);

pub const COLOR_Y35: Color = Color::new(
    "Y35",
    "Maize",
    Hue::Yellow,
    Saturation::S3,
    Brightness::B5,
    Rgb::new(255, 224, 129),
);

pub const COLOR_Y38: Color = Color::new(
    "Y38",
    "Honey",
    Hue::Yellow,
    Saturation::S3,
    Brightness::B8,
    Rgb::new(255, 255, 153),
);

pub const COLOR_YG0000: Color = Color::new(
    "YG0000",
    "Lily White",
    Hue::YellowGreen,
    Saturation::S0,
    Brightness::B000,
    Rgb::new(247, 249, 228),
);

pub const COLOR_YG00: Color = Color::new(
    "YG00",
    "Mimosa Yellow",
    Hue::YellowGreen,
    Saturation::S0,
    Brightness::B0,
    Rgb::new(238, 235, 166),
);

pub const COLOR_YG01: Color = Color::new(
    "YG01",
    "Green Bice",
    Hue::YellowGreen,
    Saturation::S0,
    Brightness::B1,
    Rgb::new(238, 242, 200),
);

pub const COLOR_YG03: Color = Color::new(
    "YG03",
    "Yellow Green",
    Hue::YellowGreen,
    Saturation::S0,
    Brightness::B3,
    Rgb::new(234, 238, 178),
);

pub const COLOR_YG05: Color = Color::new(
    "YG05",
    "Salad",
    Hue::YellowGreen,
    Saturation::S0,
    Brightness::B5,
    Rgb::new(226, 233, 153),
);

pub const COLOR_YG06: Color = Color::new(
    "YG06",
    "Yellowish Green",
    Hue::YellowGreen,
    Saturation::S0,
    Brightness::B6,
    Rgb::new(211, 227, 152),
);

pub const COLOR_YG07: Color = Color::new(
    "YG07",
    "Acid Green",
    Hue::YellowGreen,
    Saturation::S0,
    Brightness::B7,
    Rgb::new(182, 209, 53),
);

pub const COLOR_YG09: Color = Color::new(
    "YG09",
    "Lettuce Green",
    Hue::YellowGreen,
    Saturation::S0,
    Brightness::B9,
    Rgb::new(147, 197, 96),
);

pub const COLOR_YG11: Color = Color::new(
    "YG11",
    "Mignonette",
    Hue::YellowGreen,
    Saturation::S1,
    Brightness::B1,
    Rgb::new(236, 243, 213),
);

pub const COLOR_YG13: Color = Color::new(
    "YG13",
    "Chartreuse",
    Hue::YellowGreen,
    Saturation::S1,
    Brightness::B3,
    Rgb::new(223, 233, 166),
);

pub const COLOR_YG17: Color = Color::new(
    "YG17",
    "Grass Green",
    Hue::YellowGreen,
    Saturation::S1,
    Brightness::B7,
    Rgb::new(122, 191, 74),
);

pub const COLOR_YG21: Color = Color::new(
    "YG21",
    "Anise",
    Hue::YellowGreen,
    Saturation::S2,
    Brightness::B1,
    Rgb::new(249, 248, 198),
);

pub const COLOR_YG23: Color = Color::new(
    "YG23",
    "New Leaf",
    Hue::YellowGreen,
    Saturation::S2,
    Brightness::B3,
    Rgb::new(239, 239, 152),
);

pub const COLOR_YG25: Color = Color::new(
    "YG25",
    "Celadon Green",
    Hue::YellowGreen,
    Saturation::S2,
    Brightness::B5,
    Rgb::new(220, 228, 127),
);

pub const COLOR_YG41: Color = Color::new(
    "YG41",
    "Pale Cobalt Green",
    Hue::YellowGreen,
    Saturation::S4,
    Brightness::B1,
    Rgb::new(225, 238, 217),
);

pub const COLOR_YG45: Color = Color::new(
    "YG45",
    "Cobalt Green",
    Hue::YellowGreen,
    Saturation::S4,
    Brightness::B5,
    Rgb::new(197, 224, 190),
);

pub const COLOR_YG61: Color = Color::new(
    "YG61",
    "Pale Moss",
    Hue::YellowGreen,
    Saturation::S6,
    Brightness::B1,
    Rgb::new(183, 202, 144),
);

pub const COLOR_YG63: Color = Color::new(
    "YG63",
    "Pea Green",
    Hue::YellowGreen,
    Saturation::S6,
    Brightness::B3,
    Rgb::new(181, 210, 171),
);

pub const COLOR_YG67: Color = Color::new(
    "YG67",
    "Moss",
    Hue::YellowGreen,
    Saturation::S6,
    Brightness::B7,
    Rgb::new(151, 197, 146),
);

pub const COLOR_YG91: Color = Color::new(
    "YG91",
    "Putty",
    Hue::YellowGreen,
    Saturation::S9,
    Brightness::B1,
    Rgb::new(228, 223, 184),
);

pub const COLOR_YG93: Color = Color::new(
    "YG93",
    "Grayish Yellow",
    Hue::YellowGreen,
    Saturation::S9,
    Brightness::B3,
    Rgb::new(222, 219, 166),
);

pub const COLOR_YG95: Color = Color::new(
    "YG95",
    "Pale Olive",
    Hue::YellowGreen,
    Saturation::S9,
    Brightness::B5,
    Rgb::new(219, 210, 103),
);

pub const COLOR_YG97: Color = Color::new(
    "YG97",
    "Spanish Olive",
    Hue::YellowGreen,
    Saturation::S9,
    Brightness::B7,
    Rgb::new(170, 160, 0),
);

pub const COLOR_YG99: Color = Color::new(
    "YG99",
    "Marine Green",
    Hue::YellowGreen,
    Saturation::S9,
    Brightness::B9,
    Rgb::new(106, 120, 0),
);

pub const COLOR_G0000: Color = Color::new(
    "G0000",
    "Crystal Opal",
    Hue::Green,
    Saturation::S0,
    Brightness::B000,
    Rgb::new(246, 250, 246),
);

pub const COLOR_G000: Color = Color::new(
    "G000",
    "Pale Green",
    Hue::Green,
    Saturation::S0,
    Brightness::B00,
    Rgb::new(247, 251, 247),
);

pub const COLOR_G00: Color = Color::new(
    "G00",
    "Jade Green",
    Hue::Green,
    Saturation::S0,
    Brightness::B0,
    Rgb::new(234, 245, 246),
);

pub const COLOR_G02: Color = Color::new(
    "G02",
    "Spectrum Green",
    Hue::Green,
    Saturation::S0,
    Brightness::B2,
    Rgb::new(219, 236, 217),
);

pub const COLOR_G03: Color = Color::new(
    "G03",
    "Meadow Green",
    Hue::Green,
    Saturation::S0,
    Brightness::B3,
    Rgb::new(176, 222, 127),
);

pub const COLOR_G05: Color = Color::new(
    "G05",
    "Emerald Green",
    Hue::Green,
    Saturation::S0,
    Brightness::B5,
    Rgb::new(125, 192, 121),
);

pub const COLOR_G07: Color = Color::new(
    "G07",
    "Nile Green",
    Hue::Green,
    Saturation::S0,
    Brightness::B7,
    Rgb::new(143, 198, 118),
);

pub const COLOR_G09: Color = Color::new(
    "G09",
    "Veronese Green",
    Hue::Green,
    Saturation::S0,
    Brightness::B9,
    Rgb::new(143, 196, 96),
);

pub const COLOR_G12: Color = Color::new(
    "G12",
    "Sea Green",
    Hue::Green,
    Saturation::S1,
    Brightness::B2,
    Rgb::new(222, 236, 203),
);

pub const COLOR_G14: Color = Color::new(
    "G14",
    "Apple Green",
    Hue::Green,
    Saturation::S1,
    Brightness::B4,
    Rgb::new(170, 210, 148),
);

pub const COLOR_G16: Color = Color::new(
    "G16",
    "Malachite",
    Hue::Green,
    Saturation::S1,
    Brightness::B6,
    Rgb::new(118, 193, 156),
);

pub const COLOR_G17: Color = Color::new(
    "G17",
    "Forest Green",
    Hue::Green,
    Saturation::S1,
    Brightness::B7,
    Rgb::new(39, 174, 125),
);

pub const COLOR_G19: Color = Color::new(
    "G19",
    "Bright Parrot Green",
    Hue::Green,
    Saturation::S1,
    Brightness::B9,
    Rgb::new(73, 182, 138),
);

pub const COLOR_G20: Color = Color::new(
    "G20",
    "Wax White",
    Hue::Green,
    Saturation::S2,
    Brightness::B0,
    Rgb::new(242, 247, 224),
);

pub const COLOR_G21: Color = Color::new(
    "G21",
    "Lime Green",
    Hue::Green,
    Saturation::S2,
    Brightness::B1,
    Rgb::new(211, 232, 211),
);

pub const COLOR_G24: Color = Color::new(
    "G24",
    "Willow",
    Hue::Green,
    Saturation::S2,
    Brightness::B4,
    Rgb::new(209, 228, 187),
);

pub const COLOR_G28: Color = Color::new(
    "G28",
    "Ocean Green",
    Hue::Green,
    Saturation::S2,
    Brightness::B8,
    Rgb::new(35, 150, 101),
);

pub const COLOR_G29: Color = Color::new(
    "G29",
    "Pine Tree Green",
    Hue::Green,
    Saturation::S2,
    Brightness::B9,
    Rgb::new(62, 134, 103),
);

pub const COLOR_G40: Color = Color::new(
    "G40",
    "Dim Green",
    Hue::Green,
    Saturation::S4,
    Brightness::B0,
    Rgb::new(236, 244, 227),
);

pub const COLOR_G43: Color = Color::new(
    "G43",
    "Pistachio",
    Hue::Green,
    Saturation::S4,
    Brightness::B3,
    Rgb::new(146, 179, 92),
);

pub const COLOR_G46: Color = Color::new(
    "G46",
    "Mistletoe",
    Hue::Green,
    Saturation::S4,
    Brightness::B6,
    Rgb::new(84, 137, 93),
);

pub const COLOR_G82: Color = Color::new(
    "G82",
    "Spring Dim Green",
    Hue::Green,
    Saturation::S8,
    Brightness::B2,
    Rgb::new(219, 226, 196),
);

pub const COLOR_G85: Color = Color::new(
    "G85",
    "Verdigris",
    Hue::Green,
    Saturation::S8,
    Brightness::B5,
    Rgb::new(179, 205, 181),
);

pub const COLOR_G94: Color = Color::new(
    "G94",
    "Grayish Olive",
    Hue::Green,
    Saturation::S9,
    Brightness::B4,
    Rgb::new(170, 179, 142),
);

pub const COLOR_G99: Color = Color::new(
    "G99",
    "Olive",
    Hue::Green,
    Saturation::S9,
    Brightness::B9,
    Rgb::new(123, 142, 63),
);

pub const COLOR_BG0000: Color = Color::new(
    "BG0000",
    "Snow Green",
    Hue::BlueGreen,
    Saturation::S0,
    Brightness::B000,
    Rgb::new(245, 249, 246),
);

pub const COLOR_BG000: Color = Color::new(
    "BG000",
    "Pale Aqua",
    Hue::BlueGreen,
    Saturation::S0,
    Brightness::B00,
    Rgb::new(247, 251, 249),
);

pub const COLOR_BG01: Color = Color::new(
    "BG01",
    "Aqua Blue",
    Hue::BlueGreen,
    Saturation::S0,
    Brightness::B1,
    Rgb::new(214, 235, 248),
);

pub const COLOR_BG02: Color = Color::new(
    "BG02",
    "New Blue",
    Hue::BlueGreen,
    Saturation::S0,
    Brightness::B2,
    Rgb::new(212, 235, 237),
);

pub const COLOR_BG05: Color = Color::new(
    "BG05",
    "Holiday Blue",
    Hue::BlueGreen,
    Saturation::S0,
    Brightness::B5,
    Rgb::new(156, 213, 230),
);

pub const COLOR_BG07: Color = Color::new(
    "BG07",
    "Petroleum Blue",
    Hue::BlueGreen,
    Saturation::S0,
    Brightness::B7,
    Rgb::new(51, 184, 210),
);

pub const COLOR_BG09: Color = Color::new(
    "BG09",
    "Blue Green",
    Hue::BlueGreen,
    Saturation::S0,
    Brightness::B9,
    Rgb::new(9, 177, 205),
);

pub const COLOR_BG10: Color = Color::new(
    "BG10",
    "Cool Shadow",
    Hue::BlueGreen,
    Saturation::S1,
    Brightness::B0,
    Rgb::new(231, 243, 242),
);

pub const COLOR_BG11: Color = Color::new(
    "BG11",
    "Moon White",
    Hue::BlueGreen,
    Saturation::S1,
    Brightness::B1,
    Rgb::new(218, 238, 242),
);

pub const COLOR_BG13: Color = Color::new(
    "BG13",
    "Mint Green",
    Hue::BlueGreen,
    Saturation::S1,
    Brightness::B3,
    Rgb::new(211, 234, 235),
);

pub const COLOR_BG15: Color = Color::new(
    "BG15",
    "Aqua",
    Hue::BlueGreen,
    Saturation::S1,
    Brightness::B5,
    Rgb::new(181, 221, 214),
);

pub const COLOR_BG18: Color = Color::new(
    "BG18",
    "Teal Blue",
    Hue::BlueGreen,
    Saturation::S1,
    Brightness::B8,
    Rgb::new(86, 190, 179),
);

pub const COLOR_BG23: Color = Color::new(
    "BG23",
    "Coral Sea",
    Hue::BlueGreen,
    Saturation::S2,
    Brightness::B3,
    Rgb::new(205, 231, 224),
);

pub const COLOR_BG32: Color = Color::new(
    "BG32",
    "Aqua Mint",
    Hue::BlueGreen,
    Saturation::S3,
    Brightness::B2,
    Rgb::new(204, 230, 219),
);

pub const COLOR_BG34: Color = Color::new(
    "BG34",
    "Horizon Green",
    Hue::BlueGreen,
    Saturation::S3,
    Brightness::B4,
    Rgb::new(184, 222, 219),
);

pub const COLOR_BG45: Color = Color::new(
    "BG45",
    "Nile Blue",
    Hue::BlueGreen,
    Saturation::S4,
    Brightness::B5,
    Rgb::new(193, 226, 227),
);

pub const COLOR_BG49: Color = Color::new(
    "BG49",
    "Duck Blue",
    Hue::BlueGreen,
    Saturation::S4,
    Brightness::B9,
    Rgb::new(21, 178, 188),
);

pub const COLOR_BG53: Color = Color::new(
    "BG53",
    "Ice Mint",
    Hue::BlueGreen,
    Saturation::S5,
    Brightness::B3,
    Rgb::new(92, 182, 189),
);

pub const COLOR_BG57: Color = Color::new(
    "BG57",
    "Jasper",
    Hue::BlueGreen,
    Saturation::S5,
    Brightness::B7,
    Rgb::new(4, 162, 173),
);

pub const COLOR_BG70: Color = Color::new(
    "BG70",
    "Ocean Mist",
    Hue::BlueGreen,
    Saturation::S7,
    Brightness::B0,
    Rgb::new(229, 238, 227),
);

pub const COLOR_BG72: Color = Color::new(
    "BG72",
    "Ice Ocean",
    Hue::BlueGreen,
    Saturation::S7,
    Brightness::B2,
    Rgb::new(136, 190, 193),
);

pub const COLOR_BG75: Color = Color::new(
    "BG75",
    "Abyss Green",
    Hue::BlueGreen,
    Saturation::S7,
    Brightness::B5,
    Rgb::new(181, 221, 214),
);

pub const COLOR_BG78: Color = Color::new(
    "BG78",
    "Bronze",
    Hue::BlueGreen,
    Saturation::S7,
    Brightness::B8,
    Rgb::new(72, 117, 107),
);

pub const COLOR_BG90: Color = Color::new(
    "BG90",
    "Gray Sky",
    Hue::BlueGreen,
    Saturation::S9,
    Brightness::B0,
    Rgb::new(209, 204, 184),
);

pub const COLOR_BG93: Color = Color::new(
    "BG93",
    "Green Gray",
    Hue::BlueGreen,
    Saturation::S9,
    Brightness::B3,
    Rgb::new(203, 206, 196),
);

pub const COLOR_BG96: Color = Color::new(
    "BG96",
    "Bush",
    Hue::BlueGreen,
    Saturation::S9,
    Brightness::B6,
    Rgb::new(154, 176, 158),
);

pub const COLOR_BG99: Color = Color::new(
    "BG99",
    "Flagstone Blue",
    Hue::BlueGreen,
    Saturation::S9,
    Brightness::B9,
    Rgb::new(137, 169, 150),
);

pub const COLOR_B0000: Color = Color::new(
    "B0000",
    "Pale Celestine",
    Hue::Blue,
    Saturation::S0,
    Brightness::B000,
    Rgb::new(246, 251, 254),
);

pub const COLOR_B000: Color = Color::new(
    "B000",
    "Pale Porcelain Blue",
    Hue::Blue,
    Saturation::S0,
    Brightness::B00,
    Rgb::new(237, 246, 246),
);

pub const COLOR_B00: Color = Color::new(
    "B00",
    "Frost Blue",
    Hue::Blue,
    Saturation::S0,
    Brightness::B0,
    Rgb::new(234, 246, 249),
);

pub const COLOR_B01: Color = Color::new(
    "B01",
    "Mint Blue",
    Hue::Blue,
    Saturation::S0,
    Brightness::B1,
    Rgb::new(225, 241, 243),
);

pub const COLOR_B02: Color = Color::new(
    "B02",
    "Robin's Egg Blue",
    Hue::Blue,
    Saturation::S0,
    Brightness::B2,
    Rgb::new(197, 230, 240),
);

pub const COLOR_B04: Color = Color::new(
    "B04",
    "Tahitian Blue",
    Hue::Blue,
    Saturation::S0,
    Brightness::B4,
    Rgb::new(141, 209, 231),
);

pub const COLOR_B05: Color = Color::new(
    "B05",
    "Process Blue",
    Hue::Blue,
    Saturation::S0,
    Brightness::B5,
    Rgb::new(100, 197, 229),
);

pub const COLOR_B06: Color = Color::new(
    "B06",
    "Peacock Blue",
    Hue::Blue,
    Saturation::S0,
    Brightness::B6,
    Rgb::new(0, 172, 226),
);

pub const COLOR_B12: Color = Color::new(
    "B12",
    "Ice Blue",
    Hue::Blue,
    Saturation::S1,
    Brightness::B2,
    Rgb::new(214, 234, 240),
);

pub const COLOR_B14: Color = Color::new(
    "B14",
    "Light Blue",
    Hue::Blue,
    Saturation::S1,
    Brightness::B4,
    Rgb::new(141, 209, 235),
);

pub const COLOR_B16: Color = Color::new(
    "B16",
    "Chanine Blue",
    Hue::Blue,
    Saturation::S1,
    Brightness::B6,
    Rgb::new(1, 186, 231),
);

pub const COLOR_B18: Color = Color::new(
    "B18",
    "Lapis Lazuli",
    Hue::Blue,
    Saturation::S1,
    Brightness::B8,
    Rgb::new(41, 145, 201),
);

pub const COLOR_B21: Color = Color::new(
    "B21",
    "Baby Blue",
    Hue::Blue,
    Saturation::S2,
    Brightness::B1,
    Rgb::new(230, 241, 250),
);

pub const COLOR_B23: Color = Color::new(
    "B23",
    "Phthalo Blue",
    Hue::Blue,
    Saturation::S2,
    Brightness::B3,
    Rgb::new(171, 203, 233),
);

pub const COLOR_B24: Color = Color::new(
    "B24",
    "Sky",
    Hue::Blue,
    Saturation::S2,
    Brightness::B4,
    Rgb::new(163, 213, 241),
);

pub const COLOR_B26: Color = Color::new(
    "B26",
    "Cobalt Blue",
    Hue::Blue,
    Saturation::S2,
    Brightness::B6,
    Rgb::new(127, 187, 227),
);

pub const COLOR_B28: Color = Color::new(
    "B28",
    "Royal Blue",
    Hue::Blue,
    Saturation::S2,
    Brightness::B8,
    Rgb::new(21, 113, 176),
);

pub const COLOR_B29: Color = Color::new(
    "B29",
    "Ultramarine",
    Hue::Blue,
    Saturation::S2,
    Brightness::B9,
    Rgb::new(0, 119, 186),
);

pub const COLOR_B32: Color = Color::new(
    "B32",
    "Pale Blue",
    Hue::Blue,
    Saturation::S3,
    Brightness::B2,
    Rgb::new(234, 243, 247),
);

pub const COLOR_B34: Color = Color::new(
    "B34",
    "Manganese Blue",
    Hue::Blue,
    Saturation::S3,
    Brightness::B4,
    Rgb::new(155, 203, 235),
);

pub const COLOR_B37: Color = Color::new(
    "B37",
    "Antwerp Blue",
    Hue::Blue,
    Saturation::S3,
    Brightness::B7,
    Rgb::new(4, 114, 163),
);

pub const COLOR_B39: Color = Color::new(
    "B39",
    "Prussian Blue",
    Hue::Blue,
    Saturation::S3,
    Brightness::B9,
    Rgb::new(40, 106, 167),
);

pub const COLOR_B41: Color = Color::new(
    "B41",
    "Powder Blue",
    Hue::Blue,
    Saturation::S4,
    Brightness::B1,
    Rgb::new(234, 243, 251),
);

pub const COLOR_B45: Color = Color::new(
    "B45",
    "Smoky Blue",
    Hue::Blue,
    Saturation::S4,
    Brightness::B5,
    Rgb::new(143, 199, 234),
);

pub const COLOR_B52: Color = Color::new(
    "B52",
    "Soft Greenish Blue",
    Hue::Blue,
    Saturation::S5,
    Brightness::B2,
    Rgb::new(193, 214, 225),
);

pub const COLOR_B60: Color = Color::new(
    "B60",
    "Pale Blue Gray",
    Hue::Blue,
    Saturation::S6,
    Brightness::B0,
    Rgb::new(228, 232, 244),
);

pub const COLOR_B63: Color = Color::new(
    "B63",
    "Light Hydrangea",
    Hue::Blue,
    Saturation::S6,
    Brightness::B3,
    Rgb::new(188, 198, 226),
);

pub const COLOR_B66: Color = Color::new(
    "B66",
    "Clematis",
    Hue::Blue,
    Saturation::S6,
    Brightness::B3,
    Rgb::new(100, 115, 180),
);

pub const COLOR_B69: Color = Color::new(
    "B69",
    "Stratospheric Blue",
    Hue::Blue,
    Saturation::S6,
    Brightness::B9,
    Rgb::new(25, 103, 167),
);

pub const COLOR_B79: Color = Color::new(
    "B79",
    "Iris",
    Hue::Blue,
    Saturation::S7,
    Brightness::B9,
    Rgb::new(49, 65, 143),
);

pub const COLOR_B91: Color = Color::new(
    "B91",
    "Pale Grayish Blue",
    Hue::Blue,
    Saturation::S9,
    Brightness::B1,
    Rgb::new(225, 233, 237),
);

pub const COLOR_B93: Color = Color::new(
    "B93",
    "Light Crockery Blue",
    Hue::Blue,
    Saturation::S9,
    Brightness::B3,
    Rgb::new(171, 203, 223),
);

pub const COLOR_B95: Color = Color::new(
    "B95",
    "Light Grayish Cobalt",
    Hue::Blue,
    Saturation::S9,
    Brightness::B5,
    Rgb::new(136, 176, 202),
);

pub const COLOR_B97: Color = Color::new(
    "B97",
    "Night Blue",
    Hue::Blue,
    Saturation::S9,
    Brightness::B7,
    Rgb::new(69, 128, 157),
);

pub const COLOR_B99: Color = Color::new(
    "B99",
    "Agate",
    Hue::Blue,
    Saturation::S9,
    Brightness::B9,
    Rgb::new(0, 77, 122),
);

pub const COLOR_E0000: Color = Color::new(
    "E0000",
    "Floral White",
    Hue::Earth,
    Saturation::S0,
    Brightness::B000,
    Rgb::new(255, 250, 243),
);

pub const COLOR_E000: Color = Color::new(
    "E000",
    "Pale Fruit Pink",
    Hue::Earth,
    Saturation::S0,
    Brightness::B00,
    Rgb::new(255, 248, 241),
);

pub const COLOR_E00: Color = Color::new(
    "E00",
    "Cotton Pearl",
    Hue::Earth,
    Saturation::S0,
    Brightness::B0,
    Rgb::new(255, 246, 238),
);

pub const COLOR_E01: Color = Color::new(
    "E01",
    "Pink Flamingo",
    Hue::Earth,
    Saturation::S0,
    Brightness::B1,
    Rgb::new(255, 242, 233),
);

pub const COLOR_E02: Color = Color::new(
    "E02",
    "Fruit Pink",
    Hue::Earth,
    Saturation::S0,
    Brightness::B2,
    Rgb::new(255, 241, 230),
);

pub const COLOR_E04: Color = Color::new(
    "E04",
    "Lipstick Rose",
    Hue::Earth,
    Saturation::S0,
    Brightness::B4,
    Rgb::new(236, 202, 206),
);

pub const COLOR_E07: Color = Color::new(
    "E07",
    "Light Mahogany",
    Hue::Earth,
    Saturation::S0,
    Brightness::B7,
    Rgb::new(217, 146, 120),
);

pub const COLOR_E08: Color = Color::new(
    "E08",
    "Brown",
    Hue::Earth,
    Saturation::S0,
    Brightness::B8,
    Rgb::new(213, 116, 92),
);

pub const COLOR_E09: Color = Color::new(
    "E09",
    "Burnt Sienna",
    Hue::Earth,
    Saturation::S0,
    Brightness::B9,
    Rgb::new(226, 116, 83),
);

pub const COLOR_E11: Color = Color::new(
    "E11",
    "Barley Beige",
    Hue::Earth,
    Saturation::S1,
    Brightness::B1,
    Rgb::new(255, 239, 222),
);

pub const COLOR_E13: Color = Color::new(
    "E13",
    "Desert Sand",
    Hue::Earth,
    Saturation::S1,
    Brightness::B3,
    Rgb::new(239, 210, 187),
);

pub const COLOR_E15: Color = Color::new(
    "E15",
    "Earthenware",
    Hue::Earth,
    Saturation::S1,
    Brightness::B5,
    Rgb::new(253, 199, 151),
);

pub const COLOR_E17: Color = Color::new(
    "E17",
    "Reddish Brass",
    Hue::Earth,
    Saturation::S1,
    Brightness::B7,
    Rgb::new(196, 102, 85),
);

pub const COLOR_E18: Color = Color::new(
    "E18",
    "Copper",
    Hue::Earth,
    Saturation::S1,
    Brightness::B8,
    Rgb::new(151, 86, 74),
);

pub const COLOR_E19: Color = Color::new(
    "E19",
    "Redwood",
    Hue::Earth,
    Saturation::S1,
    Brightness::B9,
    Rgb::new(206, 84, 37),
);

pub const COLOR_E21: Color = Color::new(
    "E21",
    "Soft Sun",
    Hue::Earth,
    Saturation::S2,
    Brightness::B1,
    Rgb::new(255, 233, 210),
);

pub const COLOR_E23: Color = Color::new(
    "E23",
    "Hazelnut",
    Hue::Earth,
    Saturation::S2,
    Brightness::B3,
    Rgb::new(170, 106, 75),
);

pub const COLOR_E25: Color = Color::new(
    "E25",
    "Caribe Cocoa",
    Hue::Earth,
    Saturation::S2,
    Brightness::B5,
    Rgb::new(223, 182, 147),
);

pub const COLOR_E27: Color = Color::new(
    "E27",
    "Milk Chocolate",
    Hue::Earth,
    Saturation::S2,
    Brightness::B7,
    Rgb::new(172, 134, 109),
);

pub const COLOR_E29: Color = Color::new(
    "E29",
    "Burnt Umber",
    Hue::Earth,
    Saturation::S2,
    Brightness::B9,
    Rgb::new(144, 63, 10),
);

pub const COLOR_E30: Color = Color::new(
    "E30",
    "Bisque",
    Hue::Earth,
    Saturation::S3,
    Brightness::B0,
    Rgb::new(247, 239, 207),
);

pub const COLOR_E31: Color = Color::new(
    "E31",
    "Brick Beige",
    Hue::Earth,
    Saturation::S3,
    Brightness::B1,
    Rgb::new(246, 236, 215),
);

pub const COLOR_E33: Color = Color::new(
    "E33",
    "Sand",
    Hue::Earth,
    Saturation::S3,
    Brightness::B3,
    Rgb::new(246, 220, 189),
);

pub const COLOR_E34: Color = Color::new(
    "E34",
    "Toast",
    Hue::Earth,
    Saturation::S3,
    Brightness::B4,
    Rgb::new(245, 215, 179),
);

pub const COLOR_E35: Color = Color::new(
    "E35",
    "Chamois",
    Hue::Earth,
    Saturation::S3,
    Brightness::B5,
    Rgb::new(238, 210, 178),
);

pub const COLOR_E37: Color = Color::new(
    "E37",
    "Sepia",
    Hue::Earth,
    Saturation::S3,
    Brightness::B7,
    Rgb::new(217, 165, 102),
);

pub const COLOR_E39: Color = Color::new(
    "E39",
    "Leather",
    Hue::Earth,
    Saturation::S3,
    Brightness::B9,
    Rgb::new(210, 125, 51),
);

pub const COLOR_E40: Color = Color::new(
    "E40",
    "Brick White",
    Hue::Earth,
    Saturation::S4,
    Brightness::B0,
    Rgb::new(247, 240, 229),
);

pub const COLOR_E41: Color = Color::new(
    "E41",
    "Pearl White",
    Hue::Earth,
    Saturation::S4,
    Brightness::B1,
    Rgb::new(255, 244, 232),
);

pub const COLOR_E42: Color = Color::new(
    "E42",
    "Sand White",
    Hue::Earth,
    Saturation::S4,
    Brightness::B2,
    Rgb::new(242, 232, 211),
);

pub const COLOR_E43: Color = Color::new(
    "E43",
    "Dull Ivory",
    Hue::Earth,
    Saturation::S4,
    Brightness::B3,
    Rgb::new(240, 230, 203),
);

pub const COLOR_E44: Color = Color::new(
    "E44",
    "Clay",
    Hue::Earth,
    Saturation::S4,
    Brightness::B4,
    Rgb::new(212, 201, 182),
);

pub const COLOR_E47: Color = Color::new(
    "E47",
    "Dark Brown",
    Hue::Earth,
    Saturation::S4,
    Brightness::B7,
    Rgb::new(154, 130, 108),
);

pub const COLOR_E49: Color = Color::new(
    "E49",
    "Dark Bark",
    Hue::Earth,
    Saturation::S4,
    Brightness::B9,
    Rgb::new(122, 93, 69),
);

pub const COLOR_E50: Color = Color::new(
    "E50",
    "Egg Shell",
    Hue::Earth,
    Saturation::S5,
    Brightness::B0,
    Rgb::new(247, 240, 241),
);

pub const COLOR_E51: Color = Color::new(
    "E51",
    "Milky White",
    Hue::Earth,
    Saturation::S5,
    Brightness::B1,
    Rgb::new(255, 241, 222),
);

pub const COLOR_E53: Color = Color::new(
    "E53",
    "Raw Silk",
    Hue::Earth,
    Saturation::S5,
    Brightness::B3,
    Rgb::new(246, 236, 204),
);

pub const COLOR_E55: Color = Color::new(
    "E55",
    "Light Camel",
    Hue::Earth,
    Saturation::S5,
    Brightness::B5,
    Rgb::new(245, 230, 196),
);

pub const COLOR_E57: Color = Color::new(
    "E57",
    "Light Walnut",
    Hue::Earth,
    Saturation::S5,
    Brightness::B7,
    Rgb::new(194, 154, 106),
);

pub const COLOR_E59: Color = Color::new(
    "E59",
    "Walnut",
    Hue::Earth,
    Saturation::S5,
    Brightness::B9,
    Rgb::new(173, 144, 118),
);

pub const COLOR_E70: Color = Color::new(
    "E70",
    "Ash Rose",
    Hue::Earth,
    Saturation::S7,
    Brightness::B0,
    Rgb::new(241, 233, 226),
);

pub const COLOR_E71: Color = Color::new(
    "E71",
    "Champagne",
    Hue::Earth,
    Saturation::S7,
    Brightness::B1,
    Rgb::new(235, 225, 218),
);

pub const COLOR_E74: Color = Color::new(
    "E74",
    "Cocoa Brown",
    Hue::Earth,
    Saturation::S7,
    Brightness::B4,
    Rgb::new(181, 153, 140),
);

pub const COLOR_E77: Color = Color::new(
    "E77",
    "Maroon",
    Hue::Earth,
    Saturation::S7,
    Brightness::B7,
    Rgb::new(148, 113, 87),
);

pub const COLOR_E79: Color = Color::new(
    "E79",
    "Cashew",
    Hue::Earth,
    Saturation::S7,
    Brightness::B9,
    Rgb::new(82, 56, 45),
);

pub const COLOR_E81: Color = Color::new(
    "E81",
    "Ivory",
    Hue::Earth,
    Saturation::S8,
    Brightness::B1,
    Rgb::new(220, 205, 152),
);

pub const COLOR_E84: Color = Color::new(
    "E84",
    "Khaki",
    Hue::Earth,
    Saturation::S8,
    Brightness::B4,
    Rgb::new(180, 149, 95),
);

pub const COLOR_E87: Color = Color::new(
    "E87",
    "Fig",
    Hue::Earth,
    Saturation::S8,
    Brightness::B7,
    Rgb::new(91, 77, 56),
);

pub const COLOR_E89: Color = Color::new(
    "E89",
    "Pecan",
    Hue::Earth,
    Saturation::S8,
    Brightness::B9,
    Rgb::new(123, 116, 106),
);

pub const COLOR_E93: Color = Color::new(
    "E93",
    "Tea Rose",
    Hue::Earth,
    Saturation::S9,
    Brightness::B3,
    Rgb::new(254, 219, 194),
);

pub const COLOR_E95: Color = Color::new(
    "E95",
    "Tea Orange",
    Hue::Earth,
    Saturation::S9,
    Brightness::B5,
    Rgb::new(254, 199, 136),
);

pub const COLOR_E97: Color = Color::new(
    "E97",
    "Deep Orange",
    Hue::Earth,
    Saturation::S9,
    Brightness::B7,
    Rgb::new(243, 169, 98),
);

pub const COLOR_E99: Color = Color::new(
    "E99",
    "Baked Clay",
    Hue::Earth,
    Saturation::S9,
    Brightness::B9,
    Rgb::new(192, 101, 8),
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
    COLOR_RV0000,
    COLOR_RV000,
    COLOR_RV00,
    COLOR_RV02,
    COLOR_RV04,
    COLOR_RV06,
    COLOR_RV09,
    COLOR_RV10,
    COLOR_RV11,
    COLOR_RV13,
    COLOR_RV14,
    COLOR_RV17,
    COLOR_RV19,
    COLOR_RV21,
    COLOR_RV23,
    COLOR_RV25,
    COLOR_RV29,
    COLOR_RV32,
    COLOR_RV34,
    COLOR_RV42,
    COLOR_RV52,
    COLOR_RV55,
    COLOR_RV63,
    COLOR_RV66,
    COLOR_RV69,
    COLOR_RV91,
    COLOR_RV93,
    COLOR_RV95,
    COLOR_RV99,
    COLOR_R0000,
    COLOR_R000,
    COLOR_R00,
    COLOR_R01,
    COLOR_R02,
    COLOR_R05,
    COLOR_R08,
    COLOR_R11,
    COLOR_R12,
    COLOR_R14,
    COLOR_R17,
    COLOR_R20,
    COLOR_R21,
    COLOR_R22,
    COLOR_R24,
    COLOR_R27,
    COLOR_R29,
    COLOR_R30,
    COLOR_R32,
    COLOR_R35,
    COLOR_R37,
    COLOR_R39,
    COLOR_R43,
    COLOR_R46,
    COLOR_R56,
    COLOR_R59,
    COLOR_R81,
    COLOR_R83,
    COLOR_R85,
    COLOR_R89,
    COLOR_YR0000,
    COLOR_YR000,
    COLOR_YR00,
    COLOR_YR01,
    COLOR_YR02,
    COLOR_YR04,
    COLOR_YR07,
    COLOR_YR09,
    COLOR_YR12,
    COLOR_YR14,
    COLOR_YR15,
    COLOR_YR16,
    COLOR_YR18,
    COLOR_YR20,
    COLOR_YR21,
    COLOR_YR23,
    COLOR_YR24,
    COLOR_YR27,
    COLOR_YR30,
    COLOR_YR31,
    COLOR_YR61,
    COLOR_YR65,
    COLOR_YR68,
    COLOR_YR82,
    COLOR_Y0000,
    COLOR_Y000,
    COLOR_Y00,
    COLOR_Y02,
    COLOR_Y04,
    COLOR_Y06,
    COLOR_Y08,
    COLOR_Y11,
    COLOR_Y13,
    COLOR_Y15,
    COLOR_Y17,
    COLOR_Y18,
    COLOR_Y19,
    COLOR_Y21,
    COLOR_Y23,
    COLOR_Y26,
    COLOR_Y28,
    COLOR_Y32,
    COLOR_Y35,
    COLOR_Y38,
    COLOR_YG0000,
    COLOR_YG00,
    COLOR_YG01,
    COLOR_YG03,
    COLOR_YG05,
    COLOR_YG06,
    COLOR_YG07,
    COLOR_YG09,
    COLOR_YG11,
    COLOR_YG13,
    COLOR_YG17,
    COLOR_YG21,
    COLOR_YG23,
    COLOR_YG25,
    COLOR_YG41,
    COLOR_YG45,
    COLOR_YG61,
    COLOR_YG63,
    COLOR_YG91,
    COLOR_YG93,
    COLOR_YG95,
    COLOR_YG97,
    COLOR_YG99,
    COLOR_G0000,
    COLOR_G000,
    COLOR_G00,
    COLOR_G02,
    COLOR_G03,
    COLOR_G05,
    COLOR_G07,
    COLOR_G09,
    COLOR_G12,
    COLOR_G14,
    COLOR_G16,
    COLOR_G17,
    COLOR_G19,
    COLOR_G20,
    COLOR_G21,
    COLOR_G24,
    COLOR_G28,
    COLOR_G40,
    COLOR_G43,
    COLOR_G46,
    COLOR_G82,
    COLOR_G85,
    COLOR_G94,
    COLOR_G99,
    COLOR_BG0000,
    COLOR_BG000,
    COLOR_BG01,
    COLOR_BG02,
    COLOR_BG05,
    COLOR_BG07,
    COLOR_BG09,
    COLOR_BG10,
    COLOR_BG11,
    COLOR_BG13,
    COLOR_BG15,
    COLOR_BG18,
    COLOR_BG23,
    COLOR_BG32,
    COLOR_BG34,
    COLOR_BG45,
    COLOR_BG49,
    COLOR_BG53,
    COLOR_BG57,
    COLOR_BG70,
    COLOR_BG72,
    COLOR_BG75,
    COLOR_BG78,
    COLOR_BG90,
    COLOR_BG93,
    COLOR_BG96,
    COLOR_BG99,
    COLOR_B0000,
    COLOR_B000,
    COLOR_B00,
    COLOR_B01,
    COLOR_B02,
    COLOR_B04,
    COLOR_B05,
    COLOR_B06,
    COLOR_B12,
    COLOR_B14,
    COLOR_B16,
    COLOR_B18,
    COLOR_B21,
    COLOR_B23,
    COLOR_B24,
    COLOR_B26,
    COLOR_B28,
    COLOR_B29,
    COLOR_B32,
    COLOR_B34,
    COLOR_B37,
    COLOR_B39,
    COLOR_B41,
    COLOR_B45,
    COLOR_B52,
    COLOR_B60,
    COLOR_B63,
    COLOR_B66,
    COLOR_B69,
    COLOR_B79,
    COLOR_B91,
    COLOR_B93,
    COLOR_B95,
    COLOR_B97,
    COLOR_B99,
    COLOR_E0000,
    COLOR_E000,
    COLOR_E00,
    COLOR_E01,
    COLOR_E02,
    COLOR_E04,
    COLOR_E07,
    COLOR_E08,
    COLOR_E09,
    COLOR_E11,
    COLOR_E13,
    COLOR_E15,
    COLOR_E17,
    COLOR_E18,
    COLOR_E19,
    COLOR_E21,
    COLOR_E23,
    COLOR_E25,
    COLOR_E27,
    COLOR_E29,
    COLOR_E30,
    COLOR_E31,
    COLOR_E33,
    COLOR_E34,
    COLOR_E35,
    COLOR_E37,
    COLOR_E39,
    COLOR_E40,
    COLOR_E41,
    COLOR_E42,
    COLOR_E43,
    COLOR_E44,
    COLOR_E47,
    COLOR_E49,
    COLOR_E50,
    COLOR_E51,
    COLOR_E53,
    COLOR_E55,
    COLOR_E57,
    COLOR_E59,
    COLOR_E70,
    COLOR_E71,
    COLOR_E74,
    COLOR_E77,
    COLOR_E79,
    COLOR_E81,
    COLOR_E84,
    COLOR_E87,
    COLOR_E89,
    COLOR_E93,
    COLOR_E95,
    COLOR_E97,
    COLOR_E99,
    // C00, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9,
    // C10, N0, N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
    // W00, W0, W1, W2, W3, W4, W5, W6, W7, W8, W9, W10, // 0, 100, 110
    // FV, FRV, FYR, FY, FYG, FG, FBG, FB,
];
