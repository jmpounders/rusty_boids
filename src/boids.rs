use rand::prelude::*;

#[derive(Debug)]
pub struct Boid {
    pub position: [f32; 2],
    pub velocity: [f32; 2]
}

impl Boid {
    pub fn new_random(max_x: &f32, max_y: &f32, max_velocity: &f32) -> Self {
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen();
        let y: f32 = rng.gen();
        let speed: f32 = rng.gen_range(0.1..*max_velocity);
        let dir: f32 = rng.gen_range(0.0..std::f32::consts::PI);
        Self {
            position: [max_x*x, max_y*y],
            velocity: [speed*dir.cos(), speed*dir.sin()]
        }
    }
}

pub trait Movement {
    fn translate(&mut self, dx: &f32, dy: &f32);
    fn accelerate(&mut self, dx: &f32, dy: &f32);
    fn update_position(&mut self);
}

impl Movement for Boid {
    fn translate(&mut self, dx: &f32, dy: &f32) {
        self.position[0] += dx;
        self.position[1] += dy;
    }

    fn accelerate(&mut self, vx: &f32, vy: &f32) {
        self.velocity[0] += vx;
        self.velocity[1] += vy;
    }

    fn update_position(&mut self) {
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];
    }
}

pub fn get_boundary_deltav(boid: &Boid, max_x: &f32, max_y: &f32) -> (f32, f32) {
    let dx: f32;
    let dy: f32;

    // Right and left
    if boid.position[0] > *max_x {
        dx = *max_x - boid.position[0];
    } else if boid.position[0] < 0.0 {
        dx = -boid.position[0];
    } else {
        dx = 0.0;
    }

    // Bottom and top
    if boid.position[1] > *max_y {
        dy = *max_y - boid.position[1];
    } else if boid.position[1] < 0.0 {
        dy = -boid.position[1]
    } else {
        dy = 0.0;
    }
    (dx/max_x, dy/max_y)
}

