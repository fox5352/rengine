pub mod engine_state {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use crate::engine::traits::{PhysicsObjectTrait, StaticObjectTrait};

    pub enum GlobalStateResult {
        StaticOjbect(Arc<Mutex<Box<dyn StaticObjectTrait>>>),
        Animatedbject(Arc<Mutex<Box<dyn PhysicsObjectTrait>>>),
        None,
    }

    pub struct GlobalState {
        /// Mask registry: 15 mask slots containing lists of IDs of associated objects.
        masks: [Vec<String>; 15],

        /// ZIndex registry
        s_z_index: [Vec<String>; 255],
        a_z_index: [Vec<String>; 255],

        /// Identifiers for static objects (s_map keys).
        s_identifiables: Vec<String>,

        /// Identifiers for physics objects (a_map keys).
        a_identifiables: Vec<String>,

        /// Map of static objects. key: ID, value: Arc<Mutex<Box<dyn StaticObjectTrait>>>
        s_map: HashMap<String, Arc<Mutex<Box<dyn StaticObjectTrait>>>>,

        /// Map of physics objects. key: ID, value: Arc<Mutex<Box<dyn PhysicsObjectTrait>>>
        a_map: HashMap<String, Arc<Mutex<Box<dyn PhysicsObjectTrait>>>>,
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
            }
        }
    }

    impl GlobalState {
        pub fn new() -> Self {
            Self::default()
        }

        // mash managment
        /// Appends an item to a given mask index (1-based).
        ///
        /// # Arguments
        /// * `mask` - Index from 1 to 15 (inclusive).
        /// * `item` - String to append to the selected mask.
        ///
        /// # Errors
        /// Returns an error if the index is out of bounds.
        pub fn append_mask(&mut self, mask: usize, item: String) -> Result<(), String> {
            let index = mask.checked_sub(1).ok_or("mask must be >= 1")?;
            if index >= 15 {
                return Err("mask out of range, must be between 1 and 15".to_string());
            }

            self.masks[index].push(item);
            Ok(())
        }

        /// Removes the item specified from the mask index (1-based).
        ///
        /// # Arguments
        /// * 'row' - Index from 1 to 15 (inclusive).
        /// * 'id' - String to remove from the selected mask.
        pub fn remove_mask(&mut self, row: usize, id: String) {
            self.masks[row].retain(|x| x != &id);
        }

        // z-index managment
        /// Appends an static object to a given z-index index (1-based).
        ///
        /// # Arguments
        /// * 'row' - Index from 1 to 255 (inclusive).
        /// * 'id' - String to append to the selected mask.
        pub fn append_static_z_index(&mut self, row: usize, id: String) {
            self.s_z_index[row].push(id);
        }

        /// Appends an animated object to a given z-index index (1-based).
        ///
        /// # Arguments
        /// * 'row' - Index from 1 to 255 (inclusive).
        /// * 'id' - String to append to the selected mask.
        pub fn append_animated_z_index(&mut self, row: usize, id: String) {
            self.a_z_index[row].push(id);
        }

        /// Removes the static object specified from the z-index index (1-based).
        ///
        /// # Arguments
        /// * 'row' - Index from 1 to 255 (inclusive).
        /// * 'id' - String to remove from the selected mask.
        pub fn remove_static_z_index(&mut self, row: usize, id: String) {
            self.s_z_index[row].retain(|x| x != &id);
        }

        /// Removes the animated object specified from the z-index index (1-based).
        ///
        /// # Arguments
        /// * 'row' - Index from 1 to 255 (inclusive).
        /// * 'id' - String to remove from the selected mask.
        pub fn remove_animated_z_index(&mut self, row: usize, id: String) {
            self.a_z_index[row].retain(|x| x != &id);
        }

        // object managment
        /// Appends an item to a given mask index (1-based).
        ///
        /// # Arguments
        /// * 'id' - String to append to the selected mask.
        pub fn append_static_identifiable(&mut self, id: String) {
            self.s_identifiables.push(id);
        }

        /// Appends an item to a given mask index (1-based).
        ///
        /// # Arguments
        /// * 'id' - String to append to the selected mask.
        pub fn append_animated_identifiable(&mut self, id: String) {
            self.a_identifiables.push(id);
        }

        /// Inserts a new static object into the static map.
        pub fn insert_s_map(&mut self, key: String, value: Arc<Mutex<Box<dyn StaticObjectTrait>>>) {
            if self.s_map.contains_key(&key) {
                return;
            }

            self.s_map.insert(key, value);
        }

        /// Inserts a new animated object into the animated map.
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

        // Getters
        /// Returns a reference to the internal mask list.
        ///
        /// # Arguments
        /// * 'row' - Index from 1 to 15 (inclusive).
        ///
        /// # Returns
        /// A reference to the internal mask list.
        pub fn get_mask_row(&self, row: usize) -> &Vec<String> {
            &self.masks[row]
        }

        /// Returns a reference to the internal static z-index list.
        ///
        /// # Arguments
        /// * 'row' - Index from 1 to 255 (inclusive).
        ///
        /// # Returns
        /// A reference to the internal static z-index list.
        pub fn get_static_z_index_row(&self, row: usize) -> &Vec<String> {
            &self.s_z_index[row]
        }

        /// Returns a reference to the internal animated z-index list.
        ///
        /// # Arguments
        /// * 'row' - Index from 1 to 255 (inclusive).
        ///
        /// # Returns
        /// A reference to the internal animated z-index list.
        pub fn get_animated_z_index_row(&self, row: usize) -> &Vec<String> {
            &self.a_z_index[row]
        }

        /// Returns a reference static object by key.
        ///
        /// # Arguments
        /// * 'key' - The key of the static object to retrieve.
        ///
        /// # Returns
        /// A reference to the static object.
        pub fn get_static_map_value(
            &self,
            key: &str,
        ) -> Option<Arc<Mutex<Box<dyn StaticObjectTrait>>>> {
            self.s_map.get(key).cloned()
        }

        /// Returns a reference to the animated object by key.
        ///
        /// # Arguments
        /// * 'key' - The key of the animated object to retrieve.
        ///
        /// # Returns
        /// A reference to the animated object.
        pub fn get_animated_map_value(
            &self,
            key: &str,
        ) -> Option<Arc<Mutex<Box<dyn PhysicsObjectTrait>>>> {
            self.a_map.get(key).cloned()
        }
    }
}
