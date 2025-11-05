use crate::objects::*;

#[derive(Clone)]
pub struct Camera {
    pub origin: Vertex,
    pub yaw: f64,   // rotation left/right
    pub pitch: f64, // rotation up/down
    pub fov: f64,
}

impl Camera {
    pub fn new(x: f64, y: f64, z: f64, fov: f64) -> Self {
        Self {
            origin: Vertex { x, y, z },
            yaw: 0.0,
            pitch: 0.0,
            fov,
        }
    }

    pub fn move_dir(&mut self, dx: f64, dy: f64, dz: f64) {
        self.origin.x += dx;
        self.origin.y += dy;
        self.origin.z += dz;
    }

    pub fn get_forward(&self) -> Vertex {
        Vertex::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
    }

    pub fn get_right(&self) -> Vertex {
        Vertex::new(self.yaw.sin(), 0.0, -self.yaw.cos())
    }

    pub fn get_up(&self) -> Vertex {
        Vertex::new(0.0, 1.0, 0.0)
    }
}
