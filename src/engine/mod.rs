pub mod traits {
    pub use crate::units::{Point, Size};
    use uuid::Uuid;

    pub trait Identifiable {
        fn get_id(&self) -> Uuid;
    }

    pub trait Named {
        fn get_name(&self) -> String;
    }

    /// The `engine` module defines game objects and their traits.
    pub trait Object {
        fn set_size(self, size: Size);
        fn set_pos(self, pos: Point);
    }

    pub trait PhysicsObject {
        fn update(&mut self, delta_time: f32);
        fn process(&mut self, delta_time: f32);
    }
}

pub mod structures {
    use uuid::Uuid;

    use crate::{
        units::{Point, Size, Velocity},
        utils::utils::gen_id,
    };

    use super::traits::{Identifiable, Named};

    /// A static object with position and size
    // :StaticObject
    pub struct StaticObject {
        pub id: Uuid,
        pub name: String,
        pub pos: Point,
        pub size: Size,
    }

    impl StaticObject {
        /// Create a new StaticObject
        pub fn new(name: String, pos: Point, size: Size) -> Self {
            let id = gen_id();
            Self {
                id,
                name,
                pos,
                size,
            }
        }
    }

    impl Identifiable for StaticObject {
        fn get_id(&self) -> Uuid {
            self.id
        }
    }

    impl Named for StaticObject {
        fn get_name(&self) -> String {
            self.name.clone()
        }
    }

    /// Placeholder for animated objects (not implemented)
    #[derive(Default)]
    pub struct AnimatedObject {
        pub id: Uuid,
        pub name: String,
        pub pos: Point,
        pub size: Size,
        pub velocity: Velocity,
    }

    impl AnimatedObject {
        pub fn new(name: String, pos: Point, size: Size, velocity: Velocity) -> Self {
            let id = gen_id();
            Self {
                id,
                name,
                pos,
                size,
                velocity,
            }
        }
    }
}
