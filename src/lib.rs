use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

struct State {
    window: Window,
}

#[derive(Default)]
pub struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    #[expect(
        clippy::expect_used,
        reason = "can't gracefully handle errors here, just crash"
    )]
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = {
            let window_attr = Window::default_attributes()
                .with_title("Hadal - PROJECT")
                .with_inner_size(LogicalSize::new(960, 640));

            event_loop
                .create_window(window_attr)
                .expect("failed to initialize window")
        };

        log::info!("window successfully created.");
        self.state = Some(State { window });
    }

    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let Some(ref state) = self.state else {
            log::warn!("window event before window initialized");
            return;
        };

        match event {
            WindowEvent::RedrawRequested => state.window.request_redraw(),
            x => log::debug!("ignoring event: {x:?}"),
        }
    }
}
