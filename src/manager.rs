use std::{sync::Mutex, time::Instant};

use crate::{
    engine::{Object, StaticObject},
    scene::{self, PhysicsObjectTrait, World},
};

/// Main game loop function
pub struct GameLoop<S, A>
where
    S: IntoIterator<Item = Box<dyn Object>>,
    A: IntoIterator<Item = Box<dyn PhysicsObjectTrait>>,
{
    last_time: Instant,
    scene: Mutex<Option<World<'a,S, A>>>,
}

impl<S, A> GameLoop<S, A>
where
    S: IntoIterator<Item = Box<dyn Object>>,
    A: IntoIterator<Item = Box<dyn PhysicsObjectTrait>>,
{
    pub fn new() -> Self {
        return Self {
            last_time: Instant::now(),
            scene: Mutex::new(None),
        };
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_time);
        let delta_time = delta_time.as_secs_f32();

        // pass delter time here to dedicated updater funtion

        // if let Some(scene) = self.scene.as_ref() {
        //     for s_obj in scene.static_objects {
        //     }
        //     for a_obj in scene.animated_objects {

        //     }
        // }

        self.last_time = now;
    }

    pub fn update_game(self, delta_time: f32) {
        let scene = self.scene.lock().expect("Failed to get lock of scene mutex");

        if let Some(scene) =  scene.as_ref(){
            
        }


    }

    pub fn run(mut self, scene: Option<World<S, A>>)
    where
        S: IntoIterator<Item = Box<dyn Object>>,
        A: IntoIterator<Item = Box<dyn PhysicsObjectTrait>>,
    {
        let mut counter = 0;

        self.scene = scene;

        loop {
            self.update();

            #[cfg(debug_assertions)]
            {
                counter += 1;

                if counter >= 5 {
                    let mut buffer = String::new();

                    std::io::stdin().read_line(&mut buffer).expect("testing");
                    counter = 0;
                }
            }
        }
    }
}
