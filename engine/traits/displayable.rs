use macroquad::prelude::*;

// TODO: could use impl for to make Texture2D and Images "separate"
/// Accepted image types for a frame - eventually these are converted into Texture2Ds
pub enum Animateables<'a> {
    FrameByteSlice(&'a [u8]),
    FrameTexture2D(Texture2D),
    FrameImage(Image)
}

/// A set of frame data and configurations for the animation
pub struct Animation {
    frames: Vec<Texture2D>,
    repeating: bool,
    frame_timer: f32,
    frame_duration: f32,
    current_frame: usize
}


impl Animation {
    pub fn from(frames: Vec<Animateables>, frame_rate: f32, repeating: bool) -> Result<Self, Box<dyn std::error::Error>> {
        // Checking if the frame has any data to avoid access errors
        if frames.len() <= 0 {
            return Err(Box::from("Frames must have at least length 1"));
        }

        // Otherwise return an animation
        Ok(Self {
            frames: frames.iter().map(|e|   // Collect frames into a Texture2D for consistency 
                match e {
                    Animateables::FrameByteSlice(data) => Texture2D::from_file_with_format(data, None),
                    Animateables::FrameTexture2D(data) => data.clone(),
                    Animateables::FrameImage(data) => Texture2D::from_image(&data)
                }
            ).collect(),
            repeating: repeating,
            frame_timer: 1./frame_rate,
            frame_duration: 1./frame_rate,
            current_frame: 0
        })
    }


    /// Returns the current frame that should play (with respect to the frame timers) and increments the timers that control when the "next frame" should play
    pub fn get_frame(&mut self, delta_t: f32) -> &Texture2D {
        self.frame_timer -= delta_t;                            // Update frame timer
        
        if self.frame_timer <= 0. {                             // If frame timer is 0, update current_frame so the fetched frame is correct with respect to given frame rate
            self.frame_timer += self.frame_duration;            // this is so the game can have a mismatch between "logic framerate" and actual "animation framerate" 
            self.current_frame += 1;

            if self.current_frame >= self.frames.len() {
                if self.repeating {
                    self.current_frame = 0;
                } else {
                    self.current_frame = self.frames.len() - 1; // Stay on last frame
                }
            }
        }
        
        &self.frames[self.current_frame]
    }
    
    
    /// Returns the index of the current frame that should play 
    pub fn get_current_frame(&self) -> usize {
        self.current_frame
    }
}


/// Behaviour for drawing to the screen
pub trait Displayable {
    /// `static_draw` is for draw calls that do not require mutable access (usually static images or manually controlled animations) -- by default this is an empty implementation
    fn static_draw(&self) {}
    
    /// `draw` is for draw calls that require mutable access (e.g. when using engine's `Animation`)  -- by default this is an empty implementation
    fn draw(&mut self, delta_t: f32) {}
}
