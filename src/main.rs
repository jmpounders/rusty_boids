use macroquad::prelude::*;

use std::time::{SystemTime, Duration, UNIX_EPOCH};

mod boids;
use boids::Movement;

mod universe;
use universe::Universe;

fn window_conf() -> Conf {
    Conf {
        window_title: "Boids".to_owned(),
        window_height: 640,
        window_width: 640,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("Boids simulator");

    // Simulation parameters
    // Note that these boids live in a grid universe
    let n_boids = 100;
    let (nx, ny) = (10, 10);
    let max_velocity = 1.0;
    let mut flock: Vec<boids::Boid> = Vec::new();

    // Set characteristic time scale
    let t_c = 25.0;
    let dtau = 1.0 / t_c;

    // Create boids
    for _ in 0..n_boids {
        flock.push(
            boids::Boid::new_random(&max_velocity, &nx, &ny)
        );
    }

    // Create universe
    let conf = window_conf();
    let universe = Universe::new(
        conf.window_width as f32, 
        conf.window_height as f32, 
        nx, ny
    );

    // Start simulation loop
    let mut t_prev = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let mut t_now: Duration;
    loop {
        t_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let _dt = (t_now-t_prev).as_micros();
        
        universe.reset();
        for b in &flock {
            universe.add_point(b.position[0], b.position[1]);
        }

        // Compute center of mass
        let (com_x, com_y) = boids::get_average_position(&flock);

        // Accumulate forces on each boid
        for i in 0..flock.len() {
            let nbrs = boids::get_neighbors(&flock, &i);

            let b = &flock[i];
            let (dx_bdry, dy_bdry) = boids::get_boundary_deltav(b, &nx, &ny);
            let (dx_com, dy_com) = boids::get_cohesion_deltav(b, &com_x, &com_y);
            let (dx_vel, dy_vel) = boids::get_alignment_deltav(b, nbrs.as_slice());
            let (dx_sep, dy_sep) = boids::get_separation_deltav(b, nbrs.as_slice());
            let (dx, dy) = (
                dx_bdry+dx_com+dx_vel+dx_sep,
                dy_bdry+dy_com+dy_vel+dy_sep,
            );

            flock[i].accelerate(&dtau, &dx, &dy);
            flock[i].update_position(&dtau);
        }

        next_frame().await;
        t_prev = t_now;
    }
}