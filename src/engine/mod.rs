pub mod traits {
    use crate::{
        units::{PointWithDeg, Size, Velocity},
        utils::shapes::CustomShape,
    };
    use uuid::Uuid;

    pub trait ZIndexTrait {
        fn get_z_index(&self) -> u8;
    }

    pub trait IdentifiableTrait {
        fn get_id(&self) -> Uuid;
    }

    pub trait NamedTrait {
        fn get_name(&self) -> String;
    }

    pub trait MasksTrait {
        fn get_masks(&self) -> Vec<usize>;
    }

    pub trait ShapeTrait {
        fn get_shape(&self) -> CustomShape;
    }

    pub trait VelocityTrait {
        fn get_velocity(&self) -> Velocity;
    }

    pub trait SizeTrait {
        fn get_size(&self) -> Size;
    }

    pub trait PointTrait {
        fn get_pos(&self) -> PointWithDeg;
    }

    /// The `engine` module defines game objects and their traits.
    pub trait Object {
        fn set_size(self, size: Size);
        fn set_pos(self, pos: PointWithDeg);
    }

    pub trait PhysicsObject {
        #[deprecated]
        fn update(&mut self, delta_time: f32);
        fn process(&mut self, delta_time: f32);
    }

    /// full trait
    pub trait StaticObjectTrait:
        Send
        + Sync
        + ZIndexTrait
        + Object
        + IdentifiableTrait
        + NamedTrait
        + MasksTrait
        + SizeTrait
        + PointTrait
        + ShapeTrait
    {
    }
    impl<
        T: Send
            + Sync
            + ZIndexTrait
            + Object
            + IdentifiableTrait
            + NamedTrait
            + MasksTrait
            + SizeTrait
            + PointTrait
            + ShapeTrait,
    > StaticObjectTrait for T
    {
    }

    pub trait PhysicsObjectTrait:
        Send
        + Sync
        + ZIndexTrait
        + Object
        + PhysicsObject
        + IdentifiableTrait
        + NamedTrait
        + MasksTrait
        + VelocityTrait
        + SizeTrait
        + PointTrait
        + ShapeTrait
    {
    }
    impl<
        T: Send
            + Sync
            + ZIndexTrait
            + Object
            + PhysicsObject
            + IdentifiableTrait
            + NamedTrait
            + MasksTrait
            + VelocityTrait
            + SizeTrait
            + PointTrait
            + ShapeTrait,
    > PhysicsObjectTrait for T
    {
    }
}

pub mod structures {
    use uuid::Uuid;

    use crate::{
        units::{PointWithDeg, Size, Velocity},
        utils::{shapes::CustomShape, util_items::gen_id},
    };

    use super::traits::{
        IdentifiableTrait, MasksTrait, NamedTrait, PointTrait, ShapeTrait, SizeTrait,
        VelocityTrait, ZIndexTrait,
    };

    /// A static object with position and size
    // :StaticObject
    pub struct StaticObject {
        pub z_index: u8,
        pub id: Uuid,
        pub name: String,
        pub pos: PointWithDeg,
        pub size: Size,
        pub masks: Vec<usize>,
        pub shape: CustomShape,
    }

    impl StaticObject {
        /// Create a new StaticObject
        pub fn new(
            z_index: u8,
            name: String,
            pos: PointWithDeg,
            size: Size,
            masks: Option<Vec<usize>>,
            shape: CustomShape,
        ) -> Self {
            let id = gen_id();
            Self {
                z_index,
                id,
                name,
                pos,
                size,
                masks: masks.unwrap_or_default(),
                shape,
            }
        }
    }

    impl ZIndexTrait for StaticObject {
        fn get_z_index(&self) -> u8 {
            self.z_index
        }
    }

    impl IdentifiableTrait for StaticObject {
        fn get_id(&self) -> Uuid {
            self.id
        }
    }

    impl NamedTrait for StaticObject {
        fn get_name(&self) -> String {
            self.name.clone()
        }
    }

    impl MasksTrait for StaticObject {
        fn get_masks(&self) -> Vec<usize> {
            self.masks.clone()
        }
    }

    impl SizeTrait for StaticObject {
        fn get_size(&self) -> Size {
            self.size
        }
    }

    impl PointTrait for StaticObject {
        fn get_pos(&self) -> PointWithDeg {
            self.pos
        }
    }

    impl ShapeTrait for StaticObject {
        fn get_shape(&self) -> CustomShape {
            self.shape.clone()
        }
    }

    /// Placeholder for animated objects (not implemented)
    #[derive(Default)]
    pub struct AnimatedObject {
        pub z_index: u8,
        pub id: Uuid,
        pub name: String,
        pub pos: PointWithDeg,
        pub size: Size,
        pub masks: Vec<usize>,
        pub velocity: Velocity,
        pub shape: CustomShape,
    }

    impl AnimatedObject {
        pub fn new(
            z_index: u8,
            name: String,
            pos: PointWithDeg,
            size: Size,
            velocity: Velocity,
            masks: Option<Vec<usize>>,
            shape: CustomShape,
        ) -> Self {
            let id = gen_id();
            Self {
                z_index,
                id,
                name,
                pos,
                size,
                masks: masks.unwrap_or_default(),
                velocity,
                shape,
            }
        }
    }

    impl ZIndexTrait for AnimatedObject {
        fn get_z_index(&self) -> u8 {
            self.z_index
        }
    }

    impl IdentifiableTrait for AnimatedObject {
        fn get_id(&self) -> Uuid {
            self.id
        }
    }
    impl NamedTrait for AnimatedObject {
        fn get_name(&self) -> String {
            self.name.clone()
        }
    }

    impl MasksTrait for AnimatedObject {
        fn get_masks(&self) -> Vec<usize> {
            self.masks.clone()
        }
    }

    impl VelocityTrait for AnimatedObject {
        fn get_velocity(&self) -> Velocity {
            self.velocity
        }
    }

    impl SizeTrait for AnimatedObject {
        fn get_size(&self) -> Size {
            self.size
        }
    }

    impl PointTrait for AnimatedObject {
        fn get_pos(&self) -> PointWithDeg {
            self.pos
        }
    }

    impl ShapeTrait for AnimatedObject {
        fn get_shape(&self) -> CustomShape {
            self.shape.clone()
        }
    }
}
