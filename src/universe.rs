use macroquad::prelude::*;

#[derive(Debug)]
pub struct Universe {
    pub nx: i32,
    pub ny: i32,
    pub scale_x: f32,
    pub scale_y: f32
}

impl Universe {
    pub fn new(width: f32, height: f32, nx: i32, ny: i32) -> Self {
        //let (box_w, box_h) = (screen_width() as f32, screen_height() as f32);
        let scale_x = width/(nx as f32);
        let scale_y = height/(ny as f32);
        Universe { nx, ny, scale_x, scale_y}
    }

    pub fn reset(&self) {
        clear_background(BLACK);
    }

    pub fn add_point(&self, x: f32, y: f32) {
        draw_circle(x*self.scale_x, y*self.scale_y, 6.0, RED);
    }
}