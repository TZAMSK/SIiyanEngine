pub trait Round {
    fn setRadius(&mut self);
    fn setSegments(&mut self);
}

pub struct RoundProperties {
    pub radius: f32,
    pub segments: usize,
}
