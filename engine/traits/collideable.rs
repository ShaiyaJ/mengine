use macroquad::prelude::*;
use super::Implementor;

/// Checks over a list of `targets` whether the `lhs` Rect is colliding with *any* of the target's collision boxes - and returns the targets that are colliding
pub fn colliding<'a, I>(lhs: &'a Rect, targets: I) -> Vec<&'a Implementor> 
where
    I: Iterator<Item = &'a Implementor>
{
    targets
        .filter(|e| {
            if let Some(collideable) = &e.collideable {     // Check if the Implementor implements the collideable behaviour
                let inner = collideable.borrow();
                inner.collision_boxes().iter()              // If it does, check if any of the collision boxes overlap
                    .any(|rhs| lhs.overlaps(rhs)) 
            } else {                                        // Otherwise it cannot collide
                false
            }
        })
        .collect()
}

/// Behaviour for defining collision boxes and responding to collisions
pub trait Collideable {
    fn collision_boxes(&self) -> Vec<Rect>;

    fn on_collide(&mut self, colliders: Vec<&Implementor>);

    fn draw_collision_boxes(&self) {
        let mut c = 1.;     // Used for various modulo operators to make each box have a different colour

        for collision_box in self.collision_boxes().iter() {
            draw_rectangle(collision_box.x, collision_box.y, collision_box.w, collision_box.h, Color::new(1. / (c % 2.), 1. / (c % 3.), 1. / (c+1. % 6.), 0.25));

            c += 1.;
        }
    }
}
