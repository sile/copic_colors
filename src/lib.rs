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
    // Y0000, Y000, Y00, Y02, Y04, Y06, Y08, Y11, Y13, Y15, Y17, Y18, Y19, Y21, Y23,
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
