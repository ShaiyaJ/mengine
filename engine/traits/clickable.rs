use macroquad::prelude::*;
use super::*;

/// Behaviour for responding to mouse clicks
pub trait Clickable: Collideable {
    fn on_mouse_down(&mut self, button: MouseButton);
    fn on_mouse_up(&mut self, button: MouseButton);
    fn on_mouse_held(&mut self, button: MouseButton);
}
