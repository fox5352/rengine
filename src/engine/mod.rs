pub mod traits {
    use crate::units::{Point, Size};
    use uuid::Uuid;

    pub trait Identifiable {
        fn get_id(&self) -> Uuid;
    }

    pub trait Named {
        fn get_name(&self) -> String;
    }

    pub trait Masks {
        fn get_masks(&self) -> Vec<usize>;
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

    /// full trait
    pub trait StaticObjectTrait: Object + Identifiable + Named {}
    impl<T: Object + Identifiable + Named> StaticObjectTrait for T {}

    pub trait PhysicsObjectTrait: Object + PhysicsObject + Identifiable + Named {}
    impl<T: Object + PhysicsObject + Identifiable + Named> PhysicsObjectTrait for T {}
}

pub mod structures {
    use uuid::Uuid;

    use crate::{
        units::{Point, Size, Velocity},
        utils::util_items::gen_id,
    };

    use super::traits::{Identifiable, Masks, Named};

    /// A static object with position and size
    // :StaticObject
    pub struct StaticObject {
        pub id: Uuid,
        pub name: String,
        pub pos: Point,
        pub size: Size,
        pub masks: Vec<usize>,
    }

    impl StaticObject {
        /// Create a new StaticObject
        pub fn new(name: String, pos: Point, size: Size, masks: Option<Vec<usize>>) -> Self {
            let id = gen_id();
            Self {
                id,
                name,
                pos,
                size,
                masks: masks.unwrap_or_default(),
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

    impl Masks for StaticObject {
        fn get_masks(&self) -> Vec<usize> {
            self.masks.clone()
        }
    }

    /// Placeholder for animated objects (not implemented)
    #[derive(Default)]
    pub struct AnimatedObject {
        pub id: Uuid,
        pub name: String,
        pub pos: Point,
        pub size: Size,
        pub masks: Vec<usize>,
        pub velocity: Velocity,
    }

    impl AnimatedObject {
        pub fn new(
            name: String,
            pos: Point,
            size: Size,
            velocity: Velocity,
            masks: Option<Vec<usize>>,
        ) -> Self {
            let id = gen_id();
            Self {
                id,
                name,
                pos,
                size,
                masks: masks.unwrap_or_default(),
                velocity,
            }
        }
    }

    impl Identifiable for AnimatedObject {
        fn get_id(&self) -> Uuid {
            self.id
        }
    }
    impl Named for AnimatedObject {
        fn get_name(&self) -> String {
            self.name.clone()
        }
    }

    impl Masks for AnimatedObject {
        fn get_masks(&self) -> Vec<usize> {
            self.masks.clone()
        }
    }
}
