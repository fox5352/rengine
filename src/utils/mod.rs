pub mod area_calc {}

pub mod util_items {
    use uuid::Uuid;

    pub fn gen_id() -> Uuid {
        uuid::Uuid::new_v4()
    }
}

pub mod shapes {
    #[derive(Debug, Clone, Default)]
    pub struct CustomShape {
        pub points: Vec<(f32, f32)>,
    }

    impl CustomShape {
        pub fn new(points: Vec<(f32, f32)>) -> Self {
            Self { points }
        }

        pub fn add_point(&mut self, point: (f32, f32)) {
            self.points.push(point);
        }

        pub fn override_points(&mut self, points: Vec<(f32, f32)>) {
            self.points = points;
        }

        pub fn gen_rectangle() -> Self {
            Self {
                points: vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0), (0.0, 0.0)],
            }
        }

        pub fn gen_triangle() -> Self {
            Self {
                points: vec![(0.0, 0.0), (0.5, 1.0), (1.0, 0.0), (0.0, 0.0)],
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
            vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0), (0.0, 0.0)]
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
                (0.0, 1.0),
                (1.0, 1.0),
                (1.0, 0.0),
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
    use crate::units::{Point, Size};

    use super::shapes::CustomShape;

    pub fn transform_shape(point: &Point, size: &Size, shape: &CustomShape) -> Vec<(f32, f32)> {
        let angle = point.deg.to_radians();
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        shape
            .points
            .iter()
            .map(|(px, py)| {
                // Step 1: Offset the shape so it's centered around (0,0)
                let cx = px - 0.5; // px is from 0.0 to 1.0
                let cy = py - 0.5;

                // Step 2: Scale it to object size
                let sx = cx * size.x;
                let sy = cy * size.y;

                // Step 3: Rotate it around the center
                let rx = sx * cos_theta - sy * sin_theta;
                let ry = sx * sin_theta + sy * cos_theta;

                // Step 4: Move it to its position in the world
                let world_x = point.x as f32 + rx;
                let world_y = point.y as f32 + ry;

                (world_x, world_y)
            })
            .collect()
    }

    #[allow(unused_variables)]
    pub fn check_collision(
        obj1: (Point, Size, CustomShape),
        obj2: (Point, Size, CustomShape),
    ) -> bool {
        let obj1_world_points = transform_shape(&obj1.0, &obj1.1, &obj1.2);
        let obj2_world_points = transform_shape(&obj2.0, &obj2.1, &obj2.2);

        false
    }
}

#[cfg(test)]
mod test_collision_cal {
    use crate::units::{Point, Size};

    use super::collision_cal::transform_shape;
    use super::shapes::CustomShape;

    #[test]
    fn test_transform_shape_no_rotation() {
        let shape = CustomShape {
            points: vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        };
        let point = Point {
            x: 10,
            y: 20,
            deg: 0.0,
        };
        let size = Size { x: 2.0, y: 4.0 };

        let result = transform_shape(&point, &size, &shape);

        let expected = [(9.0, 18.0), (11.0, 18.0), (11.0, 22.0), (9.0, 22.0)];

        for (i, (rx, ry)) in result.iter().enumerate() {
            let (ex, ey) = expected[i];
            assert!(
                (rx - ex).abs() < 0.001,
                "x mismatch at point {}: {} vs {}",
                i,
                rx,
                ex
            );
            assert!(
                (ry - ey).abs() < 0.001,
                "y mismatch at point {}: {} vs {}",
                i,
                ry,
                ey
            );
        }
    }

    #[test]
    fn test_transform_shape_90_degrees() {
        let shape = CustomShape {
            points: vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        };
        let point = Point {
            x: 0,
            y: 0,
            deg: 90.0,
        };
        let size = Size { x: 2.0, y: 2.0 };

        let result = transform_shape(&point, &size, &shape);

        let expected = [(1.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0)];

        for (i, (rx, ry)) in result.iter().enumerate() {
            let (ex, ey) = expected[i];
            assert!(
                (rx - ex).abs() < 0.001,
                "x mismatch at point {}: {} vs {}",
                i,
                rx,
                ex
            );
            assert!(
                (ry - ey).abs() < 0.001,
                "y mismatch at point {}: {} vs {}",
                i,
                ry,
                ey
            );
        }
    }
}
