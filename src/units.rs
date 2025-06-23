/// A point with x, y and an optional degree for direction
#[derive(Clone, Copy, Debug, Default)]
pub struct PointWithDeg {
    pub x: f32,
    pub y: f32,
    pub deg: f32,
}

impl PointWithDeg {
    /// Create a new Point; defaults deg to 0 if None provided
    pub fn new(x: f32, y: f32, deg: Option<f32>) -> Self {
        Self {
            x,
            y,
            deg: deg.unwrap_or(0.0),
        }
    }
}

/// Size struct for width and height
#[derive(Clone, Copy, Debug, Default)]
pub struct Size {
    pub x: f32,
    pub y: f32,
}

impl Size {
    /// Create a new Size
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn set_x(mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(mut self, y: f32) {
        self.y = y;
    }
}
