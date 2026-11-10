use std::num::NonZeroU32;

pub struct GpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
}

impl GpuContext {
    pub async fn new<T>(display: T, width: NonZeroU32, height: NonZeroU32) -> crate::Result<Self>
    where
        T: wgpu::wgt::WgpuHasDisplayHandle + Into<wgpu::SurfaceTarget<'static>> + Clone,
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

    pub fn configure_surface(&self) {
        self.surface.configure(&self.device, &self.config);
    }

    pub fn configure_surface_with_size(&mut self, width: NonZeroU32, height: NonZeroU32) {
        self.config.width = width.get();
        self.config.height = height.get();
        self.configure_surface();
    }

    /// Returns the current surface texture and if it needs reconfiguration
    pub fn acquire_surface(&self) -> Option<(wgpu::SurfaceTexture, bool)> {
        match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => Some((texture, false)),

            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                log::warn!("Surface texture is suboptimal");
                Some((texture, true))
            }

            wgpu::CurrentSurfaceTexture::Outdated => {
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
