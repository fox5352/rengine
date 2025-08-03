//! # Game Engine Traits and Structures
//! 
//! This module provides the core traits and structures for a 2D game engine.
//! It includes object management, physics simulation, collision detection,
//! and scripting capabilities.
//! 
//! ## Key Components
//! 
//! - **Traits**: Define capabilities for game objects (position, velocity, collision, etc.)
//! - **Structures**: Concrete implementations for static and animated game objects
//! - **Physics**: Movement and collision detection systems
//! - **Scripting**: Sequence-based behavior system for animated objects

pub mod traits {
    //! Core traits that define the capabilities and behaviors of game objects.
    //! 
    //! This module contains all the trait definitions that game objects can implement
    //! to gain specific capabilities like positioning, collision detection, physics
    //! simulation, and scripting.

    use std::any::Any;
    use uuid::Uuid;

    use crate::{
        units::{PointWithDeg, Size, Velocity},
        utils::shapes::CustomShape,
    };

    /// Base trait that all game objects must implement.
    /// 
    /// Provides fundamental functionality including update cycles and type casting
    /// capabilities for dynamic object management.
    /// 
    /// # Requirements
    /// 
    /// All implementors must be thread-safe (`Send + Sync`) and support Any for downcasting.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// # use std::any::Any;
    /// # struct MyObject;
    /// impl BaseTrait for MyObject {
    ///     fn update(&mut self, delta_time: f32) {
    ///         // Update object state
    ///     }
    ///     
    ///     fn as_any(&self) -> &dyn Any { self }
    ///     fn as_any_mut(&mut self) -> &mut dyn Any { self }
    /// }
    /// ```
    pub trait BaseTrait: Any + Send + Sync {
        /// Updates the object's state based on elapsed time.
        /// 
        /// # Arguments
        /// 
        /// * `delta_time` - Time elapsed since the last update in seconds
        fn update(&mut self, delta_time: f32);
        
        /// Returns a reference to this object as `Any` for downcasting.
        fn as_any(&self) -> &dyn Any;
        
        /// Returns a mutable reference to this object as `Any` for downcasting.
        fn as_any_mut(&mut self) -> &mut dyn Any;
    }

    /// Trait for objects that have a z-index for rendering order.
    /// 
    /// Objects with higher z-index values are rendered on top of objects
    /// with lower z-index values.
    pub trait ZIndexTrait {
        /// Returns the z-index of this object.
        /// 
        /// # Returns
        /// 
        /// A `u8` value representing the rendering layer (0-255)
        fn get_z_index(&self) -> u8;
    }

    /// Trait for objects that can be uniquely identified.
    /// 
    /// Provides a UUID-based identification system for game objects,
    /// enabling efficient lookup and reference management.
    pub trait IdentifiableTrait {
        /// Returns the unique identifier for this object.
        /// 
        /// # Returns
        /// 
        /// A `Uuid` that uniquely identifies this object instance
        fn get_id(&self) -> Uuid;
    }

    /// Trait for objects that have a human-readable name.
    /// 
    /// Useful for debugging, save systems, and user interfaces.
    pub trait NamedTrait {
        /// Returns the name of this object.
        /// 
        /// # Returns
        /// 
        /// A `String` containing the object's name
        fn get_name(&self) -> String;
    }

    /// Trait for objects that participate in a masking system.
    /// 
    /// Masks are used for collision detection layers, allowing objects
    /// to selectively interact with other objects based on their mask values.
    pub trait MasksTrait {
        /// Returns the collision masks for this object.
        /// 
        /// # Returns
        /// 
        /// A `Vec<usize>` containing the mask indices this object belongs to
        fn get_masks(&self) -> Vec<usize>;
    }

    /// Trait for objects that have a geometric shape.
    /// 
    /// The shape is used for collision detection and rendering purposes.
    pub trait ShapeTrait {
        /// Returns the geometric shape of this object.
        /// 
        /// # Returns
        /// 
        /// A `CustomShape` defining the object's collision and visual boundaries
        fn get_shape(&self) -> CustomShape;
    }

    /// Trait for objects that have velocity (speed and direction).
    /// 
    /// Velocity is used in physics calculations to determine movement
    /// and collision responses.
    pub trait VelocityTrait {
        /// Returns the current velocity of this object.
        /// 
        /// # Returns
        /// 
        /// A `Velocity` struct containing x and y components of movement
        fn get_velocity(&self) -> Velocity;
    }

    /// Trait for objects that have a size dimension.
    /// 
    /// Size is used for collision detection, rendering, and spatial queries.
    pub trait SizeTrait {
        /// Returns the size of this object.
        /// 
        /// # Returns
        /// 
        /// A `Size` struct containing width and height dimensions
        fn get_size(&self) -> Size;
    }

    /// Trait for objects that have a position in 2D space.
    /// 
    /// Position includes x, y coordinates and rotation angle.
    pub trait PointTrait {
        /// Returns the current position of this object.
        /// 
        /// # Returns
        /// 
        /// A `PointWithDeg` struct containing x, y coordinates and rotation
        fn get_pos(&self) -> PointWithDeg;
    }

    /// Trait for objects that can detect and respond to collisions.
    /// 
    /// Provides collision detection and movement with collision response
    /// for physics-enabled objects.
    pub trait CollisionTrait {
        /// Checks if the object would collide at a new position.
        /// 
        /// This method performs collision detection against all relevant
        /// objects in the game world without actually moving the object.
        /// 
        /// # Arguments
        /// 
        /// * `new_point` - The position to test for collisions
        /// 
        /// # Returns
        /// 
        /// `true` if a collision would occur, `false` otherwise
        fn check_collision(&self, new_point: PointWithDeg) -> bool;
        
        /// Attempts to move the object with collision detection.
        /// 
        /// This method tries to move the object based on its velocity and
        /// the given delta time. If a collision is detected, it attempts
        /// progressively smaller movements to find a valid position.
        /// 
        /// # Arguments
        /// 
        /// * `delta_time` - Time elapsed since last frame in seconds
        /// 
        /// # Returns
        /// 
        /// `true` if movement was blocked by collision, `false` if successful
        fn move_object(&mut self, delta_time: f32) -> bool;
    }

    /// Base trait for simple game objects.
    /// 
    /// Defines the fundamental operations that can be performed on
    /// game objects like setting position and size.
    /// 
    /// # Note
    /// 
    /// This trait consumes `self` for setting operations, following
    /// a builder pattern approach.
    pub trait Object {
        /// Sets the size of this object.
        /// 
        /// # Arguments
        /// 
        /// * `size` - The new size for the object
        fn set_size(self, size: Size);
        
        /// Sets the position of this object.
        /// 
        /// # Arguments
        /// 
        /// * `pos` - The new position for the object
        fn set_pos(self, pos: PointWithDeg);
    }

    /// Trait for objects that participate in physics simulation.
    /// 
    /// Provides both legacy update methods and modern processing methods
    /// for handling physics calculations.
    pub trait PhysicsObject {
        /// Legacy update method for physics objects.
        /// 
        /// # Deprecated
        /// 
        /// Use `process()` instead for new implementations.
        /// 
        /// # Arguments
        /// 
        /// * `delta_time` - Time elapsed since last frame in seconds
        #[deprecated = "Use process() instead"]
        fn update(&mut self, delta_time: f32);
        
        /// Processes physics calculations for this object.
        /// 
        /// This method should handle movement, collision response,
        /// and other physics-related updates.
        /// 
        /// # Arguments
        /// 
        /// * `delta_time` - Time elapsed since last frame in seconds
        fn process(&mut self, delta_time: f32);
    }

    /// Combined trait for objects that can be used in scripted sequences.
    /// 
    /// This trait combines the necessary capabilities (velocity, position,
    /// collision) that scripted behaviors need to interact with objects.
    /// 
    /// # Requirements
    /// 
    /// Objects must be thread-safe and implement velocity, position,
    /// and collision traits to participate in scripted sequences.
    pub trait SequenceParamTraits: Send + Sync + VelocityTrait + PointTrait + CollisionTrait {}

    impl<T> SequenceParamTraits for T 
    where 
        T: Send + Sync + VelocityTrait + PointTrait + CollisionTrait 
    {}

    /// Type alias for script functions used in object sequences.
    /// 
    /// Script functions take a mutable reference to an object implementing
    /// `SequenceParamTraits` and return a boolean indicating whether the
    /// script step is complete.
    /// 
    /// # Returns
    /// 
    /// `true` if the script step is complete, `false` if it should continue
    pub type ScriptFn = Box<dyn FnMut(&mut dyn SequenceParamTraits) -> bool + Send + Sync + 'static>;

    /// Trait for objects that can execute scripted behavior sequences.
    /// 
    /// Sequences allow for complex, time-based behaviors to be scripted
    /// and executed on animated objects.
    pub trait SequenceTrait {
        /// Adds a script sequence to this object.
        /// 
        /// The script will be executed during the object's update cycle,
        /// allowing for complex scripted behaviors.
        /// 
        /// # Arguments
        /// 
        /// * `script` - A vector of script functions to execute in sequence
        fn add_script(&mut self, script: Vec<ScriptFn>);
        
        /// Executes the current script sequence.
        /// 
        /// This method should be called during the object's update cycle
        /// to advance scripted behaviors.
        fn run_sequence(&mut self);
    }

    /// Composite trait defining common functionality for all game objects.
    /// 
    /// This trait combines all the basic capabilities that every game object
    /// should have, including identification, positioning, sizing, and rendering.
    /// 
    /// # Implementing Types
    /// 
    /// Any type that implements all the constituent traits automatically
    /// implements this trait through the blanket implementation.
    pub trait CommonObjectTraits:
        BaseTrait
        + ZIndexTrait
        + Object
        + IdentifiableTrait
        + NamedTrait
        + MasksTrait
        + SizeTrait
        + PointTrait
        + ShapeTrait
    {}

    impl<T> CommonObjectTraits for T 
    where
        T: BaseTrait
            + ZIndexTrait
            + Object
            + IdentifiableTrait
            + NamedTrait
            + MasksTrait
            + SizeTrait
            + PointTrait
            + ShapeTrait
    {}

    /// Trait for static (non-moving) game objects.
    /// 
    /// Static objects have all the common object capabilities but do not
    /// participate in physics simulation or scripted behaviors.
    /// 
    /// # Thread Safety
    /// 
    /// Static objects must be thread-safe to allow for concurrent access
    /// during collision detection and rendering.
    pub trait StaticObjectTrait: CommonObjectTraits + Send + Sync {}

    impl<T> StaticObjectTrait for T 
    where 
        T: CommonObjectTraits + Send + Sync 
    {}

    /// Composite trait for objects with full physics and scripting capabilities.
    /// 
    /// This trait represents the most complex type of game object, combining
    /// all common object features with physics simulation, collision detection,
    /// and scripted behavior capabilities.
    /// 
    /// # Use Cases
    /// 
    /// - Player characters
    /// - NPCs with complex behaviors
    /// - Interactive environment objects
    /// - Projectiles with scripted trajectories
    pub trait PhysicsObjectTrait:
        CommonObjectTraits
        + PhysicsObject
        + VelocityTrait
        + CollisionTrait
        + SequenceTrait
        + Send
        + Sync
    {}

    impl<T> PhysicsObjectTrait for T 
    where
        T: CommonObjectTraits
            + PhysicsObject
            + VelocityTrait
            + CollisionTrait
            + SequenceTrait
            + Send
            + Sync
    {}
}

pub mod structures {
    //! Concrete implementations of game objects.
    //! 
    //! This module provides the actual struct definitions for different types
    //! of game objects, along with their trait implementations.

    use std::any::Any;
    use uuid::Uuid;

    use crate::{
        state::engine_state::{
            get_animated_identifiable, get_animated_object, get_mask_row, 
            get_static_identifiable, get_static_object,
        },
        units::{PointWithDeg, Size, Velocity},
        utils::{
            collision_cal::check_collision, 
            shapes::CustomShape, 
            util_items::gen_id
        },
    };

    use super::traits::{
        BaseTrait, CollisionTrait, IdentifiableTrait, MasksTrait, NamedTrait, 
        PhysicsObject, PhysicsObjectTrait, PointTrait, ScriptFn, SequenceParamTraits, 
        SequenceTrait, ShapeTrait, SizeTrait, VelocityTrait, ZIndexTrait
    };

    /// A static game object that doesn't move or change over time.
    /// 
    /// Static objects are used for environment elements like walls, platforms,
    /// decorative elements, and other non-interactive scenery. They participate
    /// in collision detection but do not have velocity or physics simulation.
    /// 
    /// # Fields
    /// 
    /// * `z_index` - Rendering layer (higher values render on top)
    /// * `id` - Unique identifier for this object instance
    /// * `name` - Human-readable name for debugging and identification
    /// * `pos` - Position in 2D space with rotation
    /// * `size` - Width and height dimensions
    /// * `masks` - Collision detection layer masks
    /// * `shape` - Geometric shape for collision and rendering
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// # use uuid::Uuid;
    /// # use crate::{PointWithDeg, Size, CustomShape};
    /// let wall = StaticObject::new(
    ///     1,                              // z_index
    ///     "Wall".to_string(),            // name
    ///     PointWithDeg::new(100.0, 50.0, 0.0), // position
    ///     Size::new(200.0, 20.0),       // size
    ///     Some(vec![1, 2]),              // collision masks
    ///     CustomShape::Rectangle,         // shape
    /// );
    /// ```
    pub struct StaticObject {
        /// The rendering layer of this object (0-255, higher renders on top)
        pub z_index: u8,
        /// Unique identifier for this object instance
        pub id: Uuid,
        /// Human-readable name for debugging and identification
        pub name: String,
        /// Position in 2D space with rotation angle
        pub pos: PointWithDeg,
        /// Width and height dimensions of the object
        pub size: Size,
        /// Collision detection layer masks this object belongs to
        pub masks: Vec<usize>,
        /// Geometric shape used for collision detection and rendering
        pub shape: CustomShape,
    }

    impl StaticObject {
        /// Creates a new static object with the specified properties.
        ///
        /// # Arguments
        /// 
        /// * `z_index` - The rendering layer (0-255, higher values on top)
        /// * `name` - Human-readable name for the object
        /// * `pos` - Initial position with rotation
        /// * `size` - Width and height of the object
        /// * `masks` - Optional collision masks (defaults to empty if None)
        /// * `shape` - Geometric shape for collision and rendering
        ///
        /// # Returns
        /// 
        /// A new `StaticObject` instance with a generated unique ID
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// # use crate::{StaticObject, PointWithDeg, Size, CustomShape};
        /// let platform = StaticObject::new(
        ///     0,                                    // background layer
        ///     "Platform".to_string(),
        ///     PointWithDeg::new(0.0, 100.0, 0.0), // bottom of screen
        ///     Size::new(800.0, 20.0),             // wide platform
        ///     None,                                 // no specific masks
        ///     CustomShape::Rectangle,
        /// );
        /// ```
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

    // StaticObject trait implementations
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

    impl BaseTrait for StaticObject {
        fn update(&mut self, _delta_time: f32) {
            // Static objects don't update their state
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    /// An animated game object with physics simulation and scripting capabilities.
    /// 
    /// Animated objects can move, respond to collisions, and execute scripted
    /// behaviors. They are suitable for player characters, NPCs, projectiles,
    /// and other dynamic game elements.
    /// 
    /// # Physics System
    /// 
    /// Animated objects participate in the physics simulation with:
    /// - Velocity-based movement
    /// - Collision detection and response
    /// - Delta-time based updates for frame-rate independence
    /// 
    /// # Scripting System
    /// 
    /// Objects can execute sequences of scripted behaviors that can:
    /// - Modify object properties over time
    /// - Respond to game events
    /// - Create complex AI behaviors
    /// - Implement cutscenes and animations
    /// 
    /// # Fields
    /// 
    /// * `z_index` - Rendering layer
    /// * `id` - Unique identifier
    /// * `name` - Human-readable name
    /// * `pos` - Current position with rotation
    /// * `size` - Object dimensions
    /// * `masks` - Collision detection masks
    /// * `velocity` - Current movement vector
    /// * `shape` - Geometric shape
    /// * `sequence` - Optional scripted behavior sequence
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// # use crate::{AnimatedObject, PointWithDeg, Size, Velocity, CustomShape};
    /// let player = AnimatedObject::new(
    ///     10,                             // high z_index (foreground)
    ///     "Player".to_string(),
    ///     PointWithDeg::new(100.0, 100.0, 0.0),
    ///     Size::new(32.0, 32.0),
    ///     Velocity::new(0.0, 0.0),       // initially stationary
    ///     Some(vec![1]),                  // player collision mask
    ///     CustomShape::Circle,
    /// );
    /// ```
    #[derive(Default)]
    pub struct AnimatedObject {
        /// The rendering layer of this object (0-255, higher renders on top)
        pub z_index: u8,
        /// Unique identifier for this object instance
        pub id: Uuid,
        /// Human-readable name for debugging and identification
        pub name: String,
        /// Current position in 2D space with rotation angle
        pub pos: PointWithDeg,
        /// Width and height dimensions of the object
        pub size: Size,
        /// Collision detection layer masks this object belongs to
        pub masks: Vec<usize>,
        /// Current velocity vector (pixels per second)
        pub velocity: Velocity,
        /// Geometric shape used for collision detection and rendering
        pub shape: CustomShape,
        /// Optional sequence of scripted behaviors to execute
        pub sequence: Option<Vec<ScriptFn>>,
    }

    impl AnimatedObject {
        /// Creates a new animated object with physics and scripting capabilities.
        ///
        /// # Arguments
        /// 
        /// * `z_index` - The rendering layer (0-255, higher values on top)
        /// * `name` - Human-readable name for the object
        /// * `pos` - Initial position with rotation
        /// * `size` - Width and height of the object
        /// * `velocity` - Initial velocity vector (pixels per second)
        /// * `masks` - Optional collision masks (defaults to empty if None)
        /// * `shape` - Geometric shape for collision and rendering
        ///
        /// # Returns
        /// 
        /// A new `AnimatedObject` instance with a generated unique ID
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// # use crate::{AnimatedObject, PointWithDeg, Size, Velocity, CustomShape};
        /// // Create a moving projectile
        /// let bullet = AnimatedObject::new(
        ///     5,                              // mid-layer
        ///     "Bullet".to_string(),
        ///     PointWithDeg::new(0.0, 0.0, 45.0), // angled
        ///     Size::new(4.0, 4.0),           // small size
        ///     Velocity::new(200.0, -200.0),  // moving right and up
        ///     Some(vec![3]),                  // projectile mask
        ///     CustomShape::Circle,
        /// );
        /// ```
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
                sequence: None,
            }
        }
    }

    // AnimatedObject trait implementations
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
        /// Performs collision detection at a hypothetical new position.
        /// 
        /// This method checks if the object would collide with any other objects
        /// in the game world if it were moved to the specified position. It does
        /// not actually move the object, making it safe for collision prediction.
        /// 
        /// # Collision Detection Process
        /// 
        /// 1. Creates a virtual object at the new position
        /// 2. Iterates through all collision mask rows (1-14)
        /// 3. Checks against both static and animated objects
        /// 4. Uses the shape-based collision detection system
        /// 5. Skips self-collision checks
        /// 
        /// # Arguments
        /// 
        /// * `new_point` - The hypothetical position to test for collisions
        /// 
        /// # Returns
        /// 
        /// * `true` if a collision would occur at the new position
        /// * `false` if the position is safe (no collisions)
        /// 
        /// # Performance Notes
        /// 
        /// This method may be called frequently during movement calculations,
        /// so the collision detection system should be optimized for performance.
        fn check_collision(&self, new_point: PointWithDeg) -> bool {
            let this_obj_id = self.get_id().to_string();
            let virtual_obj = (new_point, self.size, self.get_shape());

            // Check collision against all mask rows (1-14 are valid collision layers)
            for row in 1..15 {
                let row_of_mask = match get_mask_row(row) {
                    Ok(mask) => mask,
                    Err(_) => continue, // Skip invalid mask rows
                };

                for global_object_id in row_of_mask.iter() {
                    // Skip self-collision check to prevent objects from colliding with themselves
                    if *global_object_id == this_obj_id {
                        continue;
                    }

                    // Check collision with static objects in this mask row
                    if let Ok(static_ids) = get_static_identifiable() {
                        if static_ids.contains(global_object_id) {
                            if let Ok(g_obj) = get_static_object(global_object_id) {
                                let g_obj = g_obj.lock().unwrap();
                                let other_obj = (g_obj.get_pos(), g_obj.get_size(), g_obj.get_shape());
                                
                                if check_collision(virtual_obj.clone(), other_obj) {
                                    return true; // Collision detected
                                }
                            }
                        }
                    }

                    // Check collision with other animated objects in this mask row
                    if let Ok(animated_ids) = get_animated_identifiable() {
                        if animated_ids.contains(global_object_id) {
                            if let Ok(g_obj) = get_animated_object(global_object_id) {
                                let g_obj = g_obj.lock().unwrap();
                                let other_obj = (g_obj.get_pos(), g_obj.get_size(), g_obj.get_shape());
                                
                                if check_collision(virtual_obj.clone(), other_obj) {
                                    return true; // Collision detected
                                }
                            }
                        }
                    }
                }
            }

            false // No collisions detected
        }

        /// Attempts to move the object with intelligent collision response.
        /// 
        /// This method implements a sophisticated movement system that tries to
        /// move the object by its full velocity, but gracefully handles collisions
        /// by attempting progressively smaller movements until a collision-free
        /// movement is found.
        /// 
        /// # Movement Algorithm
        /// 
        /// 1. Calculate target movement based on velocity and delta time
        /// 2. Try to move the full distance
        /// 3. If collision detected, try 90% of the distance
        /// 4. Continue reducing by 10% until movement succeeds or becomes negligible
        /// 5. Update object position and scale velocity accordingly
        /// 
        /// # Collision Response
        /// 
        /// When a collision blocks movement, the object's velocity is scaled down
        /// proportionally to the successful movement factor. This creates realistic
        /// physics behavior where objects slow down when hitting obstacles.
        /// 
        /// # Arguments
        /// 
        /// * `delta_time` - Time elapsed since the last frame in seconds
        /// 
        /// # Returns
        /// 
        /// * `true` if movement was completely blocked (no movement possible)
        /// * `false` if movement succeeded (full or partial)
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// # let mut player = AnimatedObject::default();
        /// let delta_time = 0.016; // 60 FPS
        /// let blocked = player.move_object(delta_time);
        /// 
        /// if blocked {
        ///     println!("Player hit a wall!");
        /// } else {
        ///     println!("Player moved successfully");
        /// }
        /// ```
        fn move_object(&mut self, delta_time: f32) -> bool {
            let vel = self.velocity.scale(delta_time);
            let mut factor = 1.0;

            // Try progressively smaller movements until collision-free movement is found
            while factor >= 0.1 {
                let scaled_vel = vel.scale(factor);
                let new_pos = PointWithDeg {
                    x: self.pos.x + scaled_vel.x,
                    y: self.pos.y + scaled_vel.y,
                    deg: self.pos.deg, // Rotation doesn't change during movement
                };

                // Test if this movement would cause a collision
                if !self.check_collision(new_pos) {
                    self.pos = new_pos;
                    self.velocity = self.velocity.scale(factor); // Scale velocity to match successful movement
                    return false; // Movement succeeded
                }

                factor -= 0.1; // Try a smaller movement
            }

            // No valid movement found; object remains stationary
            true // Movement was completely blocked
        }
    }

    impl SequenceTrait for AnimatedObject {
        fn add_script(&mut self, script: Vec<ScriptFn>) {
            self.sequence = Some(script);
        }

        fn run_sequence(&mut self) {
            if let Some(sequence) = &mut self.sequence {
                if !sequence.is_empty() {
                    // SAFETY: This unsafe block handles the closure execution
                    // TODO: Consider safer alternatives to raw pointer manipulation
                    let _res = unsafe {
                        let closure_ptr: *mut Box<
                            dyn for<'a> FnMut(&'a mut dyn SequenceParamTraits) -> bool + Send + Sync + 'static
                        > = &mut sequence[0];

                        let closure: &mut Box<
                            dyn for<'a> FnMut(&'a mut dyn SequenceParamTraits) -> bool + Send + Sync + 'static
                        > = &mut *closure_ptr;

                        closure(self as &mut dyn SequenceParamTraits)
                    };

                    // TODO: Handle sequence completion and move to next segment
                }
            }
        }
    }

    impl BaseTrait for AnimatedObject {
        fn update(&mut self, delta_time: f32) {
            self.run_sequence();
            self.process(delta_time);
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }
}