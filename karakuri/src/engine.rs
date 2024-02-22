use self::{
    sdl_fps_controller::SdlFpsController, sdl_input_processor::SdlInputProcessor,
    sdl_renderer::SdlRenderer,
};
use crate::{
    logger,
    toy::Toy, // TODO: delete this
    utils::{Color, Resolution},
};
use sdl2::Sdl;

mod sdl_fps_controller;
mod sdl_input_processor;
mod sdl_renderer;

pub struct Engine {
    renderer: SdlRenderer,
    fps_controller: SdlFpsController,
    input_processor: SdlInputProcessor,
    toys: Vec<Toy>, // TODO: delete this
}

impl Engine {
    pub fn new(
        title: Option<&str>,
        resolution: Option<Resolution>,
        clear_color: Option<Color>,
        target_fps: Option<u32>,
        min_update_fps: Option<u32>,
    ) -> Self {
        let title = title.unwrap_or("Sandbox");

        let sdl = Self::init_sdl();

        Self {
            renderer: SdlRenderer::new(&sdl, title, resolution, clear_color),
            fps_controller: SdlFpsController::new(
                sdl.timer().unwrap_or_else(|e| {
                    logger::log_fatal(format!("Failed to get SDL2 timer: {}", e).as_str())
                }),
                target_fps,
                min_update_fps,
            ),
            input_processor: SdlInputProcessor::new(sdl.event_pump().unwrap_or_else(|e| {
                logger::log_fatal(format!("Failed to get SDL2 event pump: {}", e).as_str())
            })),
            toys: Vec::new(), // TODO: Delete this
        }
    }

    pub fn run(&mut self) {
        loop {
            let delta_time = self.fps_controller.cap_framerate();

            let result = self.input_processor.process();

            if result.should_quit {
                break;
            }

            self.update(delta_time);

            self.renderer.start_frame();
            self.draw();
            self.renderer.finish_frame();
        }
    }

    // TODO: Delete this
    pub fn add_toy(&mut self, toy: Toy) {
        self.toys.push(toy);
    }

    fn update(&mut self, delta_time: f64) {
        // TODO: Delete this
        for toy in &mut self.toys {
            toy.update(delta_time);
        }
    }

    fn draw(&mut self) {
        // TODO: Delete this
        for toy in &self.toys {
            self.renderer.render(&toy.position, &toy.size, &toy.color);
        }
    }

    fn init_sdl() -> Sdl {
        sdl2::init().unwrap_or_else(|e| {
            logger::log_fatal(format!("Failed to initialize SDL2: {}", e).as_str())
        })
    }
}
