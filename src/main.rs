extern crate nalgebra_glm as glm;
use draw::draw::Draw;
use macroquad::prelude::*;
use sim::body::body::GenericBody;
use sim::body::springsbody::SpringsBody;
use sim::sim::Simulation;
use std::env;
use std::io::{self, Write};
mod draw;
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
    if args.len() != 11 {
        eprintln!("Usage: {} <dt> <g> <max_iters> <tol> <nonstop>", args[0]);
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
    let nonstop: bool = args[10].parse().expect("Invalid bool argument");

    let run_config = RunConfig { g, dt, dhat };

    // ###################### create and init simulator ######################

    let spbody = SpringsBody::from_file("springs.spr").expect("Failed to read spring file!");
    // let spbody2 = SpringsBody::from_file("springs2.spr").expect("Failed to read spring file!");
    let mut gen_bodies: Vec<GenericBody> = Vec::new();
    gen_bodies.push(GenericBody::Springs(spbody));
    // gen_bodies.push(GenericBody::Springs(spbody2));

    let mut sim = Simulation::new(gen_bodies, &run_config);

    // ############################# init end ################################

    let mut istep = 0;
    loop {
        clear_background(BLACK);

        // step
        sim.step_damped_newton_with_contact(max_iters, tol, max_linesearch_step, tau, beta, s);

        // draw
        for body in &sim.bodies {
            body.draw();
        }

        println!("Simulated frame {}!\n", istep);

        if !nonstop {
            pause();
        }

        // dbg!(&sim.x);
        istep += 1;

        next_frame().await
    }
}
