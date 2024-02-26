#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Color(pub u32);

impl Color {
    const ALPHA: u32 = 0xFF000000;

    pub const BLACK: Color = Color(0xFF000000);
    pub const WHITE: Color = Color(0xFFFFFFFF);
    pub const RED: Color = Color(0xFFFF0000);
    pub const GREEN: Color = Color(0xFF00FF00);
    pub const BLUE: Color = Color(0xFF0000FF);
    pub const YELLOW: Color = Color(0xFFFFFF00);
    pub const CYAN: Color = Color(0xFF00FFFF);
    pub const MAGENTA: Color = Color(0xFFFF00FF);
    pub const TRANSPARENT: Color = Color(0x00000000);
    pub const LIGHT_GREY: Color = Color(0xFFD3D3D3);
    pub const GREY: Color = Color(0xFF808080);
    pub const DARK_GREY: Color = Color(0xFFA9A9A9);
    pub const LIGHT_BLUE: Color = Color(0xFFADD8E6);
    pub const LIGHT_GREEN: Color = Color(0xFF90EE90);
    pub const LIGHT_RED: Color = Color(0xFFFFA07A);
    pub const LIGHT_YELLOW: Color = Color(0xFFFFFFE0);
    pub const LIGHT_CYAN: Color = Color(0xFFE0FFFF);
    pub const LIGHT_MAGENTA: Color = Color(0xFFFFC0CB);
    pub const DARK_BLUE: Color = Color(0xFF00008B);
    pub const DARK_GREEN: Color = Color(0xFF006400);
    pub const DARK_RED: Color = Color(0xFF8B0000);
    pub const DARK_YELLOW: Color = Color(0xFFB8860B);
    pub const DARK_CYAN: Color = Color(0xFF008B8B);
    pub const DARK_MAGENTA: Color = Color(0xFF8B008B);

    pub const FUCHSIA: Color = Color(0xFFFF00FF);
    pub const LIME: Color = Color(0xFF00FF00);
    pub const OLIVE: Color = Color(0xFF808000);
    pub const PURPLE: Color = Color(0xFF800080);
    pub const TEAL: Color = Color(0xFF008080);
    pub const AQUA: Color = Color(0xFF00FFFF);
    pub const MAROON: Color = Color(0xFF800000);
    pub const NAVY: Color = Color(0xFF000080);
    pub const SILVER: Color = Color(0xFFC0C0C0);

    pub const AVOCADO: Color = Color(0xFF568203);

    pub const fn grey(value: u8) -> Color {
        Color((0xFF << 24) | ((value as u32) << 16) | ((value as u32) << 8) | (value as u32))
    }

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color((0xFF << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color(((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32))
    }

    pub const fn red(self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    pub const fn green(self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    pub const fn blue(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    pub const fn alpha(self) -> u8 {
        ((self.0 >> 24) & 0xFF) as u8
    }

    pub const fn with_red(self, red: u8) -> Color {
        Color((self.0 & 0xFF00FFFF) | ((red as u32) << 16))
    }

    pub const fn with_green(self, green: u8) -> Color {
        Color((self.0 & 0xFFFF00FF) | ((green as u32) << 8))
    }

    pub const fn with_blue(self, blue: u8) -> Color {
        Color((self.0 & 0xFFFFFF00) | (blue as u32))
    }

    pub const fn with_alpha(self, alpha: u8) -> Color {
        Color((self.0 & 0x00FFFFFF) | ((alpha as u32) << 24))
    }

    pub const fn invert(self) -> Color {
        Color((0xFFFFFFFF ^ self.0) | Self::ALPHA)
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::grey(0)
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> u32 {
        color.0
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Color {
        Color(value)
    }
}

impl From<(u8, )> for Color {
    fn from((value, ): (u8, )) -> Color {
        Color::grey(value)
    }
}

impl From<(u8, u8)> for Color {
    fn from((c, a): (u8, u8)) -> Color {
        Color::from_rgba(c, c, c, a)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Color {
        Color::from_rgb(r, g, b)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Color {
        Color::from_rgba(r, g, b, a)
    }
}

impl From<Color> for (u8, u8, u8, u8) {
    fn from(value: Color) -> Self {
        (value.red(), value.green(), value.blue(), value.alpha())
    }
}
