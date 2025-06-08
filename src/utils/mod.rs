pub mod area_calc {}

pub mod utils {
    use uuid::Uuid;

    pub fn gen_id() -> Uuid {
        uuid::Uuid::new_v4()
    }
}

pub mod shapes {
    pub struct Rectangle {
        pub width: f32,
        pub height: f32,
    }

    impl Rectangle {
        pub fn new(value: (f32, f32)) -> Self {
            Self {
                width: value.0,
                height: value.1,
            }
        }
    }

    pub struct Triangle {
        pub width: f32,
        pub height: f32,
    }

    impl Triangle {
        pub fn new(value: (f32, f32)) -> Self {
            Self {
                width: value.0,
                height: value.1,
            }
        }
    }

    pub struct CustomShape {
        pub points: Vec<(f32, f32)>,
    }

    impl CustomShape {
        pub fn new(points: Vec<(f32, f32)>) -> Self {
            Self { points }
        }
    }
}
