use crate::math::fsize::FSize;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Size(pub FSize, pub FSize);

impl Size {
    pub const ZERO: Size = Size(FSize::ZERO, FSize::ZERO);
    pub const INFINITY: Size = Size(FSize::INFINITY, FSize::INFINITY);

    pub fn square(size: f32) -> Size {
        let size = FSize::new(size);
        Size(size, size)
    }

    pub fn new(width: f32, height: f32) -> Size {
        Size(FSize::new(width), FSize::new(height))
    }

    pub fn area(self) -> f32 {
        *(self.0 * self.1)
    }

    pub fn aspect_ratio_xy(self) -> f32 {
        *(self.0 / self.1)
    }

    pub fn aspect_ratio_yx(self) -> f32 {
        *(self.1 / self.0)
    }

    pub fn max(self, other: Size) -> Size {
        Size(self.0.max(other.0), self.1.max(other.1))
    }

    pub fn min(self, other: Size) -> Size {
        Size(self.0.min(other.0), self.1.min(other.1))
    }

    pub fn with_width(self, width: f32) -> Size {
        let width = FSize::new(width);
        Size(width, self.1)
    }

    pub fn with_height(self, height: f32) -> Size {
        let height = FSize::new(height);
        Size(self.0, height)
    }

    pub fn width(self) -> f32 {
        *self.0
    }

    pub fn height(self) -> f32 {
        *self.1
    }
}

