pub mod area_calc {}

pub mod util_items {
    use uuid::Uuid;

    pub fn gen_id() -> Uuid {
        uuid::Uuid::new_v4()
    }
}

pub mod shapes {
    #[derive(Debug, Clone)]
    pub enum CustomShapeVariant {
        Rectangle,
        Triangle,
        Circle,
        Other(String),
    }

    /// A custom shape defined by a sequence of (x, y) coordinates.
    ///
    /// Coordinates are in normalized space where (0.0, 0.0) is the bottom-left
    /// and (1.0, 1.0) is the top-right of the shape's bounding box.
    #[derive(Debug, Clone)]
    pub struct CustomShape {
        /// The list of points that make up the shape, in drawing order.
        pub points: Vec<(f32, f32)>,
        pub variant: CustomShapeVariant,
    }

    impl Default for CustomShape {
        fn default() -> Self {
            Self {
                points: vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0), (0.0, 0.0)],
                variant: CustomShapeVariant::Rectangle,
            }
        }
    }

    impl CustomShape {
        /// Creates a new `CustomShape` from a given list of (x, y) points.
        ///
        /// # Arguments
        ///
        /// * `points` - A vector of points, where each point is a tuple (x, y) with values between 0.0 and 1.0.
        pub fn new(points: Vec<(f32, f32)>, variant: CustomShapeVariant) -> Self {
            Self { points, variant }
        }

        /// Adds a single point to the shape.
        ///
        /// # Arguments
        ///
        /// * `point` - A tuple (x, y) to be appended to the shape's point list.
        pub fn add_point(&mut self, point: (f32, f32)) {
            self.points.push(point);
        }

        /// Replaces the current points with a new list of points.
        ///
        /// # Arguments
        ///
        /// * `points` - A new vector of (x, y) points.
        pub fn override_points(&mut self, points: Vec<(f32, f32)>) {
            self.points = points;
        }

        /// Generates a rectangle shape with corners at:
        /// (0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), and closing back to (0.0, 0.0).
        ///
        /// The shape is closed by repeating the starting point at the end.
        pub fn gen_rectangle() -> Self {
            Self {
                variant: CustomShapeVariant::Rectangle,
                points: vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.0, 0.0)],
            }
        }

        /// Generates a triangle shape with points at:
        /// (0.0, 0.0), (0.5, 1.0), (1.0, 0.0), and closing back to (0.0, 0.0).
        ///
        /// The shape is closed by repeating the starting point at the end.
        pub fn gen_triangle() -> Self {
            Self {
                points: vec![(0.0, 0.0), (0.5, 1.0), (1.0, 0.0), (0.0, 0.0)],
                variant: CustomShapeVariant::Triangle,
            }
        }
    }
}

#[cfg(test)]
mod test_shapes {
    use super::shapes::CustomShape;

    #[test]
    fn test_gen_rectangle() {
        let shape = CustomShape::gen_rectangle();
        assert_eq!(
            shape.points,
            vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.0, 0.0)]
        );
    }

    #[test]
    fn test_gen_triangle() {
        let shape = CustomShape::gen_triangle();
        assert_eq!(
            shape.points,
            vec![(0.0, 0.0), (0.5, 1.0), (1.0, 0.0), (0.0, 0.0)]
        );
    }

    #[test]
    fn test_add_point() {
        let mut shape = CustomShape::gen_rectangle();
        shape.add_point((2.0, 2.0));
        assert_eq!(
            shape.points,
            vec![
                (0.0, 0.0),
                (1.0, 0.0),
                (1.0, 1.0),
                (0.0, 1.0),
                (0.0, 0.0),
                (2.0, 2.0)
            ]
        );
    }

    #[test]
    fn test_override_points() {
        let mut shape = CustomShape::gen_rectangle();
        shape.override_points(vec![(2.0, 2.0)]);
        assert_eq!(shape.points, vec![(2.0, 2.0)]);
    }
}

// mod for cal colitions
pub mod collision_cal {
    use crate::units::{PointWithDeg, Size};

    use super::shapes::CustomShape;

    pub fn transform_shape(
        point: &PointWithDeg,
        size: &Size,
        shape: &CustomShape,
    ) -> Vec<(f32, f32)> {
        let angle = point.deg.to_radians();
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        shape
            .points
            .iter()
            .map(|(px, py)| {
                // Flip the Y-axis at the normalized input level
                let cx = px - 0.5;
                let cy = 0.5 - py; // <-- Flip Y here

                // Scale to object size
                let sx = cx * size.x;
                let sy = cy * size.y;

                // Rotate
                let rx = sx * cos_theta - sy * sin_theta;
                let ry = sx * sin_theta + sy * cos_theta;

                // Move to world position
                let world_x = point.x as f32 + rx;
                let world_y = point.y as f32 + ry;

                (world_x, world_y)
            })
            .collect()
    }

    /// Checks if two objects collide using Axis-Aligned Bounding Box (AABB) collision detection.
    ///
    /// This method assumes that both objects are represented as rectangles aligned to the axes,
    /// meaning rotation (`deg` field) is ignored.
    ///
    /// # Arguments
    ///
    /// * `obj1` - A tuple of (position, size, shape) for the first object.
    /// * `obj2` - A tuple of (position, size, shape) for the second object.
    ///
    /// # Returns
    ///
    /// * `true` if the objects overlap.
    /// * `false` otherwise.
    #[allow(unused_variables)]
    pub fn check_collision(
        obj1: (PointWithDeg, Size, CustomShape),
        obj2: (PointWithDeg, Size, CustomShape),
    ) -> bool {
        let obj1_pos = obj1.0;
        let obj1_size = obj1.1;

        let obj2_pos = obj2.0;
        let obj2_size = obj2.1;

        let obj1_x = obj1_pos.x as f32;
        let obj1_y = obj1_pos.y as f32;
        let obj2_x = obj2_pos.x as f32;
        let obj2_y = obj2_pos.y as f32;

        obj1_x < obj2_x + obj2_size.x
            && obj1_x + obj1_size.x > obj2_x
            && obj1_y < obj2_y + obj2_size.y
            && obj1_y + obj1_size.y > obj2_y
    }
}

#[cfg(test)]
mod test_collision_cal {
    use crate::{
        units::{PointWithDeg, Size},
        utils::{collision_cal::check_collision, shapes::CustomShape},
    };
    //
    /*     use super::collision_cal::transform_shape; */

    #[test]
    fn test_aabb_collision() {
        let obj1 = (
            PointWithDeg {
                x: 0,
                y: 0,
                deg: 0.0,
            },
            Size { x: 10.0, y: 10.0 },
            CustomShape::gen_rectangle(),
        );

        let obj2 = (
            PointWithDeg {
                x: 5,
                y: 5,
                deg: 0.0,
            },
            Size { x: 10.0, y: 10.0 },
            CustomShape::gen_rectangle(),
        );

        assert!(check_collision(obj1, obj2));
    }

    #[test]
    fn test_aabb_no_collision() {
        let obj1 = (
            PointWithDeg {
                x: 0,
                y: 0,
                deg: 0.0,
            },
            Size { x: 10.0, y: 10.0 },
            CustomShape::gen_rectangle(),
        );

        let obj2 = (
            PointWithDeg {
                x: 20,
                y: 20,
                deg: 0.0,
            },
            Size { x: 10.0, y: 10.0 },
            CustomShape::gen_rectangle(),
        );

        assert!(!check_collision(obj1, obj2));
    }

    // #[test]
    // fn test_transform_shape_no_rotation() {
    //     let shape = CustomShape {
    //         points: vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
    //     };
    //     let point = Point {
    //         x: 10,
    //         y: 20,
    //         deg: 0.0,
    //     };
    //     let size = Size { x: 2.0, y: 4.0 };
    //
    //     let result = transform_shape(&point, &size, &shape);
    //
    //     let expected = [(9.0, 18.0), (11.0, 18.0), (11.0, 22.0), (9.0, 22.0)];
    //
    //     for (i, (rx, ry)) in result.iter().enumerate() {
    //         let (ex, ey) = expected[i];
    //         assert!(
    //             (rx - ex).abs() < 0.001,
    //             "x mismatch at point {}: {} vs {}",
    //             i,
    //             rx,
    //             ex
    //         );
    //         assert!(
    //             (ry - ey).abs() < 0.001,
    //             "y mismatch at point {}: {} vs {}",
    //             i,
    //             ry,
    //             ey
    //         );
    //     }
    // }
    //
    // #[test]
    // fn test_transform_shape_90_degrees() {
    //     let shape = CustomShape {
    //         points: vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
    //     };
    //     let point = Point {
    //         x: 0,
    //         y: 0,
    //         deg: 90.0,
    //     };
    //     let size = Size { x: 2.0, y: 2.0 };
    //
    //     let result = transform_shape(&point, &size, &shape);
    //
    //     let expected = [(1.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0)];
    //
    //     for (i, (rx, ry)) in result.iter().enumerate() {
    //         let (ex, ey) = expected[i];
    //         assert!(
    //             (rx - ex).abs() < 0.001,
    //             "x mismatch at point {}: {} vs {}",
    //             i,
    //             rx,
    //             ex
    //         );
    //         assert!(
    //             (ry - ey).abs() < 0.001,
    //             "y mismatch at point {}: {} vs {}",
    //             i,
    //             ry,
    //             ey
    //         );
    //     }
    // }
}
