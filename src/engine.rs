/// The `engine` module defines game objects and their traits.
pub trait Object {
    fn set_size(self, size: Size);
    fn set_pos(self, pos: Point);
}

pub trait PhysicsObject {
    fn update(self, new_pos: Point);
}

/// A point with x, y and an optional degree for direction
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub deg: u32,
}

impl Point {
    /// Create a new Point; defaults deg to 0 if None provided
    pub fn new(x: usize, y: usize, deg: Option<u32>) -> Self {
        Self {
            x,
            y,
            deg: deg.unwrap_or(0),
        }
    }
}

/// Size struct for width and height
#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub x: usize,
    pub y: usize,
}

impl Size {
    /// Create a new Size
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// A static object with position and size
pub struct StaticObject {
    pub pos: Point,
    pub size: Size,
}

impl StaticObject {
    /// Create a new StaticObject
    pub fn new(pos: Point, size: Size) -> Self {
        Self { pos, size }
    }
}

/// Placeholder for animated objects (not implemented)
pub struct AnimatedObject {
    pub pos: Point,
    pub size: Size,
}
impl AnimatedObject {
    pub fn new(pos: Point, size: Size) -> Self {
        return Self { pos, size };
    }
}
