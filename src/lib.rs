mod engine;
mod loader;
mod model;

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

pub use loader::load_gltf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to create surface: {0}")]
    CreateSurface(#[from] wgpu::CreateSurfaceError),

    #[error("Failed to request adapter: {0}")]
    RequestAdapter(#[from] wgpu::RequestAdapterError),

    #[error("Failed to request device: {0}")]
    RequestDevice(#[from] wgpu::RequestDeviceError),

    #[error("Error loading GLTF file {0}")]
    GltfLoader(#[from] gltf::Error),

    #[error("IO Error {0}")]
    FileIo(#[from] std::io::Error),
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
        let Some(ref mut state) = self.state else {
            log::warn!("window event before window initialized");
            return;
        };

        match event {
            WindowEvent::RedrawRequested => {
                state.engine.render_frame();
                state.window.request_redraw();
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if state.handle_key_should_quit(&event) {
                    event_loop.exit();
                    log::info!("Exiting application... goodbye!");
                }
            }

            WindowEvent::Resized(new_size) => {
                let Some(width) = NonZeroU32::new(new_size.width) else {
                    log::warn!("Cannot set window to zero width!");
                    return;
                };

                let Some(height) = NonZeroU32::new(new_size.height) else {
                    log::warn!("Cannot set window to zero width!");
                    return;
                };

                state.engine.resize_surface(width, height);
            }

            x => log::debug!("ignoring event: {x:?}"),
        }
    }
}

impl State {
    /// Handles keyboard input and returns whether application should quit
    #[expect(
        clippy::unused_self,
        reason = "engine key events are not yet implemented"
    )]
    fn handle_key_should_quit(&self, event: &winit::event::KeyEvent) -> bool {
        let KeyEvent {
            physical_key: PhysicalKey::Code(key),
            state,
            ..
        } = event
        else {
            log::warn!("Ignoring unidentified key");
            return false;
        };

        *key == KeyCode::Escape && state.is_pressed()
    }
}
