use macroquad::prelude::*;
use std::sync::OnceLock;

use super::SceneTrait;

static VIRTUAL_WIDTH: OnceLock<f32> = OnceLock::new();      /// The virtual "canvas" width
static VIRTUAL_HEIGHT: OnceLock<f32> = OnceLock::new();     /// The virtual "canvas" height


/// A single-use function to set the virtual "canvas" size, using this twice will cause an error
pub fn set_engine_virtual_resolution(width: f32, height: f32) {
    VIRTUAL_WIDTH.set(width).ok().expect("Virtual width already set");
    VIRTUAL_HEIGHT.set(height).ok().expect("Virtual height already set");
}


/// Getter method for the virtual "canvas" size
pub fn get_engine_virtual_resolution() -> Vec2 {
    vec2(
        *VIRTUAL_WIDTH.get().expect("Virtual width not set"),
        *VIRTUAL_HEIGHT.get().expect("Virtual height not set")
    )
}


/// Macroquad configuration defaults - custom user-ones may be provided for specific purposes
pub fn set_engine_defaults() {
    // Configuring macroquad
    set_default_filter_mode(FilterMode::Nearest);

}


/// Retrive the mouse position scaled to the bounds of the virtual "canvas"
pub fn get_virtual_mouse_pos() -> Vec2 {
    let virtual_res = get_engine_virtual_resolution();
    
    let scale: f32 = f32::min(
        screen_width() / virtual_res.x,
        screen_height() / virtual_res.y
    );
    
    Vec2 {
        x: (mouse_position().0 - (screen_width() - (virtual_res.x * scale)) * 0.5) / scale,
        y: (mouse_position().1 - (screen_height() - (virtual_res.y * scale)) * 0.5) / scale,
    }
}


/// Starts the rendering mainloop for the engine - this is a blocking method
pub async fn start_engine_mainloop(entry_scene: Box<dyn SceneTrait>) {
    let virtual_res = get_engine_virtual_resolution();

    // Setting up virtual rendering target
    let target = render_target(virtual_res.x as u32, virtual_res.y as u32);
    target.texture.set_filter(FilterMode::Nearest);


    // Setting up camera
    let mut camera = Camera2D::from_display_rect(Rect::new(
        0., virtual_res.y,
        virtual_res.x, -virtual_res.y
    ));
    camera.render_target = Some(target.clone());


    // Setting up scene
    let mut current_scene: Box<dyn SceneTrait> = entry_scene;
    current_scene.on_enter();


    // Main render and logic loop
    loop {
        // Setting up target texture (virtual resolution)
        set_camera(&camera);
        clear_background(BLACK);


        // Running active scene logic and scene switching logic
        if let Some(new_scene) = current_scene.on_frame(get_frame_time()) {
            current_scene.on_exit();
            current_scene = new_scene;
            current_scene.on_enter();
        }


        // Debug draw
        draw_text(get_fps().to_string().as_str(), 0., 10., 20., WHITE);


        // Calculating required scale to upscale (or downscale) target texture to the screen
        let scale: f32 = f32::min(
            screen_width() / virtual_res.x,
            screen_height() / virtual_res.y
        );


        // Drawing target texture to main screen
        set_default_camera();
        clear_background(BLACK);

        draw_texture_ex(
            &target.texture,
            (screen_width() - (virtual_res.x * scale)) * 0.5,
            (screen_height() - (virtual_res.y * scale)) * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(virtual_res.x * scale, virtual_res.y * scale)),
                flip_y: false,
                ..Default::default()
            }
        );


        // Frame finish
        next_frame().await
    }
}
