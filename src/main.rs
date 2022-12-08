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

        for b in &mut flock {
            let (dx, dy) = boids::get_boundary_deltav(b, &box_w, &box_h);
            b.accelerate(&dx, &dy);
            b.update_position();
        }

        next_frame().await
    }
}