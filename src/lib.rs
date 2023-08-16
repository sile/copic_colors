//! Copic color list.
//!
//! About Copic colors, please refer to the following documents:
//! - <https://copic.too.com/blogs/educational/copic-color-system>
//! - <https://copic.too.com/blogs/educational/how-are-copic-colors-organized-and-named>

/// Copic color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub code: &'static str,
    pub name: &'static str,
    pub family: Family,
    pub group: Group,
    pub value: Value,
    pub rgb: Rgb,
}

impl Color {
    const fn new(
        code: &'static str,
        name: &'static str,
        family: Family,
        group: Group,
        value: Value,
        rgb: Rgb,
    ) -> Self {
        Self {
            code,
            name,
            family,
            group,
            value,
            rgb,
        }
    }
}

/// RGB.
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

/// Colro family (aka. hue).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Family {
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
    Flourescent,
    Achromatic,
}

/// Color blending group (aka. saturation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Group {
    Undefined,
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
}

/// Color intensity value (aka. brightness).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value {
    Undefined,
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
}

pub const COLOR_BV0000: Color = Color::new(
    "BV0000",
    "Pale Thistle",
    Family::BlueViolet,
    Group::S0,
    Value::B000,
    Rgb::new(238, 236, 245),
);

pub const COLOR_BV000: Color = Color::new(
    "BV000",
    "Iridescent Mauve",
    Family::BlueViolet,
    Group::S0,
    Value::B00,
    Rgb::new(238, 231, 241),
);

pub const COLOR_BV00: Color = Color::new(
    "BV00",
    "Mauve Shadow",
    Family::BlueViolet,
    Group::S0,
    Value::B0,
    Rgb::new(233, 227, 240),
);

pub const COLOR_BV01: Color = Color::new(
    "BV01",
    "Viola",
    Family::BlueViolet,
    Group::S0,
    Value::B1,
    Rgb::new(200, 196, 223),
);

pub const COLOR_BV02: Color = Color::new(
    "BV02",
    "Prune",
    Family::BlueViolet,
    Group::S0,
    Value::B2,
    Rgb::new(190, 196, 223),
);

pub const COLOR_BV04: Color = Color::new(
    "BV04",
    "Blue Berry",
    Family::BlueViolet,
    Group::S0,
    Value::B4,
    Rgb::new(146, 164, 206),
);

pub const COLOR_BV08: Color = Color::new(
    "BV08",
    "Blue Violet",
    Family::BlueViolet,
    Group::S0,
    Value::B8,
    Rgb::new(176, 140, 185),
);

pub const COLOR_BV11: Color = Color::new(
    "BV11",
    "Soft Violet",
    Family::BlueViolet,
    Group::S1,
    Value::B1,
    Rgb::new(224, 220, 236),
);

pub const COLOR_BV13: Color = Color::new(
    "BV13",
    "Hydrangea Blue",
    Family::BlueViolet,
    Group::S1,
    Value::B3,
    Rgb::new(152, 158, 201),
);

pub const COLOR_BV17: Color = Color::new(
    "BV17",
    "Deep Reddish Blue",
    Family::BlueViolet,
    Group::S1,
    Value::B7,
    Rgb::new(126, 144, 189),
);

pub const COLOR_BV20: Color = Color::new(
    "BV20",
    "Dull Lavender",
    Family::BlueViolet,
    Group::S2,
    Value::B0,
    Rgb::new(220, 227, 242),
);

pub const COLOR_BV23: Color = Color::new(
    "BV23",
    "Grayish Lavender",
    Family::BlueViolet,
    Group::S2,
    Value::B3,
    Rgb::new(196, 205, 225),
);

pub const COLOR_BV25: Color = Color::new(
    "BV25",
    "Grayish Violet",
    Family::BlueViolet,
    Group::S2,
    Value::B5,
    Rgb::new(144, 143, 172),
);

pub const COLOR_BV29: Color = Color::new(
    "BV29",
    "Slate",
    Family::BlueViolet,
    Group::S2,
    Value::B9,
    Rgb::new(46, 66, 86),
);

pub const COLOR_BV31: Color = Color::new(
    "BV31",
    "Pale Lavender",
    Family::BlueViolet,
    Group::S3,
    Value::B1,
    Rgb::new(230, 232, 244),
);

pub const COLOR_BV34: Color = Color::new(
    "BV34",
    "Bluebell",
    Family::BlueViolet,
    Group::S3,
    Value::B4,
    Rgb::new(127, 146, 189),
);

pub const COLOR_V0000: Color = Color::new(
    "V0000",
    "Rose Quartz",
    Family::Violet,
    Group::S0,
    Value::B000,
    Rgb::new(243, 241, 248),
);

pub const COLOR_V000: Color = Color::new(
    "V000",
    "Pale Heath",
    Family::Violet,
    Group::S0,
    Value::B00,
    Rgb::new(247, 242, 247),
);

pub const COLOR_V01: Color = Color::new(
    "V01",
    "Heath",
    Family::Violet,
    Group::S0,
    Value::B1,
    Rgb::new(237, 204, 222),
);

pub const COLOR_V04: Color = Color::new(
    "V04",
    "Lilac",
    Family::Violet,
    Group::S0,
    Value::B4,
    Rgb::new(237, 185, 209),
);

pub const COLOR_V05: Color = Color::new(
    "V05",
    "Azalea",
    Family::Violet,
    Group::S0,
    Value::B5,
    Rgb::new(236, 180, 206),
);

pub const COLOR_V06: Color = Color::new(
    "V06",
    "Lavender",
    Family::Violet,
    Group::S0,
    Value::B6,
    Rgb::new(219, 165, 198),
);

pub const COLOR_V09: Color = Color::new(
    "V09",
    "Violet",
    Family::Violet,
    Group::S0,
    Value::B9,
    Rgb::new(151, 89, 154),
);

pub const COLOR_V12: Color = Color::new(
    "V12",
    "Pale Lilac",
    Family::Violet,
    Group::S1,
    Value::B2,
    Rgb::new(242, 223, 235),
);

pub const COLOR_V15: Color = Color::new(
    "V15",
    "Mallow",
    Family::Violet,
    Group::S1,
    Value::B5,
    Rgb::new(224, 182, 209),
);

pub const COLOR_V17: Color = Color::new(
    "V17",
    "Amethyst",
    Family::Violet,
    Group::S1,
    Value::B7,
    Rgb::new(179, 161, 199),
);

pub const COLOR_V20: Color = Color::new(
    "V20",
    "Wisteria",
    Family::Violet,
    Group::S2,
    Value::B0,
    Rgb::new(213, 204, 214),
);

pub const COLOR_V22: Color = Color::new(
    "V22",
    "Ash Lavender",
    Family::Violet,
    Group::S2,
    Value::B2,
    Rgb::new(123, 111, 144),
);

pub const COLOR_V25: Color = Color::new(
    "V25",
    "Pale Blackberry",
    Family::Violet,
    Group::S2,
    Value::B5,
    Rgb::new(182, 174, 198),
);

pub const COLOR_V28: Color = Color::new(
    "V28",
    "Eggplant",
    Family::Violet,
    Group::S2,
    Value::B8,
    Rgb::new(120, 122, 160),
);

pub const COLOR_V91: Color = Color::new(
    "V91",
    "Pale Grape",
    Family::Violet,
    Group::S9,
    Value::B1,
    Rgb::new(239, 208, 216),
);

pub const COLOR_V93: Color = Color::new(
    "V93",
    "Early Grape",
    Family::Violet,
    Group::S9,
    Value::B3,
    Rgb::new(237, 205, 223),
);

pub const COLOR_V95: Color = Color::new(
    "V95",
    "Light Grape",
    Family::Violet,
    Group::S9,
    Value::B5,
    Rgb::new(197, 137, 170),
);

pub const COLOR_V99: Color = Color::new(
    "V99",
    "Aubergine",
    Family::Violet,
    Group::S9,
    Value::B9,
    Rgb::new(79, 51, 77),
);

pub const COLOR_RV0000: Color = Color::new(
    "RV0000",
    "Evening Primrose",
    Family::RedViolet,
    Group::S0,
    Value::B000,
    Rgb::new(246, 239, 246),
);

pub const COLOR_RV000: Color = Color::new(
    "RV000",
    "Pale Purple",
    Family::RedViolet,
    Group::S0,
    Value::B00,
    Rgb::new(247, 233, 241),
);

pub const COLOR_RV00: Color = Color::new(
    "RV00",
    "Water Lily",
    Family::RedViolet,
    Group::S0,
    Value::B0,
    Rgb::new(243, 211, 233),
);

pub const COLOR_RV02: Color = Color::new(
    "RV02",
    "Sugared Almond Pink",
    Family::RedViolet,
    Group::S0,
    Value::B2,
    Rgb::new(252, 222, 233),
);

pub const COLOR_RV04: Color = Color::new(
    "RV04",
    "Shock Pink",
    Family::RedViolet,
    Group::S0,
    Value::B4,
    Rgb::new(249, 176, 197),
);

pub const COLOR_RV06: Color = Color::new(
    "RV06",
    "Cerise",
    Family::RedViolet,
    Group::S0,
    Value::B6,
    Rgb::new(238, 134, 174),
);

pub const COLOR_RV09: Color = Color::new(
    "RV09",
    "Fuchsia",
    Family::RedViolet,
    Group::S0,
    Value::B9,
    Rgb::new(232, 124, 172),
);

pub const COLOR_RV10: Color = Color::new(
    "RV10",
    "Pale Pink",
    Family::RedViolet,
    Group::S1,
    Value::B0,
    Rgb::new(254, 241, 245),
);

pub const COLOR_RV11: Color = Color::new(
    "RV11",
    "Pink",
    Family::RedViolet,
    Group::S1,
    Value::B1,
    Rgb::new(252, 223, 227),
);

pub const COLOR_RV13: Color = Color::new(
    "RV13",
    "Tender Pink",
    Family::RedViolet,
    Group::S1,
    Value::B3,
    Rgb::new(252, 211, 222),
);

pub const COLOR_RV14: Color = Color::new(
    "RV14",
    "Begonia Pink",
    Family::RedViolet,
    Group::S1,
    Value::B4,
    Rgb::new(248, 160, 188),
);

pub const COLOR_RV17: Color = Color::new(
    "RV17",
    "Deep Magenta",
    Family::RedViolet,
    Group::S1,
    Value::B7,
    Rgb::new(229, 139, 181),
);

pub const COLOR_RV19: Color = Color::new(
    "RV19",
    "Red Violet",
    Family::RedViolet,
    Group::S1,
    Value::B9,
    Rgb::new(222, 115, 167),
);

pub const COLOR_RV21: Color = Color::new(
    "RV21",
    "Light Pink",
    Family::RedViolet,
    Group::S2,
    Value::B1,
    Rgb::new(254, 238, 237),
);

pub const COLOR_RV23: Color = Color::new(
    "RV23",
    "Pure Pink",
    Family::RedViolet,
    Group::S2,
    Value::B3,
    Rgb::new(251, 198, 207),
);

pub const COLOR_RV25: Color = Color::new(
    "RV25",
    "Dog Rose Flower",
    Family::RedViolet,
    Group::S2,
    Value::B5,
    Rgb::new(248, 161, 193),
);

pub const COLOR_RV29: Color = Color::new(
    "RV29",
    "Crimson",
    Family::RedViolet,
    Group::S2,
    Value::B9,
    Rgb::new(241, 74, 129),
);

pub const COLOR_RV32: Color = Color::new(
    "RV32",
    "Shadow Pink",
    Family::RedViolet,
    Group::S3,
    Value::B2,
    Rgb::new(253, 220, 215),
);

pub const COLOR_RV34: Color = Color::new(
    "RV34",
    "Dark Pink",
    Family::RedViolet,
    Group::S3,
    Value::B4,
    Rgb::new(251, 188, 184),
);

pub const COLOR_RV42: Color = Color::new(
    "RV42",
    "Salmon Pink",
    Family::RedViolet,
    Group::S4,
    Value::B2,
    Rgb::new(251, 199, 191),
);

pub const COLOR_RV52: Color = Color::new(
    "RV52",
    "Cotton Candy",
    Family::RedViolet,
    Group::S5,
    Value::B2,
    Rgb::new(220, 156, 182),
);

pub const COLOR_RV55: Color = Color::new(
    "RV55",
    "Hollyhock",
    Family::RedViolet,
    Group::S5,
    Value::B5,
    Rgb::new(217, 97, 150),
);

pub const COLOR_RV63: Color = Color::new(
    "RV63",
    "Begonia",
    Family::RedViolet,
    Group::S6,
    Value::B3,
    Rgb::new(219, 138, 181),
);

pub const COLOR_RV66: Color = Color::new(
    "RV66",
    "Raspberry",
    Family::RedViolet,
    Group::S6,
    Value::B6,
    Rgb::new(141, 55, 101),
);

pub const COLOR_RV69: Color = Color::new(
    "RV69",
    "Peony",
    Family::RedViolet,
    Group::S6,
    Value::B9,
    Rgb::new(156, 98, 118),
);

pub const COLOR_RV91: Color = Color::new(
    "RV91",
    "Grayish Cherry",
    Family::RedViolet,
    Group::S9,
    Value::B1,
    Rgb::new(237, 222, 232),
);

pub const COLOR_RV93: Color = Color::new(
    "RV93",
    "Smoky Purple",
    Family::RedViolet,
    Group::S9,
    Value::B3,
    Rgb::new(238, 195, 210),
);

pub const COLOR_RV95: Color = Color::new(
    "RV95",
    "Baby Blossoms",
    Family::RedViolet,
    Group::S9,
    Value::B5,
    Rgb::new(198, 148, 170),
);

pub const COLOR_RV99: Color = Color::new(
    "RV99",
    "Argyle Purple",
    Family::RedViolet,
    Group::S9,
    Value::B9,
    Rgb::new(110, 87, 100),
);

pub const COLOR_R0000: Color = Color::new(
    "R0000",
    "Pink Beryl",
    Family::Red,
    Group::S0,
    Value::B000,
    Rgb::new(252, 246, 242),
);

pub const COLOR_R000: Color = Color::new(
    "R000",
    "Cherry White",
    Family::Red,
    Group::S0,
    Value::B00,
    Rgb::new(255, 246, 242),
);

pub const COLOR_R00: Color = Color::new(
    "R00",
    "Pinkish White",
    Family::Red,
    Group::S0,
    Value::B0,
    Rgb::new(255, 245, 239),
);

pub const COLOR_R01: Color = Color::new(
    "R01",
    "Pinkish Vanilla",
    Family::Red,
    Group::S0,
    Value::B1,
    Rgb::new(253, 224, 218),
);

pub const COLOR_R02: Color = Color::new(
    "R02",
    "Rose Salmon",
    Family::Red,
    Group::S0,
    Value::B2,
    Rgb::new(254, 220, 208),
);

pub const COLOR_R05: Color = Color::new(
    "R05",
    "Salmon Red",
    Family::Red,
    Group::S0,
    Value::B5,
    Rgb::new(248, 157, 133),
);

pub const COLOR_R08: Color = Color::new(
    "R08",
    "Vermilion",
    Family::Red,
    Group::S0,
    Value::B8,
    Rgb::new(244, 109, 86),
);

pub const COLOR_R11: Color = Color::new(
    "R11",
    "Pale Cherry Pink",
    Family::Red,
    Group::S1,
    Value::B1,
    Rgb::new(255, 232, 221),
);

pub const COLOR_R12: Color = Color::new(
    "R12",
    "Light Tea Rose",
    Family::Red,
    Group::S1,
    Value::B2,
    Rgb::new(254, 220, 203),
);

pub const COLOR_R14: Color = Color::new(
    "R14",
    "Light Rouge",
    Family::Red,
    Group::S1,
    Value::B4,
    Rgb::new(249, 168, 157),
);

pub const COLOR_R17: Color = Color::new(
    "R17",
    "Lipstick Orange",
    Family::Red,
    Group::S1,
    Value::B7,
    Rgb::new(247, 143, 114),
);

pub const COLOR_R20: Color = Color::new(
    "R20",
    "Blush",
    Family::Red,
    Group::S2,
    Value::B0,
    Rgb::new(254, 224, 217),
);

pub const COLOR_R21: Color = Color::new(
    "R21",
    "Sardonyx",
    Family::Red,
    Group::S2,
    Value::B1,
    Rgb::new(252, 182, 172),
);

pub const COLOR_R22: Color = Color::new(
    "R22",
    "Light Prawn",
    Family::Red,
    Group::S2,
    Value::B2,
    Rgb::new(255, 153, 153),
);

pub const COLOR_R24: Color = Color::new(
    "R24",
    "Prawn",
    Family::Red,
    Group::S2,
    Value::B4,
    Rgb::new(245, 127, 130),
);

pub const COLOR_R27: Color = Color::new(
    "R27",
    "Cadmium Red",
    Family::Red,
    Group::S2,
    Value::B7,
    Rgb::new(242, 83, 100),
);

pub const COLOR_R29: Color = Color::new(
    "R29",
    "Lipstick Red",
    Family::Red,
    Group::S2,
    Value::B9,
    Rgb::new(239, 0, 71),
);

pub const COLOR_R30: Color = Color::new(
    "R30",
    "Pale Yellowish Pink",
    Family::Red,
    Group::S3,
    Value::B0,
    Rgb::new(254, 234, 229),
);

pub const COLOR_R32: Color = Color::new(
    "R32",
    "Peach",
    Family::Red,
    Group::S3,
    Value::B2,
    Rgb::new(252, 204, 196),
);

pub const COLOR_R35: Color = Color::new(
    "R35",
    "Coral",
    Family::Red,
    Group::S3,
    Value::B5,
    Rgb::new(245, 122, 138),
);

pub const COLOR_R37: Color = Color::new(
    "R37",
    "Carmine",
    Family::Red,
    Group::S3,
    Value::B7,
    Rgb::new(237, 120, 125),
);

pub const COLOR_R39: Color = Color::new(
    "R39",
    "Garnet",
    Family::Red,
    Group::S3,
    Value::B9,
    Rgb::new(214, 83, 130),
);

pub const COLOR_R43: Color = Color::new(
    "R43",
    "Bougainvillaea",
    Family::Red,
    Group::S4,
    Value::B3,
    Rgb::new(241, 143, 150),
);

pub const COLOR_R46: Color = Color::new(
    "R46",
    "Strong Red",
    Family::Red,
    Group::S4,
    Value::B6,
    Rgb::new(230, 80, 109),
);

pub const COLOR_R56: Color = Color::new(
    "R56",
    "Currant",
    Family::Red,
    Group::S5,
    Value::B6,
    Rgb::new(195, 102, 121),
);

pub const COLOR_R59: Color = Color::new(
    "R59",
    "Cardinal",
    Family::Red,
    Group::S5,
    Value::B9,
    Rgb::new(197, 95, 124),
);

pub const COLOR_R81: Color = Color::new(
    "R81",
    "Rose Pink",
    Family::Red,
    Group::S8,
    Value::B1,
    Rgb::new(246, 212, 220),
);

pub const COLOR_R83: Color = Color::new(
    "R83",
    "Rose Mist",
    Family::Red,
    Group::S8,
    Value::B3,
    Rgb::new(245, 170, 191),
);

pub const COLOR_R85: Color = Color::new(
    "R85",
    "Rose Red",
    Family::Red,
    Group::S8,
    Value::B5,
    Rgb::new(222, 118, 154),
);

pub const COLOR_R89: Color = Color::new(
    "R89",
    "Dark Red",
    Family::Red,
    Group::S8,
    Value::B9,
    Rgb::new(145, 53, 77),
);

pub const COLOR_YR0000: Color = Color::new(
    "YR0000",
    "Pale Chiffon",
    Family::YellowRed,
    Group::S0,
    Value::B000,
    Rgb::new(252, 246, 235),
);

pub const COLOR_YR000: Color = Color::new(
    "YR000",
    "Silk",
    Family::YellowRed,
    Group::S0,
    Value::B00,
    Rgb::new(255, 241, 225),
);

pub const COLOR_YR00: Color = Color::new(
    "YR00",
    "Powder Pink",
    Family::YellowRed,
    Group::S0,
    Value::B0,
    Rgb::new(254, 223, 199),
);

pub const COLOR_YR01: Color = Color::new(
    "YR01",
    "Peach Puff",
    Family::YellowRed,
    Group::S0,
    Value::B1,
    Rgb::new(253, 214, 183),
);

pub const COLOR_YR02: Color = Color::new(
    "YR02",
    "Light Orange",
    Family::YellowRed,
    Group::S0,
    Value::B2,
    Rgb::new(255, 228, 207),
);

pub const COLOR_YR04: Color = Color::new(
    "YR04",
    "Chrome Orange",
    Family::YellowRed,
    Group::S0,
    Value::B4,
    Rgb::new(255, 207, 109),
);

pub const COLOR_YR07: Color = Color::new(
    "YR07",
    "Cadmium Orange",
    Family::YellowRed,
    Group::S0,
    Value::B7,
    Rgb::new(246, 120, 51),
);

pub const COLOR_YR09: Color = Color::new(
    "YR09",
    "Chinese Orange",
    Family::YellowRed,
    Group::S0,
    Value::B7,
    Rgb::new(243, 87, 0),
);

pub const COLOR_YR12: Color = Color::new(
    "YR12",
    "Loquat",
    Family::YellowRed,
    Group::S1,
    Value::B2,
    Rgb::new(254, 227, 145),
);

pub const COLOR_YR14: Color = Color::new(
    "YR14",
    "Caramel",
    Family::YellowRed,
    Group::S1,
    Value::B4,
    Rgb::new(255, 211, 76),
);

pub const COLOR_YR15: Color = Color::new(
    "YR15",
    "Pumpkin Yellow",
    Family::YellowRed,
    Group::S1,
    Value::B5,
    Rgb::new(255, 209, 152),
);

pub const COLOR_YR16: Color = Color::new(
    "YR16",
    "Apricot",
    Family::YellowRed,
    Group::S1,
    Value::B6,
    Rgb::new(255, 196, 23),
);

pub const COLOR_YR18: Color = Color::new(
    "YR18",
    "Sanguine",
    Family::YellowRed,
    Group::S1,
    Value::B8,
    Rgb::new(245, 115, 53),
);

pub const COLOR_YR20: Color = Color::new(
    "YR20",
    "Yellowish Shade",
    Family::YellowRed,
    Group::S2,
    Value::B0,
    Rgb::new(255, 233, 201),
);

pub const COLOR_YR21: Color = Color::new(
    "YR21",
    "Cream",
    Family::YellowRed,
    Group::S2,
    Value::B1,
    Rgb::new(248, 229, 187),
);

pub const COLOR_YR23: Color = Color::new(
    "YR23",
    "Yellow Ochre",
    Family::YellowRed,
    Group::S2,
    Value::B3,
    Rgb::new(242, 219, 152),
);

pub const COLOR_YR24: Color = Color::new(
    "YR24",
    "Pale Sepia",
    Family::YellowRed,
    Group::S2,
    Value::B4,
    Rgb::new(246, 220, 107),
);

pub const COLOR_YR27: Color = Color::new(
    "YR27",
    "Tuscan Orange",
    Family::YellowRed,
    Group::S2,
    Value::B7,
    Rgb::new(216, 103, 69),
);

pub const COLOR_YR30: Color = Color::new(
    "YR30",
    "Macadamia nut",
    Family::YellowRed,
    Group::S3,
    Value::B0,
    Rgb::new(247, 237, 201),
);

pub const COLOR_YR31: Color = Color::new(
    "YR31",
    "Light Reddish Yellow",
    Family::YellowRed,
    Group::S3,
    Value::B1,
    Rgb::new(255, 230, 178),
);

pub const COLOR_YR61: Color = Color::new(
    "YR61",
    "Spring Orange",
    Family::YellowRed,
    Group::S6,
    Value::B1,
    Rgb::new(254, 226, 204),
);

pub const COLOR_YR65: Color = Color::new(
    "YR65",
    "Atoll",
    Family::YellowRed,
    Group::S6,
    Value::B5,
    Rgb::new(254, 187, 101),
);

pub const COLOR_YR68: Color = Color::new(
    "YR68",
    "Orange",
    Family::YellowRed,
    Group::S6,
    Value::B8,
    Rgb::new(246, 119, 0),
);

pub const COLOR_YR82: Color = Color::new(
    "YR82",
    "Mellow Peach",
    Family::YellowRed,
    Group::S8,
    Value::B2,
    Rgb::new(255, 209, 152),
);

pub const COLOR_Y0000: Color = Color::new(
    "Y0000",
    "Yellow Fluorite",
    Family::Yellow,
    Group::S0,
    Value::B000,
    Rgb::new(255, 254, 247),
);

pub const COLOR_Y000: Color = Color::new(
    "Y000",
    "Pale Lemon",
    Family::Yellow,
    Group::S0,
    Value::B00,
    Rgb::new(255, 255, 246),
);

pub const COLOR_Y00: Color = Color::new(
    "Y00",
    "Barium Yellow",
    Family::Yellow,
    Group::S0,
    Value::B0,
    Rgb::new(255, 254, 229),
);

pub const COLOR_Y02: Color = Color::new(
    "Y02",
    "Canary Yellow",
    Family::Yellow,
    Group::S0,
    Value::B2,
    Rgb::new(249, 245, 160),
);

pub const COLOR_Y04: Color = Color::new(
    "Y04",
    "Acacia",
    Family::Yellow,
    Group::S0,
    Value::B4,
    Rgb::new(243, 233, 83),
);

pub const COLOR_Y06: Color = Color::new(
    "Y06",
    "Yellow",
    Family::Yellow,
    Group::S0,
    Value::B6,
    Rgb::new(255, 247, 114),
);

pub const COLOR_Y08: Color = Color::new(
    "Y08",
    "Acid Yellow",
    Family::Yellow,
    Group::S0,
    Value::B8,
    Rgb::new(255, 245, 0),
);

pub const COLOR_Y11: Color = Color::new(
    "Y11",
    "Pale Yellow",
    Family::Yellow,
    Group::S1,
    Value::B1,
    Rgb::new(255, 252, 211),
);

pub const COLOR_Y13: Color = Color::new(
    "Y13",
    "Lemon Yellow",
    Family::Yellow,
    Group::S1,
    Value::B3,
    Rgb::new(252, 249, 183),
);

pub const COLOR_Y15: Color = Color::new(
    "Y15",
    "Cadmium Yellow",
    Family::Yellow,
    Group::S1,
    Value::B5,
    Rgb::new(255, 238, 114),
);

pub const COLOR_Y17: Color = Color::new(
    "Y17",
    "Golden Yellow",
    Family::Yellow,
    Group::S1,
    Value::B7,
    Rgb::new(255, 234, 85),
);

pub const COLOR_Y18: Color = Color::new(
    "Y18",
    "Lightning Yellow",
    Family::Yellow,
    Group::S1,
    Value::B8,
    Rgb::new(255, 242, 87),
);

pub const COLOR_Y19: Color = Color::new(
    "Y19",
    "Napoli Yellow",
    Family::Yellow,
    Group::S1,
    Value::B9,
    Rgb::new(255, 238, 57),
);

pub const COLOR_Y21: Color = Color::new(
    "Y21",
    "Buttercup Yellow",
    Family::Yellow,
    Group::S2,
    Value::B1,
    Rgb::new(255, 242, 201),
);

pub const COLOR_Y23: Color = Color::new(
    "Y23",
    "Yellowish Beige",
    Family::Yellow,
    Group::S2,
    Value::B3,
    Rgb::new(253, 234, 190),
);

pub const COLOR_Y26: Color = Color::new(
    "Y26",
    "Mustard",
    Family::Yellow,
    Group::S2,
    Value::B6,
    Rgb::new(246, 229, 111),
);

pub const COLOR_Y28: Color = Color::new(
    "Y28",
    "Lionet Gold",
    Family::Yellow,
    Group::S2,
    Value::B8,
    Rgb::new(213, 181, 110),
);

pub const COLOR_Y32: Color = Color::new(
    "Y32",
    "Cashmere",
    Family::Yellow,
    Group::S3,
    Value::B2,
    Rgb::new(251, 230, 202),
);

pub const COLOR_Y35: Color = Color::new(
    "Y35",
    "Maize",
    Family::Yellow,
    Group::S3,
    Value::B5,
    Rgb::new(255, 224, 129),
);

pub const COLOR_Y38: Color = Color::new(
    "Y38",
    "Honey",
    Family::Yellow,
    Group::S3,
    Value::B8,
    Rgb::new(255, 255, 153),
);

pub const COLOR_YG0000: Color = Color::new(
    "YG0000",
    "Lily White",
    Family::YellowGreen,
    Group::S0,
    Value::B000,
    Rgb::new(247, 249, 228),
);

pub const COLOR_YG00: Color = Color::new(
    "YG00",
    "Mimosa Yellow",
    Family::YellowGreen,
    Group::S0,
    Value::B0,
    Rgb::new(238, 235, 166),
);

pub const COLOR_YG01: Color = Color::new(
    "YG01",
    "Green Bice",
    Family::YellowGreen,
    Group::S0,
    Value::B1,
    Rgb::new(238, 242, 200),
);

pub const COLOR_YG03: Color = Color::new(
    "YG03",
    "Yellow Green",
    Family::YellowGreen,
    Group::S0,
    Value::B3,
    Rgb::new(234, 238, 178),
);

pub const COLOR_YG05: Color = Color::new(
    "YG05",
    "Salad",
    Family::YellowGreen,
    Group::S0,
    Value::B5,
    Rgb::new(226, 233, 153),
);

pub const COLOR_YG06: Color = Color::new(
    "YG06",
    "Yellowish Green",
    Family::YellowGreen,
    Group::S0,
    Value::B6,
    Rgb::new(211, 227, 152),
);

pub const COLOR_YG07: Color = Color::new(
    "YG07",
    "Acid Green",
    Family::YellowGreen,
    Group::S0,
    Value::B7,
    Rgb::new(182, 209, 53),
);

pub const COLOR_YG09: Color = Color::new(
    "YG09",
    "Lettuce Green",
    Family::YellowGreen,
    Group::S0,
    Value::B9,
    Rgb::new(147, 197, 96),
);

pub const COLOR_YG11: Color = Color::new(
    "YG11",
    "Mignonette",
    Family::YellowGreen,
    Group::S1,
    Value::B1,
    Rgb::new(236, 243, 213),
);

pub const COLOR_YG13: Color = Color::new(
    "YG13",
    "Chartreuse",
    Family::YellowGreen,
    Group::S1,
    Value::B3,
    Rgb::new(223, 233, 166),
);

pub const COLOR_YG17: Color = Color::new(
    "YG17",
    "Grass Green",
    Family::YellowGreen,
    Group::S1,
    Value::B7,
    Rgb::new(122, 191, 74),
);

pub const COLOR_YG21: Color = Color::new(
    "YG21",
    "Anise",
    Family::YellowGreen,
    Group::S2,
    Value::B1,
    Rgb::new(249, 248, 198),
);

pub const COLOR_YG23: Color = Color::new(
    "YG23",
    "New Leaf",
    Family::YellowGreen,
    Group::S2,
    Value::B3,
    Rgb::new(239, 239, 152),
);

pub const COLOR_YG25: Color = Color::new(
    "YG25",
    "Celadon Green",
    Family::YellowGreen,
    Group::S2,
    Value::B5,
    Rgb::new(220, 228, 127),
);

pub const COLOR_YG41: Color = Color::new(
    "YG41",
    "Pale Cobalt Green",
    Family::YellowGreen,
    Group::S4,
    Value::B1,
    Rgb::new(225, 238, 217),
);

pub const COLOR_YG45: Color = Color::new(
    "YG45",
    "Cobalt Green",
    Family::YellowGreen,
    Group::S4,
    Value::B5,
    Rgb::new(197, 224, 190),
);

pub const COLOR_YG61: Color = Color::new(
    "YG61",
    "Pale Moss",
    Family::YellowGreen,
    Group::S6,
    Value::B1,
    Rgb::new(183, 202, 144),
);

pub const COLOR_YG63: Color = Color::new(
    "YG63",
    "Pea Green",
    Family::YellowGreen,
    Group::S6,
    Value::B3,
    Rgb::new(181, 210, 171),
);

pub const COLOR_YG67: Color = Color::new(
    "YG67",
    "Moss",
    Family::YellowGreen,
    Group::S6,
    Value::B7,
    Rgb::new(151, 197, 146),
);

pub const COLOR_YG91: Color = Color::new(
    "YG91",
    "Putty",
    Family::YellowGreen,
    Group::S9,
    Value::B1,
    Rgb::new(228, 223, 184),
);

pub const COLOR_YG93: Color = Color::new(
    "YG93",
    "Grayish Yellow",
    Family::YellowGreen,
    Group::S9,
    Value::B3,
    Rgb::new(222, 219, 166),
);

pub const COLOR_YG95: Color = Color::new(
    "YG95",
    "Pale Olive",
    Family::YellowGreen,
    Group::S9,
    Value::B5,
    Rgb::new(219, 210, 103),
);

pub const COLOR_YG97: Color = Color::new(
    "YG97",
    "Spanish Olive",
    Family::YellowGreen,
    Group::S9,
    Value::B7,
    Rgb::new(170, 160, 0),
);

pub const COLOR_YG99: Color = Color::new(
    "YG99",
    "Marine Green",
    Family::YellowGreen,
    Group::S9,
    Value::B9,
    Rgb::new(106, 120, 0),
);

pub const COLOR_G0000: Color = Color::new(
    "G0000",
    "Crystal Opal",
    Family::Green,
    Group::S0,
    Value::B000,
    Rgb::new(246, 250, 246),
);

pub const COLOR_G000: Color = Color::new(
    "G000",
    "Pale Green",
    Family::Green,
    Group::S0,
    Value::B00,
    Rgb::new(247, 251, 247),
);

pub const COLOR_G00: Color = Color::new(
    "G00",
    "Jade Green",
    Family::Green,
    Group::S0,
    Value::B0,
    Rgb::new(234, 245, 246),
);

pub const COLOR_G02: Color = Color::new(
    "G02",
    "Spectrum Green",
    Family::Green,
    Group::S0,
    Value::B2,
    Rgb::new(219, 236, 217),
);

pub const COLOR_G03: Color = Color::new(
    "G03",
    "Meadow Green",
    Family::Green,
    Group::S0,
    Value::B3,
    Rgb::new(176, 222, 127),
);

pub const COLOR_G05: Color = Color::new(
    "G05",
    "Emerald Green",
    Family::Green,
    Group::S0,
    Value::B5,
    Rgb::new(125, 192, 121),
);

pub const COLOR_G07: Color = Color::new(
    "G07",
    "Nile Green",
    Family::Green,
    Group::S0,
    Value::B7,
    Rgb::new(143, 198, 118),
);

pub const COLOR_G09: Color = Color::new(
    "G09",
    "Veronese Green",
    Family::Green,
    Group::S0,
    Value::B9,
    Rgb::new(143, 196, 96),
);

pub const COLOR_G12: Color = Color::new(
    "G12",
    "Sea Green",
    Family::Green,
    Group::S1,
    Value::B2,
    Rgb::new(222, 236, 203),
);

pub const COLOR_G14: Color = Color::new(
    "G14",
    "Apple Green",
    Family::Green,
    Group::S1,
    Value::B4,
    Rgb::new(170, 210, 148),
);

pub const COLOR_G16: Color = Color::new(
    "G16",
    "Malachite",
    Family::Green,
    Group::S1,
    Value::B6,
    Rgb::new(118, 193, 156),
);

pub const COLOR_G17: Color = Color::new(
    "G17",
    "Forest Green",
    Family::Green,
    Group::S1,
    Value::B7,
    Rgb::new(39, 174, 125),
);

pub const COLOR_G19: Color = Color::new(
    "G19",
    "Bright Parrot Green",
    Family::Green,
    Group::S1,
    Value::B9,
    Rgb::new(73, 182, 138),
);

pub const COLOR_G20: Color = Color::new(
    "G20",
    "Wax White",
    Family::Green,
    Group::S2,
    Value::B0,
    Rgb::new(242, 247, 224),
);

pub const COLOR_G21: Color = Color::new(
    "G21",
    "Lime Green",
    Family::Green,
    Group::S2,
    Value::B1,
    Rgb::new(211, 232, 211),
);

pub const COLOR_G24: Color = Color::new(
    "G24",
    "Willow",
    Family::Green,
    Group::S2,
    Value::B4,
    Rgb::new(209, 228, 187),
);

pub const COLOR_G28: Color = Color::new(
    "G28",
    "Ocean Green",
    Family::Green,
    Group::S2,
    Value::B8,
    Rgb::new(35, 150, 101),
);

pub const COLOR_G29: Color = Color::new(
    "G29",
    "Pine Tree Green",
    Family::Green,
    Group::S2,
    Value::B9,
    Rgb::new(62, 134, 103),
);

pub const COLOR_G40: Color = Color::new(
    "G40",
    "Dim Green",
    Family::Green,
    Group::S4,
    Value::B0,
    Rgb::new(236, 244, 227),
);

pub const COLOR_G43: Color = Color::new(
    "G43",
    "Pistachio",
    Family::Green,
    Group::S4,
    Value::B3,
    Rgb::new(146, 179, 92),
);

pub const COLOR_G46: Color = Color::new(
    "G46",
    "Mistletoe",
    Family::Green,
    Group::S4,
    Value::B6,
    Rgb::new(84, 137, 93),
);

pub const COLOR_G82: Color = Color::new(
    "G82",
    "Spring Dim Green",
    Family::Green,
    Group::S8,
    Value::B2,
    Rgb::new(219, 226, 196),
);

pub const COLOR_G85: Color = Color::new(
    "G85",
    "Verdigris",
    Family::Green,
    Group::S8,
    Value::B5,
    Rgb::new(179, 205, 181),
);

pub const COLOR_G94: Color = Color::new(
    "G94",
    "Grayish Olive",
    Family::Green,
    Group::S9,
    Value::B4,
    Rgb::new(170, 179, 142),
);

pub const COLOR_G99: Color = Color::new(
    "G99",
    "Olive",
    Family::Green,
    Group::S9,
    Value::B9,
    Rgb::new(123, 142, 63),
);

pub const COLOR_BG0000: Color = Color::new(
    "BG0000",
    "Snow Green",
    Family::BlueGreen,
    Group::S0,
    Value::B000,
    Rgb::new(245, 249, 246),
);

pub const COLOR_BG000: Color = Color::new(
    "BG000",
    "Pale Aqua",
    Family::BlueGreen,
    Group::S0,
    Value::B00,
    Rgb::new(247, 251, 249),
);

pub const COLOR_BG01: Color = Color::new(
    "BG01",
    "Aqua Blue",
    Family::BlueGreen,
    Group::S0,
    Value::B1,
    Rgb::new(214, 235, 248),
);

pub const COLOR_BG02: Color = Color::new(
    "BG02",
    "New Blue",
    Family::BlueGreen,
    Group::S0,
    Value::B2,
    Rgb::new(212, 235, 237),
);

pub const COLOR_BG05: Color = Color::new(
    "BG05",
    "Holiday Blue",
    Family::BlueGreen,
    Group::S0,
    Value::B5,
    Rgb::new(156, 213, 230),
);

pub const COLOR_BG07: Color = Color::new(
    "BG07",
    "Petroleum Blue",
    Family::BlueGreen,
    Group::S0,
    Value::B7,
    Rgb::new(51, 184, 210),
);

pub const COLOR_BG09: Color = Color::new(
    "BG09",
    "Blue Green",
    Family::BlueGreen,
    Group::S0,
    Value::B9,
    Rgb::new(9, 177, 205),
);

pub const COLOR_BG10: Color = Color::new(
    "BG10",
    "Cool Shadow",
    Family::BlueGreen,
    Group::S1,
    Value::B0,
    Rgb::new(231, 243, 242),
);

pub const COLOR_BG11: Color = Color::new(
    "BG11",
    "Moon White",
    Family::BlueGreen,
    Group::S1,
    Value::B1,
    Rgb::new(218, 238, 242),
);

pub const COLOR_BG13: Color = Color::new(
    "BG13",
    "Mint Green",
    Family::BlueGreen,
    Group::S1,
    Value::B3,
    Rgb::new(211, 234, 235),
);

pub const COLOR_BG15: Color = Color::new(
    "BG15",
    "Aqua",
    Family::BlueGreen,
    Group::S1,
    Value::B5,
    Rgb::new(181, 221, 214),
);

pub const COLOR_BG18: Color = Color::new(
    "BG18",
    "Teal Blue",
    Family::BlueGreen,
    Group::S1,
    Value::B8,
    Rgb::new(86, 190, 179),
);

pub const COLOR_BG23: Color = Color::new(
    "BG23",
    "Coral Sea",
    Family::BlueGreen,
    Group::S2,
    Value::B3,
    Rgb::new(205, 231, 224),
);

pub const COLOR_BG32: Color = Color::new(
    "BG32",
    "Aqua Mint",
    Family::BlueGreen,
    Group::S3,
    Value::B2,
    Rgb::new(204, 230, 219),
);

pub const COLOR_BG34: Color = Color::new(
    "BG34",
    "Horizon Green",
    Family::BlueGreen,
    Group::S3,
    Value::B4,
    Rgb::new(184, 222, 219),
);

pub const COLOR_BG45: Color = Color::new(
    "BG45",
    "Nile Blue",
    Family::BlueGreen,
    Group::S4,
    Value::B5,
    Rgb::new(193, 226, 227),
);

pub const COLOR_BG49: Color = Color::new(
    "BG49",
    "Duck Blue",
    Family::BlueGreen,
    Group::S4,
    Value::B9,
    Rgb::new(21, 178, 188),
);

pub const COLOR_BG53: Color = Color::new(
    "BG53",
    "Ice Mint",
    Family::BlueGreen,
    Group::S5,
    Value::B3,
    Rgb::new(92, 182, 189),
);

pub const COLOR_BG57: Color = Color::new(
    "BG57",
    "Jasper",
    Family::BlueGreen,
    Group::S5,
    Value::B7,
    Rgb::new(4, 162, 173),
);

pub const COLOR_BG70: Color = Color::new(
    "BG70",
    "Ocean Mist",
    Family::BlueGreen,
    Group::S7,
    Value::B0,
    Rgb::new(229, 238, 227),
);

pub const COLOR_BG72: Color = Color::new(
    "BG72",
    "Ice Ocean",
    Family::BlueGreen,
    Group::S7,
    Value::B2,
    Rgb::new(136, 190, 193),
);

pub const COLOR_BG75: Color = Color::new(
    "BG75",
    "Abyss Green",
    Family::BlueGreen,
    Group::S7,
    Value::B5,
    Rgb::new(181, 221, 214),
);

pub const COLOR_BG78: Color = Color::new(
    "BG78",
    "Bronze",
    Family::BlueGreen,
    Group::S7,
    Value::B8,
    Rgb::new(72, 117, 107),
);

pub const COLOR_BG90: Color = Color::new(
    "BG90",
    "Gray Sky",
    Family::BlueGreen,
    Group::S9,
    Value::B0,
    Rgb::new(209, 204, 184),
);

pub const COLOR_BG93: Color = Color::new(
    "BG93",
    "Green Gray",
    Family::BlueGreen,
    Group::S9,
    Value::B3,
    Rgb::new(203, 206, 196),
);

pub const COLOR_BG96: Color = Color::new(
    "BG96",
    "Bush",
    Family::BlueGreen,
    Group::S9,
    Value::B6,
    Rgb::new(154, 176, 158),
);

pub const COLOR_BG99: Color = Color::new(
    "BG99",
    "Flagstone Blue",
    Family::BlueGreen,
    Group::S9,
    Value::B9,
    Rgb::new(137, 169, 150),
);

pub const COLOR_B0000: Color = Color::new(
    "B0000",
    "Pale Celestine",
    Family::Blue,
    Group::S0,
    Value::B000,
    Rgb::new(246, 251, 254),
);

pub const COLOR_B000: Color = Color::new(
    "B000",
    "Pale Porcelain Blue",
    Family::Blue,
    Group::S0,
    Value::B00,
    Rgb::new(237, 246, 246),
);

pub const COLOR_B00: Color = Color::new(
    "B00",
    "Frost Blue",
    Family::Blue,
    Group::S0,
    Value::B0,
    Rgb::new(234, 246, 249),
);

pub const COLOR_B01: Color = Color::new(
    "B01",
    "Mint Blue",
    Family::Blue,
    Group::S0,
    Value::B1,
    Rgb::new(225, 241, 243),
);

pub const COLOR_B02: Color = Color::new(
    "B02",
    "Robin's Egg Blue",
    Family::Blue,
    Group::S0,
    Value::B2,
    Rgb::new(197, 230, 240),
);

pub const COLOR_B04: Color = Color::new(
    "B04",
    "Tahitian Blue",
    Family::Blue,
    Group::S0,
    Value::B4,
    Rgb::new(141, 209, 231),
);

pub const COLOR_B05: Color = Color::new(
    "B05",
    "Process Blue",
    Family::Blue,
    Group::S0,
    Value::B5,
    Rgb::new(100, 197, 229),
);

pub const COLOR_B06: Color = Color::new(
    "B06",
    "Peacock Blue",
    Family::Blue,
    Group::S0,
    Value::B6,
    Rgb::new(0, 172, 226),
);

pub const COLOR_B12: Color = Color::new(
    "B12",
    "Ice Blue",
    Family::Blue,
    Group::S1,
    Value::B2,
    Rgb::new(214, 234, 240),
);

pub const COLOR_B14: Color = Color::new(
    "B14",
    "Light Blue",
    Family::Blue,
    Group::S1,
    Value::B4,
    Rgb::new(141, 209, 235),
);

pub const COLOR_B16: Color = Color::new(
    "B16",
    "Chanine Blue",
    Family::Blue,
    Group::S1,
    Value::B6,
    Rgb::new(1, 186, 231),
);

pub const COLOR_B18: Color = Color::new(
    "B18",
    "Lapis Lazuli",
    Family::Blue,
    Group::S1,
    Value::B8,
    Rgb::new(41, 145, 201),
);

pub const COLOR_B21: Color = Color::new(
    "B21",
    "Baby Blue",
    Family::Blue,
    Group::S2,
    Value::B1,
    Rgb::new(230, 241, 250),
);

pub const COLOR_B23: Color = Color::new(
    "B23",
    "Phthalo Blue",
    Family::Blue,
    Group::S2,
    Value::B3,
    Rgb::new(171, 203, 233),
);

pub const COLOR_B24: Color = Color::new(
    "B24",
    "Sky",
    Family::Blue,
    Group::S2,
    Value::B4,
    Rgb::new(163, 213, 241),
);

pub const COLOR_B26: Color = Color::new(
    "B26",
    "Cobalt Blue",
    Family::Blue,
    Group::S2,
    Value::B6,
    Rgb::new(127, 187, 227),
);

pub const COLOR_B28: Color = Color::new(
    "B28",
    "Royal Blue",
    Family::Blue,
    Group::S2,
    Value::B8,
    Rgb::new(21, 113, 176),
);

pub const COLOR_B29: Color = Color::new(
    "B29",
    "Ultramarine",
    Family::Blue,
    Group::S2,
    Value::B9,
    Rgb::new(0, 119, 186),
);

pub const COLOR_B32: Color = Color::new(
    "B32",
    "Pale Blue",
    Family::Blue,
    Group::S3,
    Value::B2,
    Rgb::new(234, 243, 247),
);

pub const COLOR_B34: Color = Color::new(
    "B34",
    "Manganese Blue",
    Family::Blue,
    Group::S3,
    Value::B4,
    Rgb::new(155, 203, 235),
);

pub const COLOR_B37: Color = Color::new(
    "B37",
    "Antwerp Blue",
    Family::Blue,
    Group::S3,
    Value::B7,
    Rgb::new(4, 114, 163),
);

pub const COLOR_B39: Color = Color::new(
    "B39",
    "Prussian Blue",
    Family::Blue,
    Group::S3,
    Value::B9,
    Rgb::new(40, 106, 167),
);

pub const COLOR_B41: Color = Color::new(
    "B41",
    "Powder Blue",
    Family::Blue,
    Group::S4,
    Value::B1,
    Rgb::new(234, 243, 251),
);

pub const COLOR_B45: Color = Color::new(
    "B45",
    "Smoky Blue",
    Family::Blue,
    Group::S4,
    Value::B5,
    Rgb::new(143, 199, 234),
);

pub const COLOR_B52: Color = Color::new(
    "B52",
    "Soft Greenish Blue",
    Family::Blue,
    Group::S5,
    Value::B2,
    Rgb::new(193, 214, 225),
);

pub const COLOR_B60: Color = Color::new(
    "B60",
    "Pale Blue Gray",
    Family::Blue,
    Group::S6,
    Value::B0,
    Rgb::new(228, 232, 244),
);

pub const COLOR_B63: Color = Color::new(
    "B63",
    "Light Hydrangea",
    Family::Blue,
    Group::S6,
    Value::B3,
    Rgb::new(188, 198, 226),
);

pub const COLOR_B66: Color = Color::new(
    "B66",
    "Clematis",
    Family::Blue,
    Group::S6,
    Value::B3,
    Rgb::new(100, 115, 180),
);

pub const COLOR_B69: Color = Color::new(
    "B69",
    "Stratospheric Blue",
    Family::Blue,
    Group::S6,
    Value::B9,
    Rgb::new(25, 103, 167),
);

pub const COLOR_B79: Color = Color::new(
    "B79",
    "Iris",
    Family::Blue,
    Group::S7,
    Value::B9,
    Rgb::new(49, 65, 143),
);

pub const COLOR_B91: Color = Color::new(
    "B91",
    "Pale Grayish Blue",
    Family::Blue,
    Group::S9,
    Value::B1,
    Rgb::new(225, 233, 237),
);

pub const COLOR_B93: Color = Color::new(
    "B93",
    "Light Crockery Blue",
    Family::Blue,
    Group::S9,
    Value::B3,
    Rgb::new(171, 203, 223),
);

pub const COLOR_B95: Color = Color::new(
    "B95",
    "Light Grayish Cobalt",
    Family::Blue,
    Group::S9,
    Value::B5,
    Rgb::new(136, 176, 202),
);

pub const COLOR_B97: Color = Color::new(
    "B97",
    "Night Blue",
    Family::Blue,
    Group::S9,
    Value::B7,
    Rgb::new(69, 128, 157),
);

pub const COLOR_B99: Color = Color::new(
    "B99",
    "Agate",
    Family::Blue,
    Group::S9,
    Value::B9,
    Rgb::new(0, 77, 122),
);

pub const COLOR_E0000: Color = Color::new(
    "E0000",
    "Floral White",
    Family::Earth,
    Group::S0,
    Value::B000,
    Rgb::new(255, 250, 243),
);

pub const COLOR_E000: Color = Color::new(
    "E000",
    "Pale Fruit Pink",
    Family::Earth,
    Group::S0,
    Value::B00,
    Rgb::new(255, 248, 241),
);

pub const COLOR_E00: Color = Color::new(
    "E00",
    "Cotton Pearl",
    Family::Earth,
    Group::S0,
    Value::B0,
    Rgb::new(255, 246, 238),
);

pub const COLOR_E01: Color = Color::new(
    "E01",
    "Pink Flamingo",
    Family::Earth,
    Group::S0,
    Value::B1,
    Rgb::new(255, 242, 233),
);

pub const COLOR_E02: Color = Color::new(
    "E02",
    "Fruit Pink",
    Family::Earth,
    Group::S0,
    Value::B2,
    Rgb::new(255, 241, 230),
);

pub const COLOR_E04: Color = Color::new(
    "E04",
    "Lipstick Rose",
    Family::Earth,
    Group::S0,
    Value::B4,
    Rgb::new(236, 202, 206),
);

pub const COLOR_E07: Color = Color::new(
    "E07",
    "Light Mahogany",
    Family::Earth,
    Group::S0,
    Value::B7,
    Rgb::new(217, 146, 120),
);

pub const COLOR_E08: Color = Color::new(
    "E08",
    "Brown",
    Family::Earth,
    Group::S0,
    Value::B8,
    Rgb::new(213, 116, 92),
);

pub const COLOR_E09: Color = Color::new(
    "E09",
    "Burnt Sienna",
    Family::Earth,
    Group::S0,
    Value::B9,
    Rgb::new(226, 116, 83),
);

pub const COLOR_E11: Color = Color::new(
    "E11",
    "Barley Beige",
    Family::Earth,
    Group::S1,
    Value::B1,
    Rgb::new(255, 239, 222),
);

pub const COLOR_E13: Color = Color::new(
    "E13",
    "Desert Sand",
    Family::Earth,
    Group::S1,
    Value::B3,
    Rgb::new(239, 210, 187),
);

pub const COLOR_E15: Color = Color::new(
    "E15",
    "Earthenware",
    Family::Earth,
    Group::S1,
    Value::B5,
    Rgb::new(253, 199, 151),
);

pub const COLOR_E17: Color = Color::new(
    "E17",
    "Reddish Brass",
    Family::Earth,
    Group::S1,
    Value::B7,
    Rgb::new(196, 102, 85),
);

pub const COLOR_E18: Color = Color::new(
    "E18",
    "Copper",
    Family::Earth,
    Group::S1,
    Value::B8,
    Rgb::new(151, 86, 74),
);

pub const COLOR_E19: Color = Color::new(
    "E19",
    "Redwood",
    Family::Earth,
    Group::S1,
    Value::B9,
    Rgb::new(206, 84, 37),
);

pub const COLOR_E21: Color = Color::new(
    "E21",
    "Soft Sun",
    Family::Earth,
    Group::S2,
    Value::B1,
    Rgb::new(255, 233, 210),
);

pub const COLOR_E23: Color = Color::new(
    "E23",
    "Hazelnut",
    Family::Earth,
    Group::S2,
    Value::B3,
    Rgb::new(170, 106, 75),
);

pub const COLOR_E25: Color = Color::new(
    "E25",
    "Caribe Cocoa",
    Family::Earth,
    Group::S2,
    Value::B5,
    Rgb::new(223, 182, 147),
);

pub const COLOR_E27: Color = Color::new(
    "E27",
    "Milk Chocolate",
    Family::Earth,
    Group::S2,
    Value::B7,
    Rgb::new(172, 134, 109),
);

pub const COLOR_E29: Color = Color::new(
    "E29",
    "Burnt Umber",
    Family::Earth,
    Group::S2,
    Value::B9,
    Rgb::new(144, 63, 10),
);

pub const COLOR_E30: Color = Color::new(
    "E30",
    "Bisque",
    Family::Earth,
    Group::S3,
    Value::B0,
    Rgb::new(247, 239, 207),
);

pub const COLOR_E31: Color = Color::new(
    "E31",
    "Brick Beige",
    Family::Earth,
    Group::S3,
    Value::B1,
    Rgb::new(246, 236, 215),
);

pub const COLOR_E33: Color = Color::new(
    "E33",
    "Sand",
    Family::Earth,
    Group::S3,
    Value::B3,
    Rgb::new(246, 220, 189),
);

pub const COLOR_E34: Color = Color::new(
    "E34",
    "Toast",
    Family::Earth,
    Group::S3,
    Value::B4,
    Rgb::new(245, 215, 179),
);

pub const COLOR_E35: Color = Color::new(
    "E35",
    "Chamois",
    Family::Earth,
    Group::S3,
    Value::B5,
    Rgb::new(238, 210, 178),
);

pub const COLOR_E37: Color = Color::new(
    "E37",
    "Sepia",
    Family::Earth,
    Group::S3,
    Value::B7,
    Rgb::new(217, 165, 102),
);

pub const COLOR_E39: Color = Color::new(
    "E39",
    "Leather",
    Family::Earth,
    Group::S3,
    Value::B9,
    Rgb::new(210, 125, 51),
);

pub const COLOR_E40: Color = Color::new(
    "E40",
    "Brick White",
    Family::Earth,
    Group::S4,
    Value::B0,
    Rgb::new(247, 240, 229),
);

pub const COLOR_E41: Color = Color::new(
    "E41",
    "Pearl White",
    Family::Earth,
    Group::S4,
    Value::B1,
    Rgb::new(255, 244, 232),
);

pub const COLOR_E42: Color = Color::new(
    "E42",
    "Sand White",
    Family::Earth,
    Group::S4,
    Value::B2,
    Rgb::new(242, 232, 211),
);

pub const COLOR_E43: Color = Color::new(
    "E43",
    "Dull Ivory",
    Family::Earth,
    Group::S4,
    Value::B3,
    Rgb::new(240, 230, 203),
);

pub const COLOR_E44: Color = Color::new(
    "E44",
    "Clay",
    Family::Earth,
    Group::S4,
    Value::B4,
    Rgb::new(212, 201, 182),
);

pub const COLOR_E47: Color = Color::new(
    "E47",
    "Dark Brown",
    Family::Earth,
    Group::S4,
    Value::B7,
    Rgb::new(154, 130, 108),
);

pub const COLOR_E49: Color = Color::new(
    "E49",
    "Dark Bark",
    Family::Earth,
    Group::S4,
    Value::B9,
    Rgb::new(122, 93, 69),
);

pub const COLOR_E50: Color = Color::new(
    "E50",
    "Egg Shell",
    Family::Earth,
    Group::S5,
    Value::B0,
    Rgb::new(247, 240, 241),
);

pub const COLOR_E51: Color = Color::new(
    "E51",
    "Milky White",
    Family::Earth,
    Group::S5,
    Value::B1,
    Rgb::new(255, 241, 222),
);

pub const COLOR_E53: Color = Color::new(
    "E53",
    "Raw Silk",
    Family::Earth,
    Group::S5,
    Value::B3,
    Rgb::new(246, 236, 204),
);

pub const COLOR_E55: Color = Color::new(
    "E55",
    "Light Camel",
    Family::Earth,
    Group::S5,
    Value::B5,
    Rgb::new(245, 230, 196),
);

pub const COLOR_E57: Color = Color::new(
    "E57",
    "Light Walnut",
    Family::Earth,
    Group::S5,
    Value::B7,
    Rgb::new(194, 154, 106),
);

pub const COLOR_E59: Color = Color::new(
    "E59",
    "Walnut",
    Family::Earth,
    Group::S5,
    Value::B9,
    Rgb::new(173, 144, 118),
);

pub const COLOR_E70: Color = Color::new(
    "E70",
    "Ash Rose",
    Family::Earth,
    Group::S7,
    Value::B0,
    Rgb::new(241, 233, 226),
);

pub const COLOR_E71: Color = Color::new(
    "E71",
    "Champagne",
    Family::Earth,
    Group::S7,
    Value::B1,
    Rgb::new(235, 225, 218),
);

pub const COLOR_E74: Color = Color::new(
    "E74",
    "Cocoa Brown",
    Family::Earth,
    Group::S7,
    Value::B4,
    Rgb::new(181, 153, 140),
);

pub const COLOR_E77: Color = Color::new(
    "E77",
    "Maroon",
    Family::Earth,
    Group::S7,
    Value::B7,
    Rgb::new(148, 113, 87),
);

pub const COLOR_E79: Color = Color::new(
    "E79",
    "Cashew",
    Family::Earth,
    Group::S7,
    Value::B9,
    Rgb::new(82, 56, 45),
);

pub const COLOR_E81: Color = Color::new(
    "E81",
    "Ivory",
    Family::Earth,
    Group::S8,
    Value::B1,
    Rgb::new(220, 205, 152),
);

pub const COLOR_E84: Color = Color::new(
    "E84",
    "Khaki",
    Family::Earth,
    Group::S8,
    Value::B4,
    Rgb::new(180, 149, 95),
);

pub const COLOR_E87: Color = Color::new(
    "E87",
    "Fig",
    Family::Earth,
    Group::S8,
    Value::B7,
    Rgb::new(91, 77, 56),
);

pub const COLOR_E89: Color = Color::new(
    "E89",
    "Pecan",
    Family::Earth,
    Group::S8,
    Value::B9,
    Rgb::new(123, 116, 106),
);

pub const COLOR_E93: Color = Color::new(
    "E93",
    "Tea Rose",
    Family::Earth,
    Group::S9,
    Value::B3,
    Rgb::new(254, 219, 194),
);

pub const COLOR_E95: Color = Color::new(
    "E95",
    "Tea Orange",
    Family::Earth,
    Group::S9,
    Value::B5,
    Rgb::new(254, 199, 136),
);

pub const COLOR_E97: Color = Color::new(
    "E97",
    "Deep Orange",
    Family::Earth,
    Group::S9,
    Value::B7,
    Rgb::new(243, 169, 98),
);

pub const COLOR_E99: Color = Color::new(
    "E99",
    "Baked Clay",
    Family::Earth,
    Group::S9,
    Value::B9,
    Rgb::new(192, 101, 8),
);

pub const COLOR_C_00: Color = Color::new(
    "C-00",
    "Cool Gray No.00",
    Family::CoolGray,
    Group::Undefined,
    Value::B0,
    Rgb::new(241, 244, 246),
);

pub const COLOR_C_0: Color = Color::new(
    "C-0",
    "Cool Gray No.0",
    Family::CoolGray,
    Group::Undefined,
    Value::B00,
    Rgb::new(237, 242, 244),
);

pub const COLOR_C_1: Color = Color::new(
    "C-1",
    "Cool Gray No.1",
    Family::CoolGray,
    Group::Undefined,
    Value::B0,
    Rgb::new(228, 234, 237),
);

pub const COLOR_C_2: Color = Color::new(
    "C-2",
    "Cool Gray No.2",
    Family::CoolGray,
    Group::Undefined,
    Value::B1,
    Rgb::new(217, 224, 228),
);

pub const COLOR_C_3: Color = Color::new(
    "C-3",
    "Cool Gray No.3",
    Family::CoolGray,
    Group::Undefined,
    Value::B2,
    Rgb::new(208, 213, 218),
);

pub const COLOR_C_4: Color = Color::new(
    "C-4",
    "Cool Gray No.4",
    Family::CoolGray,
    Group::Undefined,
    Value::B3,
    Rgb::new(185, 193, 199),
);

pub const COLOR_C_5: Color = Color::new(
    "C-5",
    "Cool Gray No.5",
    Family::CoolGray,
    Group::Undefined,
    Value::B4,
    Rgb::new(164, 174, 178),
);

pub const COLOR_C_6: Color = Color::new(
    "C-6",
    "Cool Gray No.6",
    Family::CoolGray,
    Group::Undefined,
    Value::B5,
    Rgb::new(147, 155, 161),
);

pub const COLOR_C_7: Color = Color::new(
    "C-7",
    "Cool Gray No.7",
    Family::CoolGray,
    Group::Undefined,
    Value::B6,
    Rgb::new(119, 127, 131),
);

pub const COLOR_C_8: Color = Color::new(
    "C-8",
    "Cool Gray No.8",
    Family::CoolGray,
    Group::Undefined,
    Value::B7,
    Rgb::new(99, 105, 109),
);

pub const COLOR_C_9: Color = Color::new(
    "C-9",
    "Cool Gray No.9",
    Family::CoolGray,
    Group::Undefined,
    Value::B8,
    Rgb::new(83, 81, 80),
);

pub const COLOR_C_10: Color = Color::new(
    "C-10",
    "Cool Gray No.10",
    Family::CoolGray,
    Group::Undefined,
    Value::B9,
    Rgb::new(31, 30, 30),
);

pub const COLOR_N_0: Color = Color::new(
    "N-0",
    "Neutral Gray No.0",
    Family::NeutralGray,
    Group::Undefined,
    Value::B00,
    Rgb::new(242, 242, 242),
);

pub const COLOR_N_1: Color = Color::new(
    "N-1",
    "Neutral Gray No.1",
    Family::NeutralGray,
    Group::Undefined,
    Value::B0,
    Rgb::new(235, 235, 235),
);

pub const COLOR_N_2: Color = Color::new(
    "N-2",
    "Neutral Gray No.2",
    Family::NeutralGray,
    Group::Undefined,
    Value::B1,
    Rgb::new(227, 227, 227),
);

pub const COLOR_N_3: Color = Color::new(
    "N-3",
    "Neutral Gray No.3",
    Family::NeutralGray,
    Group::Undefined,
    Value::B2,
    Rgb::new(220, 220, 220),
);

pub const COLOR_N_4: Color = Color::new(
    "N-4",
    "Neutral Gray No.4",
    Family::NeutralGray,
    Group::Undefined,
    Value::B3,
    Rgb::new(201, 201, 201),
);

pub const COLOR_N_5: Color = Color::new(
    "N-5",
    "Neutral Gray No.5",
    Family::NeutralGray,
    Group::Undefined,
    Value::B4,
    Rgb::new(183, 182, 182),
);

pub const COLOR_N_6: Color = Color::new(
    "N-6",
    "Neutral Gray No.6",
    Family::NeutralGray,
    Group::Undefined,
    Value::B5,
    Rgb::new(164, 164, 164),
);

pub const COLOR_N_7: Color = Color::new(
    "N-7",
    "Neutral Gray No.7",
    Family::NeutralGray,
    Group::Undefined,
    Value::B6,
    Rgb::new(133, 132, 133),
);

pub const COLOR_N_8: Color = Color::new(
    "N-8",
    "Neutral Gray No.8",
    Family::NeutralGray,
    Group::Undefined,
    Value::B7,
    Rgb::new(111, 110, 110),
);

pub const COLOR_N_9: Color = Color::new(
    "N-9",
    "Neutral Gray No.9",
    Family::NeutralGray,
    Group::Undefined,
    Value::B8,
    Rgb::new(83, 81, 80),
);

pub const COLOR_N_10: Color = Color::new(
    "N-10",
    "Neutral Gray No.10",
    Family::NeutralGray,
    Group::Undefined,
    Value::B9,
    Rgb::new(31, 30, 30),
);

pub const COLOR_T_0: Color = Color::new(
    "T-0",
    "Toner Gray No.0",
    Family::TonerGray,
    Group::Undefined,
    Value::B00,
    Rgb::new(242, 242, 242),
);

pub const COLOR_T_1: Color = Color::new(
    "T-1",
    "Toner Gray No.1",
    Family::TonerGray,
    Group::Undefined,
    Value::B0,
    Rgb::new(240, 240, 238),
);

pub const COLOR_T_2: Color = Color::new(
    "T-2",
    "Toner Gray No.2",
    Family::TonerGray,
    Group::Undefined,
    Value::B1,
    Rgb::new(233, 233, 230),
);

pub const COLOR_T_3: Color = Color::new(
    "T-3",
    "Toner Gray No.3",
    Family::TonerGray,
    Group::Undefined,
    Value::B2,
    Rgb::new(221, 220, 215),
);

pub const COLOR_T_4: Color = Color::new(
    "T-4",
    "Toner Gray No.4",
    Family::TonerGray,
    Group::Undefined,
    Value::B3,
    Rgb::new(202, 200, 196),
);

pub const COLOR_T_5: Color = Color::new(
    "T-5",
    "Toner Gray No.5",
    Family::TonerGray,
    Group::Undefined,
    Value::B4,
    Rgb::new(184, 181, 176),
);

pub const COLOR_T_6: Color = Color::new(
    "T-6",
    "Toner Gray No.6",
    Family::TonerGray,
    Group::Undefined,
    Value::B5,
    Rgb::new(164, 162, 158),
);

pub const COLOR_T_7: Color = Color::new(
    "T-7",
    "Toner Gray No.7",
    Family::TonerGray,
    Group::Undefined,
    Value::B6,
    Rgb::new(133, 131, 128),
);

pub const COLOR_T_8: Color = Color::new(
    "T-8",
    "Toner Gray No.8",
    Family::TonerGray,
    Group::Undefined,
    Value::B7,
    Rgb::new(111, 109, 106),
);

pub const COLOR_T_9: Color = Color::new(
    "T-9",
    "Toner Gray No.9",
    Family::TonerGray,
    Group::Undefined,
    Value::B8,
    Rgb::new(83, 80, 77),
);

pub const COLOR_T_10: Color = Color::new(
    "T-10",
    "Toner Gray No.10",
    Family::TonerGray,
    Group::Undefined,
    Value::B9,
    Rgb::new(31, 30, 30),
);

pub const COLOR_W_00: Color = Color::new(
    "W-00",
    "Warm Gray No.00",
    Family::WarmGray,
    Group::Undefined,
    Value::B00,
    Rgb::new(247, 247, 241),
);

pub const COLOR_W_0: Color = Color::new(
    "W-0",
    "Warm Gray No.0",
    Family::WarmGray,
    Group::Undefined,
    Value::B00,
    Rgb::new(247, 246, 240),
);

pub const COLOR_W_1: Color = Color::new(
    "W-1",
    "Warm Gray No.1",
    Family::WarmGray,
    Group::Undefined,
    Value::B0,
    Rgb::new(238, 237, 231),
);

pub const COLOR_W_2: Color = Color::new(
    "W-2",
    "Warm Gray No.2",
    Family::WarmGray,
    Group::Undefined,
    Value::B1,
    Rgb::new(238, 237, 233),
);

pub const COLOR_W_3: Color = Color::new(
    "W-3",
    "Warm Gray No.3",
    Family::WarmGray,
    Group::Undefined,
    Value::B2,
    Rgb::new(221, 220, 213),
);

pub const COLOR_W_4: Color = Color::new(
    "W-4",
    "Warm Gray No.4",
    Family::WarmGray,
    Group::Undefined,
    Value::B3,
    Rgb::new(202, 201, 194),
);

pub const COLOR_W_5: Color = Color::new(
    "W-5",
    "Warm Gray No.5",
    Family::WarmGray,
    Group::Undefined,
    Value::B4,
    Rgb::new(184, 182, 176),
);

pub const COLOR_W_6: Color = Color::new(
    "W-6",
    "Warm Gray No.6",
    Family::WarmGray,
    Group::Undefined,
    Value::B5,
    Rgb::new(164, 163, 156),
);

pub const COLOR_W_7: Color = Color::new(
    "W-7",
    "Warm Gray No.7",
    Family::WarmGray,
    Group::Undefined,
    Value::B6,
    Rgb::new(134, 132, 127),
);

pub const COLOR_W_8: Color = Color::new(
    "W-8",
    "Warm Gray No.8",
    Family::WarmGray,
    Group::Undefined,
    Value::B7,
    Rgb::new(111, 110, 105),
);

pub const COLOR_W_9: Color = Color::new(
    "W-9",
    "Warm Gray No.9",
    Family::WarmGray,
    Group::Undefined,
    Value::B8,
    Rgb::new(83, 80, 76),
);

pub const COLOR_W_10: Color = Color::new(
    "W-10",
    "Warm Gray No.10",
    Family::WarmGray,
    Group::Undefined,
    Value::B9,
    Rgb::new(31, 30, 29),
);

pub const COLOR_0: Color = Color::new(
    "0",
    "Colorless Blender",
    Family::Achromatic,
    Group::Undefined,
    Value::Undefined,
    Rgb::new(255, 255, 255),
);

pub const COLOR_100: Color = Color::new(
    "100",
    "Black",
    Family::Achromatic,
    Group::S0,
    Value::B0,
    Rgb::new(1, 1, 1),
);

pub const COLOR_110: Color = Color::new(
    "110",
    "Special Black",
    Family::Achromatic,
    Group::S1,
    Value::B0,
    Rgb::new(1, 1, 1),
);

pub const COLOR_FV: Color = Color::new(
    "FV",
    "Fluorescent Violet",
    Family::Flourescent,
    Group::Undefined,
    Value::Undefined,
    Rgb::new(132, 112, 178),
);

pub const COLOR_FRV: Color = Color::new(
    "FRV",
    "Fluorescent Pink",
    Family::Flourescent,
    Group::Undefined,
    Value::Undefined,
    Rgb::new(249, 176, 203),
);

pub const COLOR_FYR: Color = Color::new(
    "FYR",
    "Fluorescent Orange",
    Family::Flourescent,
    Group::Undefined,
    Value::Undefined,
    Rgb::new(255, 215, 164),
);

pub const COLOR_FY: Color = Color::new(
    "FY",
    "Fluorescent Yellow",
    Family::Flourescent,
    Group::Undefined,
    Value::Undefined,
    Rgb::new(255, 249, 160),
);

pub const COLOR_FYG: Color = Color::new(
    "FYG",
    "Fluorescent Yellow Green",
    Family::Flourescent,
    Group::Undefined,
    Value::Undefined,
    Rgb::new(220, 228, 170),
);

pub const COLOR_FG: Color = Color::new(
    "FG",
    "Fluorescent Green",
    Family::Flourescent,
    Group::Undefined,
    Value::Undefined,
    Rgb::new(175, 206, 21),
);

pub const COLOR_FBG: Color = Color::new(
    "FBG",
    "Fluorescent Blue Green",
    Family::Flourescent,
    Group::Undefined,
    Value::Undefined,
    Rgb::new(128, 205, 231),
);

pub const COLOR_FB: Color = Color::new(
    "FB",
    "Fluorescent Blue",
    Family::Flourescent,
    Group::Undefined,
    Value::Undefined,
    Rgb::new(11, 147, 205),
);

/// The list of all Copic colors.
pub const ALL_COLORS: [Color; 358] = [
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
    COLOR_YG67,
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
    COLOR_G29,
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
    COLOR_C_00,
    COLOR_C_0,
    COLOR_C_1,
    COLOR_C_2,
    COLOR_C_3,
    COLOR_C_4,
    COLOR_C_5,
    COLOR_C_6,
    COLOR_C_7,
    COLOR_C_8,
    COLOR_C_9,
    COLOR_C_10,
    COLOR_N_0,
    COLOR_N_1,
    COLOR_N_2,
    COLOR_N_3,
    COLOR_N_4,
    COLOR_N_5,
    COLOR_N_6,
    COLOR_N_7,
    COLOR_N_8,
    COLOR_N_9,
    COLOR_N_10,
    COLOR_T_0,
    COLOR_T_1,
    COLOR_T_2,
    COLOR_T_3,
    COLOR_T_4,
    COLOR_T_5,
    COLOR_T_6,
    COLOR_T_7,
    COLOR_T_8,
    COLOR_T_9,
    COLOR_T_10,
    COLOR_W_00,
    COLOR_W_0,
    COLOR_W_1,
    COLOR_W_2,
    COLOR_W_3,
    COLOR_W_4,
    COLOR_W_5,
    COLOR_W_6,
    COLOR_W_7,
    COLOR_W_8,
    COLOR_W_9,
    COLOR_W_10,
    COLOR_0,
    COLOR_100,
    COLOR_110,
    COLOR_FV,
    COLOR_FRV,
    COLOR_FYR,
    COLOR_FY,
    COLOR_FYG,
    COLOR_FG,
    COLOR_FBG,
    COLOR_FB,
];
