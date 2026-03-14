use crate::display::{WINDOW_HEIGHT, WINDOW_WIDTH};
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::dpi::LogicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

pub struct WindowState {
    pub window: Arc<Window>,
    surface: Surface<Arc<Window>, Arc<Window>>,
}

impl WindowState {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        let size = LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        let attrs = Window::default_attributes()
            .with_title("Chip-8 Emulator")
            .with_inner_size(size)
            .with_resizable(false);
        let window = Arc::new(event_loop.create_window(attrs).expect("create window"));
        let context = Context::new(Arc::clone(&window)).expect("create softbuffer context");
        let mut surface =
            Surface::new(&context, Arc::clone(&window)).expect("create softbuffer surface");

        let (w, h) = (
            NonZeroU32::new(WINDOW_WIDTH).unwrap(),
            NonZeroU32::new(WINDOW_HEIGHT).unwrap(),
        );
        surface.resize(w, h).expect("resize surface");

        Self { window, surface }
    }

    pub fn render(&mut self, chip8_buffer: &[bool; 2048]) {
        let mut frame = self.surface.buffer_mut().expect("get frame buffer");
        crate::display::render(&mut frame, chip8_buffer);
        frame.present().expect("present frame");
    }
}
