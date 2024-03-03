use self::{
    sdl_fps_controller::SdlFpsController, sdl_input_processor::SdlInputProcessor,
    sdl_renderer::SdlRenderer,
};
use crate::{
    logger,
    math::Vector2,
    scene::Scene,
    utils::{Color, Resolution},
};
use sdl2::Sdl;

mod sdl_fps_controller;
mod sdl_input_processor;
mod sdl_renderer;

#[derive(Copy, Clone, Default)]
pub struct RenderData {
    pub position: Vector2,
    pub size: Vector2,
    pub color: Color,
}

pub struct Engine {
    scene: Scene,
    renderer: SdlRenderer,
    fps_controller: SdlFpsController,
    input_processor: SdlInputProcessor,
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
            scene: Scene::default(),
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
        }
    }

    pub fn scene(&mut self) -> &mut Scene {
        &mut self.scene
    }

    pub fn run(&mut self) {
        loop {
            let delta_time = self.fps_controller.cap_framerate();

            let result = self.input_processor.process();

            if result.should_quit {
                break;
            }

            self.renderer.start_frame();
            for entity in self.scene.entities() {
                entity.on_step(delta_time);

                let RenderData {
                    position,
                    size,
                    color,
                } = entity.render_data();
                self.renderer.render(position, size, color);
            }
            self.renderer.finish_frame();
        }
    }

    fn init_sdl() -> Sdl {
        sdl2::init().unwrap_or_else(|e| {
            logger::log_fatal(format!("Failed to initialize SDL2: {}", e).as_str())
        })
    }
}
