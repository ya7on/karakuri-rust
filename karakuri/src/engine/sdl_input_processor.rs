use sdl2::{event::Event, keyboard::Keycode, EventPump};

pub struct SdlInputProcessor {
    event_pump: EventPump,
}

pub struct InputProcessingResult {
    pub should_quit: bool,
}

impl SdlInputProcessor {
    pub fn new(event_pump: EventPump) -> Self {
        Self { event_pump }
    }

    pub fn process(&mut self) -> InputProcessingResult {
        #[allow(clippy::never_loop)]
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return InputProcessingResult { should_quit: true },
                _ => return InputProcessingResult { should_quit: false },
            }
        }

        InputProcessingResult { should_quit: false }
    }
}
