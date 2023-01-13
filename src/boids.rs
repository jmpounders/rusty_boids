use rand::prelude::*;

#[derive(Debug)]
pub struct Boid {
    pub position: [f32; 2],
    pub velocity: [f32; 2]
}

impl Boid {
    pub fn new_random(max_velocity: &f32, nx: &i32, ny: &i32) -> Self {
        let mut rng = rand::thread_rng();
        let (nx, ny) = (*nx as f32, *ny as f32);
        let x: f32 = rng.gen::<f32>()*nx;
        let y: f32 = rng.gen::<f32>()*ny;
        let speed: f32 = *max_velocity;
        let dir: f32 = rng.gen_range(0.0..std::f32::consts::PI);
        Self {
            position: [x, y],
            velocity: [speed*dir.cos(), speed*dir.sin()]
        }
    }
}

pub trait Movement {
    fn accelerate(&mut self, dt: &f32, dx: &f32, dy: &f32);
    fn update_position(&mut self, dt: &f32);
}

impl Movement for Boid {
    fn accelerate(&mut self, dt: &f32, vx: &f32, vy: &f32) {
        self.velocity[0] += dt*vx;
        self.velocity[1] += dt*vy;

        let speed = norm(&self.velocity[0], &self.velocity[1]);
        let mut alpha = 1.0;
        if (speed>1.2) | (speed<0.8) {
            alpha = 1.0/speed;
        }
        self.velocity[0] *= alpha;
        self.velocity[1] *= alpha;
    }

    fn update_position(&mut self, dt: &f32) {
        self.position[0] += dt*self.velocity[0];
        self.position[1] += dt*self.velocity[1];
    }
}

fn norm(dx: &f32, dy: &f32) -> f32 {
    (dx.powi(2) + dy.powi(2)).sqrt()
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

pub fn get_neighbors<'a>(flock: &'a [Boid], self_ref: &usize) -> Vec<&'a Boid> {
    let mut nbrs: Vec<&Boid> = Vec::new();
    let b = &flock[*self_ref];
    for (i,bo) in flock.iter().enumerate() {
        if i == *self_ref {
            continue;
        }
        let dx = bo.position[0] - b.position[0];
        let dy = bo.position[1] - b.position[1];
        let dist = (dx.powi(2) + dy.powi(2)).sqrt();
        let in_perception = dx*b.position[0] + dy*b.position[1];
        if (dist<1.0) & (in_perception>0.0) {
            nbrs.push(bo);
        }
    }
    nbrs
}

pub fn get_boundary_deltav(boid: &Boid, nx: &i32, ny: &i32) -> (f32, f32) {
    let coeff: f32 = 1.0;
    let margin: f32 = 1.0;
    let dx: f32;
    let dy: f32;

    let (max_x, max_y) = (*nx as f32, *ny as f32);

    // Right and left
    if boid.position[0] > (max_x-margin) {
        dx = max_x - margin - boid.position[0];
    } else if boid.position[0] < margin {
        dx = margin - boid.position[0];
    } else {
        dx = 0.0;
    }

    // Bottom and top
    if boid.position[1] > (max_y-margin) {
        dy = max_y - margin - boid.position[1];
    } else if boid.position[1] < margin {
        dy = margin - boid.position[1]
    } else {
        dy = 0.0;
    }

    (coeff*dx, coeff*dy)
}

pub fn get_cohesion_deltav(boid: &Boid, com_x: &f32, com_y: &f32) -> (f32, f32) {
    let coeff: f32 = 0.1;
    let (dx, dy) = ((com_x - boid.position[0]), (com_y - boid.position[1]));
    (coeff*dx, coeff*dy)
}

pub fn get_alignment_deltav(boid: &Boid, nbrs: &[&Boid]) -> (f32, f32) {
    let coeff: f32 = 0.5;
    let (mut avg_x, mut avg_y) = (0.0, 0.0);
    for b in nbrs {
        avg_x += b.velocity[0];
        avg_y += b.velocity[1];
    }
    let n = nbrs.len() as f32;
    let (dx, dy) = if n > 0.0 {
        (avg_x/n - boid.velocity[0], avg_y/n - boid.velocity[1])
    } else {
        (0.0, 0.0)
    };
    (coeff*dx, coeff*dy)
}

pub fn get_separation_deltav(boid: &Boid, nbrs: &[&Boid]) -> (f32, f32) {
    let coeff: f32 = 1.0;
    let (mut avg_x, mut avg_y, mut n) = (0.0, 0.0, 0.0);
    for b in nbrs {
        let d = ((b.position[0]-boid.position[0]).powi(2) + 
                 (b.position[1]-boid.position[1]).powi(2)).sqrt();
        if d<0.5 {
            avg_x += b.position[0];
            avg_y += b.position[1];
            n += 1.0;
        }
    }
    let (dx, dy) = if n > 0.0 {
        (boid.position[0] - avg_x/n, boid.position[1] - avg_y/n)
    } else {
        (0.0, 0.0)
    };
    (coeff*dx, coeff*dy)
}