pub mod shape;

use crate::scene::shape::Shape;

pub struct Scene {
    pub shapes: Vec<Shape>,
}
