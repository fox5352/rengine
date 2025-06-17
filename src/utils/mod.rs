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
