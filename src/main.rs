use macroquad::prelude::*;

mod boids;
use boids::Movement;

fn window_conf() -> Conf {
    Conf {
        window_title: "Boids".to_owned(),
        window_height: 480,
        window_width: 640,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("Boids simulator");

    let (box_w, box_h) = (screen_width() as f32, screen_height() as f32);
    let n_boids = 5;
    let max_velocity = 1.0;
    let mut flock: Vec<boids::Boid> = Vec::new();

    for _ in 0..n_boids {
        flock.push(boids::Boid::new_random(&box_w, &box_h, &max_velocity));
    }
    for b in &flock {
        println!("{:?}", b)
    }

    loop {
        clear_background(BLACK);
        for b in &flock {
            draw_circle(b.position[0], b.position[1], 6.0, RED);
        }

        // Compute center of mass and average velocity
        let (com_x, com_y) = boids::get_average_position(&flock);
        let (vel_x, vel_y) = boids::get_average_velocity(&flock);

        // Problem that I hit here:
        // I wanted to combine the delta_v comps and the update
        // inside one for loop, but the former only needed immut refs
        // while the latter needed mut refs.  particuarly a problem
        // when calling get_neighbors
        let mut delta_v: Vec<(f32, f32)> = Vec::new();
        for b in &flock {
            let nbrs = boids::get_neighbors(&flock, b);
            let (dx_bdry, dy_bdry) = boids::get_boundary_deltav(b, &box_w, &box_h);
            let (dx_com, dy_com) = boids::get_cohesion_deltav(b, &com_x, &com_y);
            let (dx_vel, dy_vel) = boids::get_alignment_deltav(b, &vel_x, &vel_y);
            let (dx_sep, dy_sep) = boids::get_separation_deltav(b, nbrs.as_slice());
            let (dx, dy) = (
                dx_bdry+dx_com/100.0+dx_vel/100.0+dx_sep/100.0,
                dy_bdry+dy_com/100.0+dy_vel/100.0+dy_sep/100.0,
            );
            delta_v.push((dx, dy));
        }

        // Update positions
        for (b, (dx, dy)) in flock.iter_mut().zip(delta_v) {
            b.accelerate(&dx, &dy);
            b.update_position();
        }

        next_frame().await
    }
}