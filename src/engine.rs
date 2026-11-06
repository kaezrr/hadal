use crate::Result;
use std::num::NonZeroU32;
use wgpu::{CurrentSurfaceTexture, wgt::WgpuHasDisplayHandle};

pub struct Engine {
    gpu: GpuContext,
}

impl Engine {
    pub fn new<T>(display: T, width: NonZeroU32, height: NonZeroU32) -> Result<Self>
    where
        T: WgpuHasDisplayHandle + Into<wgpu::SurfaceTarget<'static>> + Clone,
    {
        Ok(Self {
            gpu: pollster::block_on(GpuContext::new(display, width, height))?,
        })
    }

    pub fn render_frame(&self) {
        let Some(surface_texture) = self.gpu.get_surface_texture() else {
            log::warn!("Could not obtain surface texture, skipping frame...");
            return;
        };

        let command_encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let command_buffer = command_encoder.finish();

        self.gpu.queue.submit(std::iter::once(command_buffer));
        self.gpu.queue.present(surface_texture);
    }
}

struct GpuContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
}

impl GpuContext {
    async fn new<T>(display: T, width: NonZeroU32, height: NonZeroU32) -> Result<Self>
    where
        T: WgpuHasDisplayHandle + Into<wgpu::SurfaceTarget<'static>> + Clone,
    {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_with_display_handle(
            Box::new(display.clone()),
        ));

        let surface = instance.create_surface(display)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await?;

        log::info!("Using GPU device: {}", adapter.get_info().name);

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("device_queue_request"),
                ..Default::default()
            })
            .await?;

        let capabilities = surface.get_capabilities(&adapter);

        let format = capabilities
            .formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .unwrap_or(capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            color_space: wgpu::SurfaceColorSpace::Auto,
            width: width.get(),
            height: height.get(),
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        Ok(Self {
            device,
            queue,
            surface,
            config,
        })
    }

    fn configure_surface(&self) {
        self.surface.configure(&self.device, &self.config);
    }

    fn configure_surface_with_size(&mut self, width: NonZeroU32, height: NonZeroU32) {
        self.config.width = width.get();
        self.config.height = height.get();
        self.configure_surface();
    }

    fn get_surface_texture(&self) -> Option<wgpu::SurfaceTexture> {
        match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(texture) => Some(texture),

            CurrentSurfaceTexture::Suboptimal(texture) => {
                log::warn!("Surface texture is suboptimal");
                Some(texture)
            }

            CurrentSurfaceTexture::Outdated => {
                log::warn!("Surface texture is outdated");
                self.configure_surface();
                None
            }

            err => {
                log::warn!("Surface texture error: {err:?}");
                None
            }
        }
    }
}
