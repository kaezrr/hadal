mod engine;

use std::{num::NonZeroU32, sync::Arc};

use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::engine::Engine;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Failed to create surface: {0}")]
    CreateSurface(#[from] wgpu::CreateSurfaceError),

    #[error("Failed to request adapter: {0}")]
    RequestAdapter(#[from] wgpu::RequestAdapterError),

    #[error("Failed to request device: {0}")]
    RequestDevice(#[from] wgpu::RequestDeviceError),
}

type Result<T> = core::result::Result<T, Error>;

struct State {
    window: Arc<Window>,
    engine: Engine,
}

#[derive(Default)]
pub struct Application {
    state: Option<State>,
}

impl ApplicationHandler for Application {
    #[expect(
        clippy::expect_used,
        reason = "can't gracefully handle errors here, just crash"
    )]
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let width = unsafe { NonZeroU32::new_unchecked(960) };
        let height = unsafe { NonZeroU32::new_unchecked(640) };

        let window = {
            let window_attr = Window::default_attributes()
                .with_title("Hadal - PROJECT")
                .with_inner_size(LogicalSize::new(width.get(), height.get()));

            Arc::new(
                event_loop
                    .create_window(window_attr)
                    .expect("failed to initialize window"),
            )
        };

        log::info!("Window successfully created");

        let engine =
            Engine::new(window.clone(), width, height).expect("failed to initialize engine");

        log::info!("Engine successfully initialized");

        self.state = Some(State { window, engine });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let Some(ref state) = self.state else {
            log::warn!("window event before window initialized");
            return;
        };

        match event {
            WindowEvent::RedrawRequested => {
                state.engine.render_frame();
                state.window.request_redraw();
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if state.handle_keyboard_event(event) {
                    event_loop.exit();
                    log::info!("Exiting application... goodbye!");
                }
            }

            x => log::debug!("ignoring event: {x:?}"),
        }
    }
}

impl State {
    /// Handles keyboard input and returns whether application should quit
    fn handle_keyboard_event(&self, event: winit::event::KeyEvent) -> bool {
        let KeyEvent {
            physical_key: PhysicalKey::Code(key),
            state,
            ..
        } = event
        else {
            log::warn!("Ignoring unidentified key");
            return false;
        };

        key == KeyCode::Escape && state.is_pressed()
    }
}
