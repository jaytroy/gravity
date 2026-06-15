use crate::triangle::{create_triangle_vertices, Triangle};
use crate::transform::Transform;

pub struct Entity {
    pub triangle: Triangle,
    pub transform: Transform,
}

impl Entity {
    pub fn new(triangle: Triangle, transform: Transform) -> Entity {
        Entity {
            triangle,
            transform,
        }
    }

    pub fn create_vertices(&mut self) -> Vec<f32> {
        create_triangle_vertices(&self.triangle, &self.transform.offset_x, &self.transform.offset_y)
    }

    pub fn rotate(&mut self, i: f32) {
        self.triangle.v1 += i * self.transform.rotation;
        self.triangle.v2 += i * self.transform.rotation;
        self.triangle.v3 += i * self.transform.rotation;
    }
}