pub mod engine_state {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex, RwLock},
    };

    use once_cell::sync::Lazy;

    use crate::engine::traits::{PhysicsObjectTrait, StaticObjectTrait};

    /// Represents possible return types when querying the global state.
    pub enum GlobalStateResult {
        /// A static object implementing `StaticObjectTrait`.
        StaticOjbect(Arc<Mutex<Box<dyn StaticObjectTrait>>>),
        /// An animated/physics object implementing `PhysicsObjectTrait`.
        Animatedbject(Arc<Mutex<Box<dyn PhysicsObjectTrait>>>),
        /// No object found.
        None,
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
    }

    impl Default for GlobalState {
        /// Constructs a default `GlobalState` with empty registries and maps.
        fn default() -> Self {
            Self {
                masks: [(); 15].map(|_| Vec::new()),
                s_z_index: [(); 255].map(|_| Vec::new()),
                a_z_index: [(); 255].map(|_| Vec::new()),
                s_identifiables: Vec::new(),
                a_identifiables: Vec::new(),
                s_map: HashMap::new(),
                a_map: HashMap::new(),
            }
        }
    }

    impl GlobalState {
        /// Constructs a new `GlobalState` instance.
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
            let index = mask.checked_sub(1).ok_or("mask must be >= 1")?;
            if index >= 15 {
                return Err("mask out of range, must be between 1 and 15".to_string());
            }

            self.masks[index].push(item);
            Ok(())
        }

        /// Removes an object ID from a specific mask row.
        pub fn remove_mask(&mut self, row: usize, id: String) {
            self.masks[row].retain(|x| x != &id);
        }

        // ====================
        // Z-Index Management
        // ====================

        /// Adds a static object ID to a z-index layer.
        pub fn append_static_z_index(&mut self, row: usize, id: String) {
            self.s_z_index[row].push(id);
        }

        /// Adds an animated object ID to a z-index layer.
        pub fn append_animated_z_index(&mut self, row: usize, id: String) {
            self.a_z_index[row].push(id);
        }

        /// Removes a static object ID from a z-index layer.
        pub fn remove_static_z_index(&mut self, row: usize, id: String) {
            self.s_z_index[row].retain(|x| x != &id);
        }

        /// Removes an animated object ID from a z-index layer.
        pub fn remove_animated_z_index(&mut self, row: usize, id: String) {
            self.a_z_index[row].retain(|x| x != &id);
        }

        // ====================
        // Object Identifiables
        // ====================

        /// Registers a static object's ID.
        pub fn append_static_identifiable(&mut self, id: String) {
            self.s_identifiables.push(id);
        }

        /// Registers an animated object's ID.
        pub fn append_animated_identifiable(&mut self, id: String) {
            self.a_identifiables.push(id);
        }

        /// Unregisters a static object's ID.
        pub fn remove_static_identifiable(&mut self, id: String) {
            self.s_identifiables.retain(|x| x != &id);
        }

        /// Unregisters an animated object's ID.
        pub fn remove_animated_identifiable(&mut self, id: String) {
            self.a_identifiables.retain(|x| x != &id);
        }

        // ====================
        // Object Map Management
        // ====================

        /// Inserts a static object into the static map if the key doesn't exist.
        pub fn insert_s_map(&mut self, key: String, value: Arc<Mutex<Box<dyn StaticObjectTrait>>>) {
            if self.s_map.contains_key(&key) {
                return;
            }
            self.s_map.insert(key, value);
        }

        /// Inserts an animated object into the animated map if the key doesn't exist.
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
        pub fn remove_s_map(&mut self, key: String) {
            self.s_map.remove(&key);
        }

        /// Removes an animated object by key.
        pub fn remove_a_map(&mut self, key: String) {
            self.a_map.remove(&key);
        }

        // ====================
        // Getters
        // ====================

        /// Gets the list of object IDs in the specified mask row.
        pub fn get_mask_row(&self, row: usize) -> &Vec<String> {
            &self.masks[row]
        }

        /// Gets the list of static object IDs in a z-index layer.
        pub fn get_static_z_index_row(&self, row: usize) -> &Vec<String> {
            &self.s_z_index[row]
        }

        /// Gets the list of animated object IDs in a z-index layer.
        pub fn get_animated_z_index_row(&self, row: usize) -> &Vec<String> {
            &self.a_z_index[row]
        }

        /// Retrieves a static object by key.
        pub fn get_static_map_value(
            &self,
            key: &str,
        ) -> Option<Arc<Mutex<Box<dyn StaticObjectTrait>>>> {
            self.s_map.get(key).cloned()
        }

        /// Retrieves an animated object by key.
        pub fn get_animated_map_value(
            &self,
            key: &str,
        ) -> Option<Arc<Mutex<Box<dyn PhysicsObjectTrait>>>> {
            self.a_map.get(key).cloned()
        }
    }

    /// Thread-safe, lazily initialized global state for shared object registry.
    pub static GLOBAL_STATE: Lazy<Arc<RwLock<GlobalState>>> =
        Lazy::new(|| Arc::new(RwLock::new(GlobalState::new())));

    /// Adds a static object to the global state, updating masks, z-index, identifiers, and map.
    pub fn add_static_object(object: Arc<Mutex<Box<dyn StaticObjectTrait>>>) {
        let mut global_state = GLOBAL_STATE.write().unwrap();
        let obj = object.lock().unwrap();
        let id = obj.get_id().to_string();
        let obj_masks = obj.get_masks();
        let obj_z_index = obj.get_z_index() as usize;

        for obj_mask_row in obj_masks {
            global_state.append_mask(obj_mask_row, id.clone()).unwrap();
        }

        global_state.append_static_z_index(obj_z_index, id.clone());
        global_state.append_static_identifiable(id.clone());
        global_state.insert_s_map(id.clone(), Arc::clone(&object));
    }

    /// Adds an animated object to the global state, updating masks, z-index, identifiers, and map.
    pub fn add_animated_object(object: Arc<Mutex<Box<dyn PhysicsObjectTrait>>>) {
        let mut global_state = GLOBAL_STATE.write().unwrap();
        let obj = object.lock().unwrap();
        let id = obj.get_id().to_string();
        let obj_masks = obj.get_masks();
        let obj_z_index = obj.get_z_index() as usize;

        for obj_mask_row in obj_masks {
            global_state.append_mask(obj_mask_row, id.clone()).unwrap();
        }

        global_state.append_animated_z_index(obj_z_index, id.clone());
        global_state.append_animated_identifiable(id.clone());
        global_state.insert_a_map(id.clone(), Arc::clone(&object));
    }

    /// Removes a static object from all registries and maps using a given row and ID.
    pub fn remove_static_object(row: usize, id: String) {
        let mut global_state = GLOBAL_STATE.write().unwrap();

        global_state.remove_mask(row, id.clone());
        global_state.remove_static_z_index(row, id.clone());
        global_state.remove_static_identifiable(id.clone());
        global_state.remove_s_map(id.clone());
    }

    /// Removes an animated object from all registries and maps using a given row and ID.
    pub fn remove_animated_object(row: usize, id: String) {
        let mut global_state = GLOBAL_STATE.write().unwrap();

        global_state.remove_mask(row, id.clone());
        global_state.remove_animated_z_index(row, id.clone());
        global_state.remove_animated_identifiable(id.clone());
        global_state.remove_a_map(id.clone());
    }
}

#[cfg(test)]
mod testing_global_state_machine {
    // use std::sync::{Arc, Mutex};
    //
    // use super::engine_state::add_static_object;
    // use crate::{
    //     engine::{structures::StaticObject, traits::StaticObjectTrait},
    //     units::{PointWithDeg, Size},
    //     utils::shapes::CustomShape,
    // };
    //
    // fn gen_static_object() -> StaticObject {
    //     StaticObject::new(
    //         1,
    //         String::from("test"),
    //         PointWithDeg::new(0.0, 0.0, None),
    //         Size::new(10.0, 5.0),
    //         Some(vec![1, 6]),
    //         CustomShape::gen_triangle(),
    //     )
    // }
    //
    // fn gen_animated_object() -> StaticObject {
    //     StaticObject::new(
    //         1,
    //         String::from("test"),
    //         PointWithDeg::new(0.0, 0.0, None),
    //         Size::new(10.0, 5.0),
    //         Some(vec![1, 6]),
    //         CustomShape::gen_triangle(),
    //     )
    // }

    // #[test]
    // fn test_add_and_remove_static_object() {
    //     let test_obj: Arc<Mutex<Box<dyn StaticObjectTrait>>> =
    //         Arc::new(Mutex::new(Box::new(gen_static_object())));
    //
    //     add_static_object(test_obj);
    // }
}
