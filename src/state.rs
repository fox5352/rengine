pub mod engine_state {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex, RwLock},
    };

    use once_cell::sync::Lazy;

    use crate::engine::traits::{BaseTrait, PhysicsObjectTrait, StaticObjectTrait};

    /// Represents possible return types when querying the global state.
    pub enum GlobalStateResult {
        /// A static object implementing `StaticObjectTrait`.
        StaticOjbect(Arc<Mutex<Box<dyn StaticObjectTrait>>>),
        /// An animated/physics object implementing `PhysicsObjectTrait`.
        Animatedbject(Arc<Mutex<Box<dyn PhysicsObjectTrait>>>),
        /// No object found.
        None,
    }

    pub enum ObjectType {
        StaticObject,
        AnimatedObject        
    }

    pub struct Capsule {
        obj_type: ObjectType,
        obj: Box<dyn BaseTrait>
    }

    /// Central registry for managing masks, z-index ordering, and object mappings.
    pub struct GlobalState {
        /// Mask registry: 15 mask slots, each storing IDs of associated objects.
        masks: [Vec<String>; 15],

        /// Static objects' z-index registry: 255 slots for drawing/rendering order.
        s_z_index: [Vec<String>; 255],

        /// Animated objects' z-index registry: 255 slots for drawing/rendering order.
        a_z_index: [Vec<String>; 255],

        /// Identifiers for static objects (keys to `s_map`).
        s_identifiables: Vec<String>,

        /// Identifiers for animated/physics objects (keys to `a_map`).
        a_identifiables: Vec<String>,

        /// Map of static objects.
        s_map: HashMap<String, Arc<Mutex<Box<dyn StaticObjectTrait>>>>,

        /// Map of animated/physics objects.
        a_map: HashMap<String, Arc<Mutex<Box<dyn PhysicsObjectTrait>>>>,
        // TODO: i want to put a_map and s_map in a hasmap here they both impl basetrait
        map: HashMap<String, Capsule>
    }

    impl Default for GlobalState {
        fn default() -> Self {
            Self {
                masks: [(); 15].map(|_| Vec::new()),
                s_z_index: [(); 255].map(|_| Vec::new()),
                a_z_index: [(); 255].map(|_| Vec::new()),
                s_identifiables: Vec::new(),
                a_identifiables: Vec::new(),
                s_map: HashMap::new(),
                a_map: HashMap::new(),
                map: HashMap::new()
            }
        }
    }

    impl GlobalState {
        /// Constructs a new `GlobalState` instance.
        ///
        /// Returns a new instance of `GlobalState`.
        pub fn new() -> Self {
            Self::default()
        }

        // ====================
        // Mask Management
        // ====================

        /// Adds an object ID to a specified mask row.
        ///
        /// # Arguments
        /// * `mask` - 1-based index (1-15) of the mask.
        /// * `item` - Object ID to insert.
        ///
        /// # Errors
        /// Returns an error if index is out of range.
        pub fn append_mask(&mut self, mask: usize, item: String) -> Result<(), String> {
            if !(1..=15).contains(&mask) {
                return Err("mask out of range, must be between 1 and 15".to_string());
            }

            if let Some(row) = self.masks.get_mut(mask - 1) {
                row.push(item);
            }

            Ok(())
        }

        /// Removes an object ID from a specific mask row.
        ///
        /// # Arguments
        /// * `mask` - 1-based index (1-15) of the mask.
        /// * `item` - Object ID to remove.
        ///
        /// # Errors
        /// Returns an error if index is out of range.
        pub fn remove_mask(&mut self, row: usize, id: String) -> Result<(), String> {
            if !(1..=15).contains(&row) {
                return Err("mask out of range, must be between 1 and 15".to_string());
            }
            self.masks[row - 1].retain(|x| x != &id);
            Ok(())
        }

        // ====================
        // Z-Index Management
        // ====================

        /// Adds a static object ID to a z-index layer.
        ///
        /// # Arguments
        /// * `row` - 1-based index (1-255) of the z-index layer.
        /// * `id` - Object ID to insert.
        ///
        /// # Errors
        /// Returns an error if index is out of range.
        pub fn append_static_z_index(&mut self, row: usize, id: String) -> Result<(), String> {
            if !(1..=255).contains(&row) {
                return Err("z-index out of range, must be between 1 and 255".to_string());
            }

            if let Some(row) = self.s_z_index.get_mut(row - 1) {
                row.push(id);
            }

            Ok(())
        }

        /// Adds an animated object ID to a z-index layer.
        ///
        /// # Arguments
        /// * `row` - 1-based index (1-255) of the z-index layer.
        /// * `id` - Object ID to insert.
        ///
        /// # Errors
        /// Returns an error if index is out of range.
        pub fn append_animated_z_index(&mut self, row: usize, id: String) -> Result<(), String> {
            if !(1..=255).contains(&row) {
                return Err("z-index out of range, must be between 1 and 255".to_string());
            }

            if let Some(row) = self.a_z_index.get_mut(row - 1) {
                row.push(id);
            }

            Ok(())
        }

        /// Removes a static object ID from a z-index layer.
        ///
        /// # Arguments
        /// * `row` - 1-based index (1-255) of the z-index layer.
        /// * `id` - Object ID to remove.
        ///
        /// # Errors
        /// Returns an error if index is out of range.
        pub fn remove_static_z_index(&mut self, row: usize, id: String) -> Result<(), String> {
            if !(1..=255).contains(&row) {
                return Err("z-index out of range, must be between 1 and 255".to_string());
            }
            self.s_z_index[row - 1].retain(|x| x != &id);
            Ok(())
        }

        /// Removes an animated object ID from a z-index layer.
        ///
        /// # Arguments
        /// * `row` - 1-based index (1-255) of the z-index layer.
        /// * `id` - Object ID to remove.
        ///
        /// # Errors
        /// Returns an error if index is out of range.
        pub fn remove_animated_z_index(&mut self, row: usize, id: String) -> Result<(), String> {
            if !(1..=255).contains(&row) {
                return Err("z-index out of range, must be between 1 and 255".to_string());
            }
            self.a_z_index[row - 1].retain(|x| x != &id);
            Ok(())
        }

        // ====================
        // Object Identifiables
        // ====================

        /// Registers a static object's ID.
        ///
        /// # Arguments
        /// * `id` - Object ID to insert.
        pub fn append_static_identifiable(&mut self, id: String) {
            self.s_identifiables.push(id);
        }

        /// Registers an animated object's ID.
        ///
        /// # Arguments
        /// * `id` - Object ID to insert.
        pub fn append_animated_identifiable(&mut self, id: String) {
            self.a_identifiables.push(id);
        }

        /// Unregisters a static object's ID.
        ///
        /// # Arguments
        /// * `id` - Object ID to remove.
        pub fn remove_static_identifiable(&mut self, id: String) {
            self.s_identifiables.retain(|x| x != &id);
        }

        /// Unregisters an animated object's ID.
        ///
        /// # Arguments
        /// * `id` - Object ID to remove.
        pub fn remove_animated_identifiable(&mut self, id: String) {
            self.a_identifiables.retain(|x| x != &id);
        }

        // ====================
        // Object Map Management
        // ====================

        /// Inserts a static object into the static map if the key doesn't exist.
        ///
        /// # Arguments
        /// * `key` - Object ID to insert.
        /// * `value` - Static object to insert.
        pub fn insert_s_map(&mut self, key: String, value: Arc<Mutex<Box<dyn StaticObjectTrait>>>) {
            if self.s_map.contains_key(&key) {
                return;
            }
            self.s_map.insert(key, value);
        }

        /// Inserts an animated object into the animated map if the key doesn't exist.
        ///
        /// # Arguments
        /// * `key` - Object ID to insert.
        /// * `value` - Animated object to insert.
        pub fn insert_a_map(
            &mut self,
            key: String,
            value: Arc<Mutex<Box<dyn PhysicsObjectTrait>>>,
        ) {
            if self.a_map.contains_key(&key) {
                return;
            }
            self.a_map.insert(key, value);
        }

        /// Removes a static object by key.
        ///
        /// # Arguments
        /// * `key` - Object ID to remove.
        pub fn remove_s_map(&mut self, key: String) {
            self.s_map.remove(&key);
        }

        /// Removes an animated object by key.
        ///
        /// # Arguments
        /// * `key` - Object ID to remove.
        pub fn remove_a_map(&mut self, key: String) {
            self.a_map.remove(&key);
        }

        // ====================
        // Getters
        // ====================

        /// Gets the list of object IDs in the specified mask row.
        ///
        /// # Arguments
        /// * `row` - 1-based index (1-15) of the mask.
        ///
        /// # Success
        /// Returns the list of object IDs in the specified mask row.
        ///
        /// # Errors
        /// Returns an error if index is out of range.
        pub fn get_mask_row(&self, row: usize) -> Result<Vec<String>, String> {
            if !(1..=15).contains(&row) {
                return Err("mask out of range, must be between 1 and 15".to_string());
            }
            Ok(self.masks[row - 1].clone())
        }

        /// Gets the list of static object IDs in a z-index layer.
        ///
        /// # Arguments
        /// * `row` - 1-based index (1-255) of the z-index layer.
        ///
        /// # Success
        /// Returns the list of static object IDs in the specified z-index layer.
        ///
        /// # Errors
        /// Returns an error if index is out of range.
        pub fn get_static_z_index_row(&self, row: usize) -> Result<Vec<String>, String> {
            if !(1..=255).contains(&row) {
                return Err("z-index out of range, must be between 1 and 255".to_string());
            }
            Ok(self.s_z_index[row - 1].clone())
        }

        /// Gets the list of animated object IDs in a z-index layer.
        ///
        /// # Arguments
        /// * `row` - 1-based index (1-255) of the z-index layer.
        ///
        /// # Success
        /// Returns the list of animated object IDs in the specified z-index layer.
        ///
        /// # Errors
        /// Returns an error if index is out of range.
        pub fn get_animated_z_index_row(&self, row: usize) -> Result<Vec<String>, String> {
            if !(1..=255).contains(&row) {
                return Err("z-index out of range, must be between 1 and 255".to_string());
            }
            Ok(self.a_z_index[row - 1].clone())
        }

        /// Retrieves the list of static object IDs.
        ///
        /// Returns the list of static object IDs.
        pub fn get_static_identifiables(&self) -> Vec<String> {
            self.s_identifiables.clone()
        }

        /// Retrieves the list of animated object IDs.
        ///
        /// Returns the list of animated object IDs.
        pub fn get_animated_identifiables(&self) -> Vec<String> {
            self.a_identifiables.clone()
        }

        /// Retrieves a static object by key.
        ///
        /// # Arguments
        /// * `key` - Object ID to retrieve.
        ///
        /// # Success
        /// Returns the static object with the specified key.
        ///
        /// # Errors
        /// Returns an error if the object is not found.
        pub fn get_static_object(
            &self,
            key: &str,
        ) -> Option<Arc<Mutex<Box<dyn StaticObjectTrait>>>> {
            self.s_map.get(key).cloned()
        }

        /// Retrieves an animated object by key.
        ///
        /// # Arguments
        /// * `key` - Object ID to retrieve.
        ///
        /// # Success
        /// Returns the animated object with the specified key.
        ///
        /// # Errors
        /// Returns an error if the object is not found.
        pub fn get_animated_object(
            &self,
            key: &str,
        ) -> Option<Arc<Mutex<Box<dyn PhysicsObjectTrait>>>> {
            self.a_map.get(key).cloned()
        }
    }

    /// Thread-safe, lazily initialized global state for shared object registry.
    pub static GLOBAL_STATE: Lazy<Arc<RwLock<GlobalState>>> =
        Lazy::new(|| Arc::new(RwLock::new(GlobalState::new())));

    // ====================
    // Public Functions to Manage Global State
    // =====================

    /// Gets a list of object IDs in the specified mask row.
    ///
    /// # Arguments
    /// * `row` - 1-based index (1-15) of the mask.
    ///
    /// # Success
    /// Returns the list of object IDs in the specified mask row.
    ///
    /// # Errors
    /// Returns an error if index is out of range.
    pub fn get_mask_row(row: usize) -> Result<Vec<String>, String> {
        let global_state = GLOBAL_STATE
            .read()
            .map_err(|_| "Failed to lock on get_mask_row".to_string())?;

        let mask_row = global_state.get_mask_row(row)?;

        drop(global_state);

        Ok(mask_row)
    }

    /// Gets a list of static object IDs in a z-index layer.
    ///
    /// # Arguments
    /// * `row` - 1-based index (1-255) of the z-index layer.
    ///
    /// # Success
    /// Returns the list of static object IDs in the specified z-index layer.
    ///
    /// # Errors
    /// Returns an error if index is out of range.
    pub fn get_static_z_index_row(row: usize) -> Result<Vec<String>, String> {
        let global_state = GLOBAL_STATE
            .read()
            .map_err(|_| "Failed to lock on get_static_z_index_row".to_string())?;

        let mask_row = global_state.get_static_z_index_row(row)?;

        drop(global_state);

        Ok(mask_row)
    }

    /// Gets a list of animated object IDs in a z-index layer.
    ///
    /// # Arguments
    /// * `row` - 1-based index (1-255) of the z-index layer.
    ///
    /// # Success
    /// Returns the list of animated object IDs in the specified z-index layer.
    ///
    /// # Errors
    /// Returns an error if index is out of range.
    pub fn get_animated_z_index_row(row: usize) -> Result<Vec<String>, String> {
        let global_state = GLOBAL_STATE
            .read()
            .map_err(|_| "Failed to lock on get_animated_z_index_row".to_string())?;

        let mask_row = global_state.get_animated_z_index_row(row)?;

        drop(global_state);

        Ok(mask_row)
    }

    /// Retrieves the list of static object IDs.
    ///
    /// # Success
    /// Returns the list of static object IDs.
    pub fn get_static_identifiable() -> Result<Vec<String>, String> {
        let global_state = GLOBAL_STATE
            .read()
            .map_err(|_| "Failed to lock on get_static_identifiable".to_string())?;

        let mask_row = global_state.get_static_identifiables();

        drop(global_state);

        Ok(mask_row)
    }

    /// Retrieves the list of static object IDs.
    ///
    /// # Arguments
    /// * `key` - Object ID to retrieve.
    ///
    /// # Success
    /// Returns the static object with the specified key.
    ///
    /// # Errors
    /// Returns an error if the object is not found.
    pub fn get_static_object(key: &str) -> Result<Arc<Mutex<Box<dyn StaticObjectTrait>>>, String> {
        let global_state = GLOBAL_STATE
            .read()
            .map_err(|_| "Failed to lock on get_static_object".to_string())?;

        if let Some(obj) = global_state.get_static_object(key) {
            Ok(obj)
        } else {
            Err("Failed to get static object".to_string())
        }
    }

    pub fn get_animated_object(
        key: &str,
    ) -> Result<Arc<Mutex<Box<dyn PhysicsObjectTrait>>>, String> {
        let global_state = GLOBAL_STATE
            .read()
            .map_err(|_| "Failed to lock on get_animated_object".to_string())?;

        if let Some(obj) = global_state.get_animated_object(key) {
            Ok(obj)
        } else {
            Err("Failed to get animated object".to_string())
        }
    }

    /// Retrieves the list of animated object IDs.
    ///
    /// # Success
    /// Returns the list of animated object IDs.
    pub fn get_animated_identifiable() -> Result<Vec<String>, String> {
        let global_state = GLOBAL_STATE
            .read()
            .map_err(|_| "Failed to lock on get_animated_identifiable".to_string())?;

        let mask_row = global_state.get_animated_identifiables();

        drop(global_state);

        Ok(mask_row)
    }

    // ====================
    // Public Functions to Add Objects to Global State
    // =====================

    /// Adds a mask to a specific row in the global state.
    ///
    /// # Arguments
    /// * `row` - 1-based index (1-15) of the mask.
    /// * `id` - Object ID to insert.
    ///
    /// # Success
    /// Returns `Ok(())` if the mask is successfully added to the global state.
    ///
    /// # Errors
    /// Returns an error if the mask fails to be added to the global state.
    pub fn append_mask_to_row(row: usize, id: String) -> Result<(), String> {
        let mut global_state = GLOBAL_STATE
            .write()
            .map_err(|_| "Failed to lock on append_mask".to_string())?;

        global_state.append_mask(row, id)?;

        drop(global_state);

        Ok(())
    }

    /// Adds a static object ID to a z-index layer.
    ///
    /// # Arguments
    /// * `row` - 1-based index (1-255) of the z-index layer.
    /// * `id` - Object ID to insert.
    ///
    /// # Success
    /// Returns `Ok(())` if the object ID is successfully added to the global state.
    ///
    /// # Errors
    /// Returns an error if the object ID fails to be added to the global state.
    pub fn append_static_id_to_z_index_row(row: usize, id: String) -> Result<(), String> {
        let mut global_state = GLOBAL_STATE
            .write()
            .map_err(|_| "Failed to lock on append_id_to_index_row".to_string())?;

        global_state.append_static_z_index(row, id)?;

        drop(global_state);

        Ok(())
    }

    /// Adds an animated object ID to a z-index layer.
    ///
    /// # Arguments
    /// * `row` - 1-based index (1-255) of the z-index layer.
    /// * `id` - Object ID to insert.
    ///
    /// # Success
    /// Returns `Ok(())` if the object ID is successfully added to the global state.
    ///
    /// # Errors
    /// Returns an error if the object ID fails to be added to the global state.
    pub fn append_animated_id_to_z_index_row(row: usize, id: String) -> Result<(), String> {
        let mut global_state = GLOBAL_STATE
            .write()
            .map_err(|_| "Failed to lock on append_id_to_index_row".to_string())?;

        global_state.append_animated_z_index(row, id)?;

        drop(global_state);

        Ok(())
    }

    /// Adds a static object ID to the global state.
    ///
    /// # Arguments
    /// * `id` - Object ID to insert.
    ///
    /// # Success
    /// Returns `Ok(())` if the object ID is successfully added to the global state.
    ///
    /// # Errors
    /// Returns an error if the object ID fails to be added to the global state.
    pub fn append_static_identifiable(id: String) -> Result<(), String> {
        let mut global_state = GLOBAL_STATE
            .write()
            .map_err(|_| "Failed to lock on append_static_identifiable".to_string())?;

        global_state.append_static_identifiable(id);

        drop(global_state);

        Ok(())
    }

    /// Adds an animated object ID to the global state.
    ///
    /// # Arguments
    /// * `id` - Object ID to insert.
    ///
    /// # Success
    /// Returns `Ok(())` if the object ID is successfully added to the global state.
    ///
    /// # Errors
    /// Returns an error if the object ID fails to be added to the global state.
    pub fn append_animated_identifiable(id: String) -> Result<(), String> {
        let mut global_state = GLOBAL_STATE
            .write()
            .map_err(|_| "Failed to lock on append_animated_identifiable".to_string())?;

        global_state.append_animated_identifiable(id);

        drop(global_state);

        Ok(())
    }

    /// Adds a static object to the global state.
    ///
    /// # Arguments
    /// * `id` - Object ID to insert.
    /// * `obj` - Object to insert.
    ///
    /// # Success
    /// Returns `Ok(())` if the object is successfully added to the global state.
    ///
    /// # Errors
    /// Returns an error if the object fails to be added to the global state.
    pub fn insert_static_object(
        id: String,
        obj: Arc<Mutex<Box<dyn StaticObjectTrait>>>,
    ) -> Result<(), String> {
        let mut global_state = GLOBAL_STATE
            .write()
            .map_err(|_| "Failed to lock on insert_static_object".to_string())?;

        global_state.insert_s_map(id, obj);

        drop(global_state);

        Ok(())
    }

    /// Adds an animated object to the global state.
    ///
    /// # Arguments
    /// * `id` - Object ID to insert.
    /// * `obj` - Object to insert.
    ///
    /// # Success
    /// Returns `Ok(())` if the object is successfully added to the global state.
    ///
    /// # Errors
    /// Returns an error if the object fails to be added to the global state.
    pub fn insert_animated_object(
        id: String,
        obj: Arc<Mutex<Box<dyn PhysicsObjectTrait>>>,
    ) -> Result<(), String> {
        let mut global_state = GLOBAL_STATE
            .write()
            .map_err(|_| "Failed to lock on insert_animated_object".to_string())?;

        global_state.insert_a_map(id, obj);

        drop(global_state);

        Ok(())
    }

    // ====================
    // Public Functions to Remove Objects from Global State
    // ====================

    /// Removes an object ID from a mask row.
    ///
    /// # Arguments
    /// * `row` - 1-based index (1-255) of the z-index layer.
    /// * `id` - Object ID to remove.
    ///
    /// # Success
    /// Returns `Ok(())` if the object ID is successfully removed from the global state.
    ///
    /// # Errors
    /// Returns an error if the object ID fails to be removed from the global state.
    pub fn remove_mask_from_row(row: usize, id: String) -> Result<(), String> {
        let mut global_state = GLOBAL_STATE
            .write()
            .map_err(|_| "Failed to lock on remove_mask_from_row".to_string())?;

        global_state
            .remove_mask(row, id)
            .map_err(|_| "Failed to remove masks from row".to_string())?;

        drop(global_state);

        Ok(())
    }

    /// Removes an object ID from a static z-index layer.
    ///
    /// # Arguments
    /// * `row` - 1-based index (1-255) of the z-index layer.
    /// * `id` - Object ID to remove.
    ///
    /// # Success
    /// Returns `Ok(())` if the object ID is successfully removed from the global state.
    ///
    /// # Errors
    /// Returns an error if the object ID fails to be removed from the global state.
    pub fn remove_static_z_index_from_row(row: usize, id: String) -> Result<(), String> {
        let mut global_state = GLOBAL_STATE
            .write()
            .map_err(|_| "Failed to lock on remove_static_z_index_from_row".to_string())?;

        global_state
            .remove_static_z_index(row, id)
            .map_err(|_| "Failed to remove masks from row".to_string())?;

        drop(global_state);

        Ok(())
    }

    /// Removes an object ID from an animated z-index layer.
    ///
    /// # Arguments
    /// * `row` - 1-based index (1-255) of the z-index layer.
    /// * `id` - Object ID to remove.
    ///
    /// # Success
    /// Returns `Ok(())` if the object ID is successfully removed from the global state.
    ///
    /// # Errors
    /// Returns an error if the object ID fails to be removed from the global state.
    pub fn remove_animated_z_index_from_row(row: usize, id: String) -> Result<(), String> {
        let mut global_state = GLOBAL_STATE
            .write()
            .map_err(|_| "Failed to lock on remove_animated_z_index_from_row".to_string())?;

        global_state
            .remove_animated_z_index(row, id)
            .map_err(|_| "Failed to remove masks from row".to_string())?;

        drop(global_state);

        Ok(())
    }

    /// Removes an object ID from the global state.
    ///
    /// # Arguments
    /// * `id` - Object ID to remove.
    ///
    /// # Success
    /// Returns `Ok(())` if the object ID is successfully removed from the global state.
    ///
    /// # Errors
    /// Returns an error if the object ID fails to be removed from the global state.
    pub fn remove_static_identifiable(id: String) -> Result<(), String> {
        let mut global_state = GLOBAL_STATE
            .write()
            .map_err(|_| "Failed to lock on remove_static_identifiable".to_string())?;

        global_state.remove_static_identifiable(id);

        drop(global_state);

        Ok(())
    }

    /// Removes an object ID from the global state.
    ///
    /// # Arguments
    /// * `id` - Object ID to remove.
    ///
    /// # Success
    /// Returns `Ok(())` if the object ID is successfully removed from the global state.
    ///
    /// # Errors
    /// Returns an error if the object ID fails to be removed from the global state.
    pub fn remove_animated_identifiable(id: String) -> Result<(), String> {
        let mut global_state = GLOBAL_STATE
            .write()
            .map_err(|_| "Failed to lock on remove_animated_identifiable".to_string())?;

        global_state.remove_animated_identifiable(id);

        drop(global_state);

        Ok(())
    }

    /// Removes an object ID from the global state.
    ///
    /// # Arguments
    /// * `id` - Object ID to remove.
    ///
    /// # Success
    /// Returns `Ok(())` if the object ID is successfully removed from the global state.
    ///
    /// # Errors
    /// Returns an error if the object ID fails to be removed from the global state.
    pub fn remove_static_object(id: String) -> Result<(), String> {
        let mut global_state = GLOBAL_STATE
            .write()
            .map_err(|_| "Failed to lock on remove_static_object".to_string())?;

        global_state.remove_s_map(id);

        drop(global_state);

        Ok(())
    }

    /// Removes an object ID from the global state.
    ///
    /// # Arguments
    /// * `id` - Object ID to remove.
    ///
    /// # Success
    /// Returns `Ok(())` if the object ID is successfully removed from the global state.
    ///
    /// # Errors
    /// Returns an error if the object ID fails to be removed from the global state.
    pub fn remove_animated_object(id: String) -> Result<(), String> {
        let mut global_state = GLOBAL_STATE
            .write()
            .map_err(|_| "Failed to lock on remove_animated_object".to_string())?;

        global_state.remove_a_map(id);

        drop(global_state);

        Ok(())
    }

    // ====================
    // Public Functions to bookkeep and manage Global State automagicly
    // ====================

    /// Adds a static object to the global state. and manage's the Global State automagicly
    /// bookkeeping.
    ///
    /// # Arguments
    /// * `obj` - Object to insert.
    ///
    /// # Success
    /// Returns `Ok(())` if the object ID is successfully added to the global state.
    ///
    /// # Errors
    /// Returns an error if the object ID fails to be added to the global state.
    pub fn a_add_s_object(obj: Arc<Mutex<Box<dyn StaticObjectTrait>>>) -> Result<(), String> {
        let lock_obj = obj
            .lock()
            .map_err(|_| "Failed to lock on a_add_s_object".to_string())?;

        let id = lock_obj.get_id().to_string();

        for row in lock_obj.get_masks() {
            append_mask_to_row(row, id.clone())?;
        }

        append_static_id_to_z_index_row(lock_obj.get_z_index() as usize, id.clone())?;

        append_static_identifiable(id.clone())?;

        drop(lock_obj);

        insert_static_object(id, obj)?;

        Ok(())
    }

    /// Adds an animated object to the global state. and manage's the Global State automagicly
    /// bookkeeping.
    ///
    /// # Arguments
    /// * `obj` - Object to insert.
    ///
    /// # Success
    /// Returns `Ok(())` if the object ID is successfully added to the global state.
    ///
    /// # Errors
    /// Returns an error if the object ID fails to be added to the global state.
    pub fn a_add_a_object(obj: Arc<Mutex<Box<dyn PhysicsObjectTrait>>>) -> Result<(), String> {
        let lock_obj = obj
            .lock()
            .map_err(|_| "Failed to lock on a_add_a_object".to_string())?;

        let id = lock_obj.get_id().to_string();

        for row in lock_obj.get_masks() {
            append_mask_to_row(row, id.clone())?;
        }

        append_animated_id_to_z_index_row(lock_obj.get_z_index() as usize, id.clone())?;

        append_animated_identifiable(id.clone())?;

        drop(lock_obj);

        insert_animated_object(id, obj)?;

        Ok(())
    }

    /// Removes a static object from the global state. and manage's the Global State automagicly
    /// bookkeeping.
    ///
    /// # Arguments
    /// * `obj` - Object to remove.
    ///
    /// # Success
    /// Returns `Ok(())` if the object ID is successfully removed from the global state.
    pub fn a_remove_s_object(obj: Arc<Mutex<Box<dyn StaticObjectTrait>>>) -> Result<(), String> {
        let lock_obj = obj
            .lock()
            .map_err(|_| "Failed to lock on a_remove_s_object".to_string())?;

        let id = lock_obj.get_id().to_string();

        for row in lock_obj.get_masks() {
            remove_mask_from_row(row, id.clone())?;
        }

        remove_static_z_index_from_row(lock_obj.get_z_index() as usize, id.clone())?;

        remove_static_identifiable(id.clone())?;

        drop(lock_obj);

        remove_static_object(id)?;

        Ok(())
    }

    /// Removes an animated object from the global state. and manage's the Global State automagicly
    /// bookkeeping.
    ///
    /// # Arguments
    /// * `obj` - Object to remove.
    ///
    /// # Success
    /// Returns `Ok(())` if the object ID is successfully removed from the global state.
    pub fn a_remove_a_object(obj: Arc<Mutex<Box<dyn PhysicsObjectTrait>>>) -> Result<(), String> {
        let lock_obj = obj
            .lock()
            .map_err(|_| "Failed to lock on a_remove_a_object".to_string())?;

        let id = lock_obj.get_id().to_string();

        for row in lock_obj.get_masks() {
            remove_mask_from_row(row, id.clone())?;
        }

        remove_animated_z_index_from_row(lock_obj.get_z_index() as usize, id.clone())?;

        remove_animated_identifiable(id.clone())?;

        drop(lock_obj);

        remove_animated_object(id)?;

        Ok(())
    }
}

#[cfg(test)]
mod testing_global_state_machine {

    use std::sync::{Arc, Mutex};

    use serial_test::serial;

    use crate::{
        engine::{
            structures::{AnimatedObject, StaticObject},
            traits::{PhysicsObjectTrait, StaticObjectTrait},
        },
        state::engine_state::{
            a_add_a_object, a_add_s_object, a_remove_a_object, a_remove_s_object,
            append_mask_to_row, get_animated_identifiable, get_animated_object,
            get_animated_z_index_row, get_static_z_index_row, remove_animated_identifiable,
            remove_animated_object, remove_static_identifiable,
        },
        units::{PointWithDeg, Size, Velocity},
        utils::shapes::CustomShape,
    };

    use super::engine_state::{
        append_animated_id_to_z_index_row, append_animated_identifiable,
        append_static_id_to_z_index_row, append_static_identifiable, get_mask_row,
        get_static_identifiable, get_static_object, insert_animated_object, insert_static_object,
        remove_animated_z_index_from_row, remove_mask_from_row, remove_static_object,
        remove_static_z_index_from_row,
    };

    fn _gen_static_object() -> StaticObject {
        StaticObject::new(
            1,
            String::from("test"),
            PointWithDeg::new(0.0, 0.0, None),
            Size::new(10.0, 5.0),
            Some(vec![1]),
            CustomShape::gen_triangle(),
        )
    }

    fn _gen_animated_object() -> AnimatedObject {
        AnimatedObject::new(
            1,
            String::from("test"),
            PointWithDeg::new(0.0, 0.0, None),
            Size::new(10.0, 5.0),
            Velocity::new(),
            Some(vec![1]),
            CustomShape::gen_triangle(),
        )
    }
    #[test]
    #[serial]
    fn test_append_1_on_each_mask_row_and_remove_it() {
        let id_template = String::from("test");
        for i in 1..15 {
            append_mask_to_row(i, id_template.clone()).unwrap();
        }

        for i in 1..15 {
            remove_mask_from_row(i, id_template.clone()).unwrap();
        }

        for i in 1..15 {
            assert_eq!(get_mask_row(i).unwrap().len(), 0);
        }
    }

    #[test]
    #[serial]
    fn test_append_1_on_all_static_z_index_rows_and_remove_it() {
        let id_template = String::from("test");
        for i in 1..255 {
            append_static_id_to_z_index_row(i, id_template.clone()).unwrap();
        }

        for i in 1..255 {
            assert_eq!(get_static_z_index_row(i).unwrap().len(), 1);
        }

        for i in 1..255 {
            remove_static_z_index_from_row(i, id_template.clone()).unwrap();
        }

        for i in 1..255 {
            assert_eq!(get_static_z_index_row(i).unwrap().len(), 0);
        }
    }

    #[test]
    #[serial]
    fn test_append_1_on_all_animated_z_index_rows_and_remove_it() {
        let id_template = String::from("test");
        for i in 1..255 {
            append_animated_id_to_z_index_row(i, id_template.clone()).unwrap();
        }

        for i in 1..255 {
            assert_eq!(get_animated_z_index_row(i).unwrap().len(), 1);
        }

        for i in 1..255 {
            remove_animated_z_index_from_row(i, id_template.clone()).unwrap();
        }

        for i in 1..255 {
            assert_eq!(get_animated_z_index_row(i).unwrap().len(), 0);
        }
    }

    #[test]
    #[serial]
    fn test_append_1_to_static_identifiables_and_remove_it() {
        let id_template = String::from("test");
        append_static_identifiable(id_template.clone()).unwrap();

        assert_eq!(get_static_identifiable().unwrap().len(), 1);

        remove_static_identifiable(id_template.clone()).unwrap();

        assert_eq!(get_static_identifiable().unwrap().len(), 0);
    }

    #[test]
    #[serial]
    fn test_append_1_to_animated_identifiables_and_remove_it() {
        let id_template = String::from("test");

        append_animated_identifiable(id_template.clone()).unwrap();

        assert_eq!(get_animated_identifiable().unwrap().len(), 1);

        remove_animated_identifiable(id_template.clone()).unwrap();

        assert_eq!(get_animated_identifiable().unwrap().len(), 0);
    }

    #[test]
    #[serial]
    fn test_insert_static_object_into_map_and_remove() {
        let obj: Arc<Mutex<Box<dyn StaticObjectTrait>>> = Arc::new(Mutex::new(Box::new(
            _gen_static_object(),
        )
            as Box<dyn StaticObjectTrait>));
        let obj_id = obj.lock().unwrap().get_id().to_string();

        {
            insert_static_object(obj_id.clone(), Arc::clone(&obj)).unwrap();
        }

        {
            let obj_2 = get_static_object(&obj_id).unwrap();

            assert!(Arc::ptr_eq(&obj, &obj_2));
        }

        {
            remove_static_object(obj_id.clone()).unwrap();

            assert!(get_static_object(&obj_id).is_err());
        }
    }

    #[test]
    #[serial]
    fn test_insert_animated_object_into_map_and_remove() {
        let obj: Arc<Mutex<Box<dyn PhysicsObjectTrait>>> = Arc::new(Mutex::new(Box::new(
            _gen_animated_object(),
        )
            as Box<dyn PhysicsObjectTrait>));
        let obj_id = obj.lock().unwrap().get_id().to_string();

        {
            insert_animated_object(obj_id.clone(), Arc::clone(&obj)).unwrap();
        }

        {
            let obj_2 = get_animated_object(&obj_id).unwrap();

            assert!(Arc::ptr_eq(&obj, &obj_2));
        }

        {
            remove_animated_object(obj_id.clone()).unwrap();

            assert!(get_animated_object(&obj_id).is_err());
        }

        {
            let obj_2 = get_animated_object(&obj_id);
            assert!(obj_2.is_err());
        }
    }

    #[test]
    #[serial]
    fn test_auto_add_object_static() {
        let obj = Arc::new(Mutex::new(
            Box::new(_gen_static_object()) as Box<dyn StaticObjectTrait>
        ));

        let obj_id = obj.lock().unwrap().get_id().to_string();
        let masks_rows = obj.lock().unwrap().get_masks();

        // Perform the addition
        a_add_s_object(Arc::clone(&obj)).unwrap();

        // Check the object was inserted
        let fetched_obj = get_static_object(&obj_id).unwrap();
        assert!(Arc::ptr_eq(&obj, &fetched_obj));

        // check if in ifentifiables
        let statics = get_static_identifiable().unwrap();
        assert!(statics.contains(&obj_id));

        // check if in z-index rows
        let z_row = get_static_z_index_row(obj.lock().unwrap().get_z_index() as usize).unwrap();
        assert!(z_row.contains(&obj_id));

        // Check it appears in the correct mask rows
        for row in masks_rows {
            let global_mask_row = get_mask_row(row).unwrap();
            let mut found = false;

            for global_mask in global_mask_row {
                if global_mask == obj_id {
                    found = true;
                    break;
                }
            }

            assert!(found);
        }
    }

    #[test]
    #[serial]
    fn test_auto_add_object_animated() {
        let obj = Arc::new(Mutex::new(
            Box::new(_gen_animated_object()) as Box<dyn PhysicsObjectTrait>
        ));

        let obj_id = obj.lock().unwrap().get_id().to_string();
        let masks_rows = obj.lock().unwrap().get_masks();

        // Perform the addition
        a_add_a_object(Arc::clone(&obj)).unwrap();

        // Check the object was inserted
        let fetched_obj = get_animated_object(&obj_id).unwrap();
        assert!(Arc::ptr_eq(&obj, &fetched_obj));

        // check if in identifiables
        let anims = get_animated_identifiable().unwrap();
        assert!(anims.contains(&obj_id));

        // check if in z-index rows
        let z_row = get_animated_z_index_row(obj.lock().unwrap().get_z_index() as usize).unwrap();
        assert!(z_row.contains(&obj_id));

        // Check it appears in the correct mask rows
        for row in masks_rows {
            let global_mask_row = get_mask_row(row).unwrap();
            let mut found = false;

            for global_mask in global_mask_row {
                if global_mask == obj_id {
                    found = true;
                    break;
                }
            }

            assert!(found);
        }
    }

    #[test]
    #[serial]
    fn test_auto_remove_object_static() {
        let obj = Arc::new(Mutex::new(
            Box::new(_gen_static_object()) as Box<dyn StaticObjectTrait>
        ));

        let obj_id = obj.lock().unwrap().get_id().to_string();

        // Perform the addition
        a_add_s_object(Arc::clone(&obj)).unwrap();

        // Check the object was inserted
        let fetched_obj = get_static_object(&obj_id).unwrap();
        assert!(Arc::ptr_eq(&obj, &fetched_obj));

        // Perform the removal
        a_remove_s_object(Arc::clone(&obj)).unwrap();

        // Check the object was removed
        let obj_2 = get_static_object(&obj_id);
        assert!(obj_2.is_err());

        // check if in identifiables
        let statics = get_static_identifiable().unwrap();
        assert!(!statics.contains(&obj_id));

        // check if in z-index rows
        let z_index = obj.lock().unwrap().get_z_index() as usize;
        if let Ok(z_row) = get_static_z_index_row(z_index) {
            assert!(!z_row.contains(&obj_id));
        }

        // Check it appears in the correct mask rows
        for row in obj.lock().unwrap().get_masks() {
            let global_mask_row = get_mask_row(row).unwrap();
            let mut found = false;

            for global_mask in global_mask_row {
                if global_mask == obj_id {
                    found = true;
                    break;
                }
            }

            assert!(!found);
        }
    }

    #[test]
    #[serial]
    fn test_auto_remove_object_animated() {
        let obj = Arc::new(Mutex::new(
            Box::new(_gen_animated_object()) as Box<dyn PhysicsObjectTrait>
        ));

        let obj_id = obj.lock().unwrap().get_id().to_string();

        // Perform the addition
        a_add_a_object(Arc::clone(&obj)).unwrap();

        // Check the object was inserted
        let fetched_obj = get_animated_object(&obj_id).unwrap();
        assert!(Arc::ptr_eq(&obj, &fetched_obj));

        // Perform the removal
        a_remove_a_object(Arc::clone(&obj)).unwrap();

        // Check the object was removed
        let obj_2 = get_animated_object(&obj_id);
        assert!(obj_2.is_err());

        // check if in identifiables
        let anims = get_animated_identifiable().unwrap();
        assert!(!anims.contains(&obj_id));

        // check if in z-index rows
        let z_index = obj.lock().unwrap().get_z_index() as usize;
        if let Ok(z_row) = get_animated_z_index_row(z_index) {
            assert!(!z_row.contains(&obj_id));
        }

        // Check it appears in the correct mask rows
        for row in obj.lock().unwrap().get_masks() {
            let global_mask_row = get_mask_row(row).unwrap();
            let mut found = false;

            for global_mask in global_mask_row {
                if global_mask == obj_id {
                    found = true;
                    break;
                }
            }

            assert!(!found);
        }
    }
}
