use macroquad::prelude::*;

/// Behaviour for responding to mouse drags
pub trait Draggable {
    fn process_drag(&mut self, button: MouseButton, mouse_delta: Vec2);
}
