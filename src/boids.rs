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

pub fn get_average_position(flock: &[Boid]) -> (f32, f32) {
    let (mut avg_x, mut avg_y) = (0.0, 0.0);
    for b in flock {
        avg_x += b.position[0];
        avg_y += b.position[1];
    }
    let n = flock.len() as f32;
    (avg_x/n, avg_y/n)
}

pub fn get_average_velocity(flock: &[Boid]) -> (f32, f32) {
    let (mut avg_x, mut avg_y) = (0.0, 0.0);
    for b in flock {
        avg_x += b.velocity[0];
        avg_y += b.velocity[1];
    }
    let n = flock.len() as f32;
    (avg_x/n, avg_y/n)
}

pub fn get_neighbors<'a>(flock: &'a [Boid], b: &Boid) -> Vec<&'a Boid> {
    let mut nbrs: Vec<&Boid> = Vec::new();
    for bo in flock {
        let d = ((b.position[0]-bo.position[1]).powi(2) + 
                 (b.position[0]-bo.position[1]).powi(2)).sqrt();
        if d<10.0 {
            nbrs.push(bo);
        }
    }
    nbrs
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

pub fn get_cohesion_deltav(boid: &Boid, com_x: &f32, com_y: &f32) -> (f32, f32) {
    ((com_x - boid.position[0]), (com_y - boid.position[1]))
}

pub fn get_alignment_deltav(boid: &Boid, vel_x: &f32, vel_y: &f32) -> (f32, f32) {
    ((vel_x - boid.velocity[0]), (vel_y - boid.velocity[1]))
}

pub fn get_separation_deltav(boid: &Boid, nbrs: &[&Boid]) -> (f32, f32) {
    let (mut avg_x, mut avg_y) = (0.0, 0.0);
    for b in nbrs {
        avg_x += b.position[0];
        avg_y += b.position[1];
    }
    let n = nbrs.len() as f32;
    if n > 0.0 {
        (boid.position[0] - avg_x/n, boid.position[1] - avg_y/n)
    } else {
        (0.0, 0.0)
    }
}