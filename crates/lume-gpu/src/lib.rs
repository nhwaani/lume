pub mod renderer;
pub mod text;

use anyhow::Result;

pub struct LumeGpu {
    renderer: renderer::Renderer,
    text_renderer: text::TextRenderer,
}

impl LumeGpu {
    pub async fn new(
        window: winit::window::Window,
        clear_color: wgpu::Color,
        brand_color: [f32; 4],
    ) -> Result<Self> {
        let renderer = renderer::Renderer::new(window, clear_color).await?;
        let text_renderer =
            text::TextRenderer::new(&renderer.device, renderer.config.format, brand_color)?;
        Ok(Self {
            renderer,
            text_renderer,
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(new_size);
    }

    pub fn reconfigure(&self) {
        self.renderer.reconfigure();
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.renderer.size
    }

    pub fn request_redraw(&self) {
        self.renderer.window.request_redraw();
    }

    pub fn set_clear_color(&mut self, color: wgpu::Color) {
        self.renderer.set_clear_color(color);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.renderer.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.renderer
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Frame Encoder"),
                });

        self.renderer.record_clear(&mut encoder, &view);

        self.text_renderer.queue_brand();
        // Text is non-fatal — log and keep the frame going.
        if let Err(e) = self.text_renderer.draw(
            &self.renderer.device,
            &mut encoder,
            &view,
            self.renderer.size,
        ) {
            eprintln!("text render error: {e:#}");
        }

        self.renderer
            .queue
            .submit(std::iter::once(encoder.finish()));
        output.present();
        self.text_renderer.recall();
        Ok(())
    }
}

pub fn init() {
    println!("Lume GPU Initialized");
}
