use ordered_float::NotNan;

pub struct Point(NotNan<f32>, NotNan<f32>);

const ZERO: Point = unsafe {
    let zero = NotNan::new_unchecked(0.0);
    Point(zero, zero)
};

const INF: Point = unsafe {
    let inf = NotNan::new_unchecked(f32::INFINITY);
    Point(inf, inf)
};

impl Point {
    pub const ZERO: Point = ZERO;
    pub const INFINITY: Point = INF;

    pub fn new(x: f32, y: f32) -> Point {
        Point(NotNan::new(x).unwrap(), NotNan::new(y).unwrap())
    }

    pub fn x(self) -> f32 {
        *self.0
    }

    pub fn y(self) -> f32 {
        *self.1
    }
}
