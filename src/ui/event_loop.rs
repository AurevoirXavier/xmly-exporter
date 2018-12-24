// --- std ---
use std::time::Instant;
// --- external ---
use conrod::backend::glium::glium::glutin;

pub struct EventLoop {
    last_update: Instant,
    ui_needs_update: bool,
}

impl EventLoop {
    pub fn new() -> EventLoop {
        EventLoop {
            last_update: Instant::now(),
            ui_needs_update: true,
        }
    }

    pub fn next(&mut self, events_loop: &mut glutin::EventsLoop) -> Vec<glutin::Event> {
        // --- std ---
        use std::{
            thread::sleep,
            time::Duration,
        };

        let last_update = self.last_update;
        let sixteen_ms = Duration::from_millis(16);
        let duration_since_last_update = Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms { sleep(sixteen_ms - duration_since_last_update); }

        let mut events = vec![];
        events_loop.poll_events(|event| events.push(event));
        if events.is_empty() && !self.ui_needs_update {
            events_loop.run_forever(|event| {
                events.push(event);
                glutin::ControlFlow::Break
            });
        }

        self.ui_needs_update = false;
        self.last_update = Instant::now();

        events
    }

    pub fn needs_update(&mut self) { self.ui_needs_update = true; }
}
