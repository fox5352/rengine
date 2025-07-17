/// A point in 2D space with an optional direction in degrees.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointWithDeg {
    pub x: f32,
    pub y: f32,
    pub deg: f32,
}

impl PointWithDeg {
    /// Creates a new `PointWithDeg` from x and y coordinates, and an optional degree.
    ///
    /// - `x`: f32 - X coordinate
    /// - `y`: f32 - Y coordinate
    /// - `deg`: Option<f32> - Direction in degrees (defaults to 0.0 if None)
    ///
    /// Returns: `PointWithDeg`
    pub fn new(x: f32, y: f32, deg: Option<f32>) -> Self {
        Self {
            x,
            y,
            deg: deg.unwrap_or(0.0),
        }
    }
}

/// A 2D size representation with width (`x`) and height (`y`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub x: f32,
    pub y: f32,
}

impl Size {
    /// Creates a new `Size` from width and height values.
    ///
    /// - `x`: f32 - Width
    /// - `y`: f32 - Height
    ///
    /// Returns: `Size`
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Represents velocity with x and y components.
/// Includes builder-style and mutating methods for scaling.
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    /// Creates a new zero-initialized `Velocity`.
    ///
    /// Returns: `Velocity`
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `Velocity` with the given x and y components.
    ///
    /// - `x`: f32 - Horizontal velocity
    /// - `y`: f32 - Vertical velocity
    ///
    /// Returns: `Velocity`
    pub fn from(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Sets the x component (builder-style).
    ///
    /// - `x`: f32 - New x value
    ///
    /// Returns: updated `Velocity`
    pub fn set_x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }

    /// Sets the y component (builder-style).
    ///
    /// - `y`: f32 - New y value
    ///
    /// Returns: updated `Velocity`
    pub fn set_y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }

    /// Scales both x and y components by a factor (builder-style).
    ///
    /// - `factor`: f32 - Multiplier to apply
    ///
    /// Returns: updated `Velocity`
    pub fn scale(mut self, factor: f32) -> Self {
        self.x *= factor;
        self.y *= factor;
        self
    }

    /// Scales only the x component by a factor (builder-style).
    ///
    /// - `factor`: f32 - Multiplier to apply to x
    ///
    /// Returns: updated `Velocity`
    pub fn scale_x(mut self, factor: f32) -> Self {
        self.x *= factor;
        self
    }

    /// Scales only the y component by a factor (builder-style).
    ///
    /// - `factor`: f32 - Multiplier to apply to y
    ///
    /// Returns: updated `Velocity`
    pub fn scale_y(mut self, factor: f32) -> Self {
        self.y *= factor;
        self
    }

    /// Scales both components in-place.
    ///
    /// - `factor`: f32 - Multiplier to apply
    pub fn scale_mut(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
    }

    /// Scales the x component in-place.
    ///
    /// - `factor`: f32 - Multiplier to apply to x
    pub fn scale_x_mut(&mut self, factor: f32) {
        self.x *= factor;
    }

    /// Scales the y component in-place.
    ///
    /// - `factor`: f32 - Multiplier to apply to y
    pub fn scale_y_mut(&mut self, factor: f32) {
        self.y *= factor;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_with_deg() {
        let p = PointWithDeg::new(10.0, 20.0, Some(90.0));
        assert_eq!(
            p,
            PointWithDeg {
                x: 10.0,
                y: 20.0,
                deg: 90.0
            }
        );

        let default_deg = PointWithDeg::new(1.0, 2.0, None);
        assert_eq!(default_deg.deg, 0.0);
    }

    #[test]
    fn test_size() {
        let s = Size::new(50.0, 75.0);
        assert_eq!(s, Size { x: 50.0, y: 75.0 });
    }

    #[test]
    fn test_velocity_builder_style() {
        let v = Velocity::from(10.0, 5.0)
            .scale(0.5)
            .scale_x(2.0)
            .scale_y(0.5);
        assert_eq!(v, Velocity { x: 10.0, y: 1.25 });
    }

    #[test]
    fn test_velocity_mutating_style() {
        let mut v = Velocity::from(10.0, 5.0);
        v.scale_mut(0.5);
        assert_eq!(v, Velocity { x: 5.0, y: 2.5 });

        v.scale_x_mut(2.0);
        assert_eq!(v.x, 10.0);

        v.scale_y_mut(0.5);
        assert_eq!(v.y, 1.25);
    }

    #[test]
    fn test_velocity_setters() {
        let v = Velocity::new().set_x(3.0).set_y(4.0);
        assert_eq!(v, Velocity { x: 3.0, y: 4.0 });
    }
}

