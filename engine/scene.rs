use super::*;
use macroquad::prelude::*;

/// A base store for elements
pub struct Scene {
    pub elements: Vec<Implementor>
}


impl Scene {
    pub fn new() -> Self where Self: Sized {
        Scene {
            elements: vec![]
        }
    }

    /// Registers a new element to the scene
    pub fn register(&mut self, e: Implementor) {
        self.elements.push(e);
    }
    
    
    /// Default frame progression - useful for scenes that do not require specialist logic
    pub fn frame_progression(&mut self, delta_t: f32) {
        for e in self.elements.iter() {
        
            // Inputs
            if let Some(implements) = &e.clickable {
                let mouse_position = get_virtual_mouse_pos();
                let mut hovered = false;
                let mut inner = implements.borrow_mut();
                
                for col_box in inner.collision_boxes() {
                    if is_cursor_over(&col_box, mouse_position) {
                        hovered = true;
                        break;
                    }
                }
                
                if is_mouse_button_pressed(MouseButton::Left) {
                    inner.on_mouse_held(MouseButton::Left);
                }
                if is_mouse_button_pressed(MouseButton::Middle) {
                    inner.on_mouse_held(MouseButton::Middle);
                }
                if is_mouse_button_pressed(MouseButton::Right) {
                    inner.on_mouse_held(MouseButton::Right);
                }
                
                if is_mouse_button_down(MouseButton::Left) {
                    inner.on_mouse_down(MouseButton::Left);
                }
                if is_mouse_button_down(MouseButton::Middle) {
                    inner.on_mouse_down(MouseButton::Middle);
                }
                if is_mouse_button_down(MouseButton::Right) {
                    inner.on_mouse_down(MouseButton::Right);
                }
                
                if is_mouse_button_released(MouseButton::Left) {
                    inner.on_mouse_up(MouseButton::Left);
                }
                if is_mouse_button_released(MouseButton::Middle) {
                    inner.on_mouse_up(MouseButton::Middle);
                }
                if is_mouse_button_released(MouseButton::Right) {
                    inner.on_mouse_up(MouseButton::Right);
                }
            }
            
            if let Some(implements) = &e.inputtable {
                let mut inner = implements.borrow_mut();
                inner.process_input();
            }
            
            if let Some(implements) = &e.hoverable {
                let mouse_position = get_virtual_mouse_pos();
                let mut hovered = false;
                let mut inner = implements.borrow_mut();
                
                for col_box in inner.collision_boxes() {
                    if is_cursor_over(&col_box, mouse_position) {
                        inner.on_hover(mouse_position);
                        hovered = true;
                        break;
                    }
                }
                
                if !hovered {
                    inner.on_not_hover(mouse_position);
                }
            }
            
            if let Some(implements) = &e.draggable {
                let mut inner = implements.borrow_mut();
                
                let mouse_delta = mouse_delta_position();
                
                if is_mouse_button_down(MouseButton::Left) {
                    inner.process_drag(MouseButton::Left, mouse_delta);
                } else if is_mouse_button_down(MouseButton::Right) {
                    inner.process_drag(MouseButton::Right, mouse_delta);
                } else if is_mouse_button_down(MouseButton::Middle) {
                    inner.process_drag(MouseButton::Middle, mouse_delta);
                }
            }
            
            // Collisions
           if let Some(implements) = &e.collideable {
                let mut inner = implements.borrow_mut();
                
                let boxes = &inner.collision_boxes()[0];
                let others = self.elements.iter().filter(|x| !std::ptr::eq(*x, e)); 
                let colliders = colliding(boxes, others);
                inner.on_collide(colliders);
                inner.draw_collision_boxes();
            }
            
            // Manual updating
            if let Some(implements) = &e.updateable {
                let mut inner = implements.borrow_mut();
                inner.update(delta_t);
            }
                       
                       
            // Drawing 
            if let Some(implements) = &e.displayable {
                let mut inner = implements.borrow_mut();
                inner.draw(delta_t);
                inner.static_draw();
            }
        }
    }
}


/// The main scene trait that defines standard hooks for user-made scenes
pub trait SceneTrait {
    fn on_enter(&mut self);
    fn on_frame(&mut self, delta_t: f32) -> Option<Box<dyn SceneTrait>>;
    fn on_exit(&mut self);
}

