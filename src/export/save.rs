use std::{fs, path::Path};

use crate::sim::sim::Simulation;

pub struct Save {
    active: bool,
    path: Box<Path>,
    istep: u32,
}

impl Save {
    pub fn new(active: bool, path_str: String) -> Self {
        let path = Box::<Path>::from(Path::new(&path_str));
        if active {
            if !path.is_dir() {
                // try mkdir
                match fs::create_dir(&path) {
                    Ok(_) => println!("Successfully created directory: {}", path_str),
                    Err(e) => panic!("Failed to create directory: {}. Error: {}", path_str, e),
                }
            } else {
                panic!("Directory already exists: {}", path_str);
            }
        }

        Self {
            active: active,
            path: path,
            istep: 0,
        }
    }

    pub fn save(&mut self, sim: &Simulation) {
        if self.active {
            // just panic if save fails
            let filepath = self.path.join(Path::new(&format!("frame_{}", self.istep)));
            let filename = filepath.to_str().unwrap();

            sim.save(filename)
                .expect(&format!("Saving frame {} failed!", self.istep));
            // delete previous frames
            for index in 0..self.istep {
                let filepath = self.path.join(Path::new(&format!("frame_{}", index)));
                if filepath.exists() {
                    fs::remove_file(&filepath).expect(&format!("Failed to delete frame {}", index));
                }
            }
            self.istep += 1;
        }
    }
}
