use std::sync::Arc;

use hadal::GpuContext;
use hadal::HadalRenderer;
use winit::dpi::PhysicalSize;
use winit::event::MouseScrollDelta;
use winit::keyboard::KeyCode;
use winit::window::CursorGrabMode;
use winit::window::Window;

#[derive(Debug)]
pub struct State {
    window: Arc<Window>,
    renderer: HadalRenderer<'static>,
    cursor_grabbed: bool,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let gpu_context = {
            let PhysicalSize { width, height } = window.inner_size();
            GpuContext::new(window.clone(), width, height).await?
        };

        let renderer = HadalRenderer::new(gpu_context)?;

        Ok(Self {
            window,
            renderer,
            cursor_grabbed: false,
        })
    }

    pub fn render(&self) -> anyhow::Result<()> {
        todo!()
    }

    pub fn update(&mut self, dt: core::time::Duration) {
        self.renderer.update(dt);
    }

    pub fn process_keyboard(&mut self, _key: KeyCode, _is_pressed: bool) {
        todo!()
    }

    pub fn process_mouse_delta(&mut self, _dx: f64, _dy: f64) {
        if !self.cursor_grabbed {
            return;
        }
        todo!()
    }

    pub fn process_mouse_scroll(&mut self, _delta: &MouseScrollDelta) {
        if !self.cursor_grabbed {
            return;
        }
        todo!()
    }

    pub fn resize_surface(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.renderer.resize_surface(width, height);
    }

    pub fn capture_mouse(&mut self) -> anyhow::Result<()> {
        self.window.set_cursor_grab(CursorGrabMode::Locked)?;
        self.window.set_cursor_visible(false);
        self.cursor_grabbed = true;
        Ok(())
    }

    pub fn release_mouse(&mut self) -> anyhow::Result<()> {
        self.window.set_cursor_grab(CursorGrabMode::None)?;
        self.window.set_cursor_visible(true);
        self.cursor_grabbed = false;
        Ok(())
    }
}
