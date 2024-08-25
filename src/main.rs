extern crate libc;
extern crate nalgebra_glm as glm;
use draw::draw::Draw;
use macroquad::prelude::*;
use sim::body::body::GenericBody;
use sim::body::springsbody::SpringsBody;
use sim::sim::Simulation;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
mod draw;
mod export;
mod sim;

fn pause() {
    println!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Springs"),
        window_width: 1000,
        window_height: 1000,
        ..Default::default()
    }
}

#[derive(Clone)]
struct RunConfig {
    dt: f32,
    g: f32,
    dhat: f32,
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = env::args().collect();
    const min_num_params: usize = 13;
    if args.len() < min_num_params {
        eprintln!("Usage: {} ...", args[0]);
        eprintln!("Expected at least {} params", min_num_params);
        eprintln!("Got params: {:?}", args);
        std::process::exit(1);
    }

    let dt: f32 = args[1].parse().expect("Invalid f32 argument");
    let g: f32 = args[2].parse().expect("Invalid f32 argument");
    let max_iters: u32 = args[3].parse().expect("Invalid u32 argument");
    let tol: f32 = args[4].parse().expect("Invalid f32 argument");
    let max_linesearch_step: u32 = args[5].parse().expect("Invalid u32 argument");
    let tau: f32 = args[6].parse().expect("Invalid f32 argument");
    let beta: f32 = args[7].parse().expect("Invalid f32 argument");
    let dhat: f32 = args[8].parse().expect("Invalid f32 argument");
    let s: f32 = args[9].parse().expect("Invalid f32 argument");
    let accd_max_iter: u32 = args[10].parse().expect("Invalid u32 argument");
    let nonstop: bool = args[11].parse().expect("Invalid bool argument");
    let save_frames: bool = args[12].parse().expect("Invalid bool argument");
    let save_path: String = args[13].parse().expect("Invalid path");

    // ###################### init saving folder ######################
    let path = Path::new(&save_path);
    // check if path is valid
    if !path.is_dir() {
        // try mkdir
        match fs::create_dir(path) {
            Ok(_) => println!("Successfully created directory: {}", save_path),
            Err(e) => panic!("Failed to create directory: {}. Error: {}", save_path, e),
        }
    } else {
        panic!("Directory already exists: {}", save_path);
    }

    // ###################### create and init simulator ######################
    let run_config = RunConfig { g, dt, dhat };

    let spbody =
        SpringsBody::from_file_with_v0("springs.spr").expect("Failed to read spring file!");
    let spbody2 =
        SpringsBody::from_file_with_v0("springs2.spr").expect("Failed to read spring file!");
    let mut gen_bodies: Vec<GenericBody> = Vec::new();
    gen_bodies.push(GenericBody::Springs(spbody));
    gen_bodies.push(GenericBody::Springs(spbody2));

    let mut sim = Simulation::new(gen_bodies, &run_config);

    // ############################# init end ################################

    let mut istep = 0;
    loop {
        clear_background(BLACK);

        // step
        sim.step_damped_newton_with_contact(
            max_iters,
            tol,
            max_linesearch_step,
            tau,
            beta,
            s,
            accd_max_iter,
        );
        if save_frames {
            sim.export();
        }

        // draw
        for body in &sim.bodies {
            body.draw();
        }

        println!("Simulated frame {}!\n", istep);

        if !nonstop {
            pause();
        }

        if save_frames {
            // just panic if save fails
            let filepath = path.join(Path::new(&format!("frame_{}", istep)));
            let filename = filepath.to_str().unwrap();

            sim.save(filename)
                .expect(&format!("Saving frame {} failed!", istep));
            // delete previous frames
            for index in 0..istep {
                let filepath = path.join(Path::new(&format!("frame_{}", index)));
                if filepath.exists() {
                    fs::remove_file(&filepath).expect(&format!("Failed to delete frame {}", index));
                }
            }
        }

        // dbg!(&sim.x);
        istep += 1;

        next_frame().await
    }
}
