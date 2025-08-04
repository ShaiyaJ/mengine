/// General behaviour for per-frame mutation with respect to delta time
pub trait Updateable {
    fn update(&mut self, delta_t: f32);
}

