use macroquad::prelude::{Vec2, Rect};
use super::Collideable;


/// A check for whether a point position is within a Rect *Alias for `Rect.contains`*
pub fn is_cursor_over(target: &Rect, cursor_pos: Vec2) -> bool {
    target.contains(cursor_pos)
}


/// A check for whether a cursor is over any collision box of a `Collideable`
pub fn is_cursor_over_any<T: Collideable>(target: &T, cursor_pos: Vec2) -> bool {
    target.collision_boxes().iter().any(|b| b.contains(cursor_pos))
}


/// Behaviour for responding to mouse hovers
pub trait Hoverable: Collideable {
    fn on_hover(&mut self, mouse_position: Vec2);
    fn on_not_hover(&mut self, mouse_position: Vec2);
}
