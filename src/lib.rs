// Declare the modules so Rust knows about them
pub mod engine; // Contains core game object definitions and traits
pub mod manager;
pub mod scene;
pub mod state;
pub mod types;
pub mod units;
pub mod utils; // Defines the game world and holds collections of objects // Contains the game loop and manages object updates/input
pub mod window;

pub mod scripts; // add custom scripts for objects here
