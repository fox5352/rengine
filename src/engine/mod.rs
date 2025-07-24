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

    pub trait CollisionTrait {
        fn check_collision(&self, new_point: PointWithDeg) -> bool;
        fn move_object(&mut self, delta_time: f32) -> bool;
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
        + CollisionTrait
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
            + ShapeTrait
            + CollisionTrait,
    > PhysicsObjectTrait for T
    {
    }
}

pub mod structures {
    use uuid::Uuid;

    use crate::{
        state::engine_state::{
            get_animated_identifiable, get_animated_object, get_mask_row, get_static_identifiable,
            get_static_object,
        },
        units::{PointWithDeg, Size, Velocity},
        utils::{collision_cal::check_collision, shapes::CustomShape, util_items::gen_id},
    };

    use super::traits::{
        CollisionTrait, IdentifiableTrait, MasksTrait, NamedTrait, PointTrait, ShapeTrait,
        SizeTrait, VelocityTrait, ZIndexTrait,
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
        ///
        /// # Arguments
        /// * `z_index` - The z-index of the object.
        /// * `name` - The name of the object.
        /// * `pos` - The position of the object.
        /// * `size` - The size of the object.
        /// * `masks` - The masks of the object.
        /// * `shape` - The shape of the object.
        ///
        /// # Returns
        /// A new StaticObject.
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
        /// Create a new AnimatedObject
        ///
        /// # Arguments
        /// * `z_index` - The z-index of the object.
        /// * `name` - The name of the object.
        /// * `pos` - The position of the object.
        /// * `size` - The size of the object.
        /// * `velocity` - The velocity of the object.
        /// * `masks` - The masks of the object.
        /// * `shape` - The shape of the object.
        ///
        /// # Returns
        /// A new AnimatedObject.
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

    impl CollisionTrait for AnimatedObject {
        /// Check if the object collides with another object
        ///
        /// # Arguments
        /// * `new_point` - The new position of the object.
        ///
        /// # Returns
        /// true if the object collides with another object, false otherwise.
        fn check_collision(&self, new_point: PointWithDeg) -> bool {
            let this_obj_id = self.get_id().to_string();
            let _virtual_obj = (new_point, self.size, self.get_shape());

            for row in 1..15 {
                let row_of_mask =
                    get_mask_row(row).expect("failed to get mask row at collision check");

                for global_object_id in row_of_mask.iter() {
                    // Skip self
                    if *global_object_id == this_obj_id {
                        continue;
                    }

                    if get_static_identifiable()
                        .unwrap()
                        .contains(global_object_id)
                    {
                        let g_obj = get_static_object(global_object_id).unwrap();
                        let g_obj = g_obj.lock().unwrap();

                        if check_collision(
                            _virtual_obj.clone(),
                            (g_obj.get_pos(), g_obj.get_size(), g_obj.get_shape()),
                        ) {
                            return true; // collision found
                        }
                    } else if get_animated_identifiable()
                        .unwrap()
                        .contains(global_object_id)
                    {
                        let g_obj = get_animated_object(global_object_id).unwrap();
                        let g_obj = g_obj.lock().unwrap();

                        if check_collision(
                            _virtual_obj.clone(),
                            (g_obj.get_pos(), g_obj.get_size(), g_obj.get_shape()),
                        ) {
                            return true; // collision found
                        }
                    }
                }
            }
            false
        }

        fn move_object(&mut self, delta_time: f32) -> bool {
            let vel = self.velocity.scale(delta_time); // Apply delta_time to velocity
            let mut factor = 1.0;

            while factor >= 0.1 {
                let scaled_vel = vel.scale(factor);
                let new_pos = PointWithDeg {
                    x: self.pos.x + scaled_vel.x,
                    y: self.pos.y + scaled_vel.y,
                    deg: self.pos.deg,
                };

                if !self.check_collision(new_pos) {
                    self.pos = new_pos;
                    self.velocity = self.velocity.scale(factor); // scale original velocity
                    return false;
                }

                factor -= 0.1;
            }

            // No valid position found; object remains where it is
            true
        }
    }
}
