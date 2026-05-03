use anyhow::{Context, Result};
use wgpu::util::StagingBelt;
use wgpu_glyph::{ab_glyph::FontArc, GlyphBrush, GlyphBrushBuilder, Section, Text};

pub struct TextRenderer {
    glyph_brush: GlyphBrush<()>,
    staging_belt: StagingBelt,
    brand_color: [f32; 4],
}

impl TextRenderer {
    pub fn new(
        device: &wgpu::Device,
        render_format: wgpu::TextureFormat,
        brand_color: [f32; 4],
    ) -> Result<Self> {
        let font = FontArc::try_from_slice(include_bytes!("../assets/FiraCode-Regular.ttf"))
            .context("loading embedded FiraCode font")?;
        let glyph_brush = GlyphBrushBuilder::using_font(font).build(device, render_format);
        Ok(Self {
            glyph_brush,
            staging_belt: StagingBelt::new(1024),
            brand_color,
        })
    }

    pub fn queue_brand(&mut self) {
        self.glyph_brush.queue(
            Section::default()
                .with_screen_position((24.0, 24.0))
                .add_text(
                    Text::new("Lume")
                        .with_color(self.brand_color)
                        .with_scale(48.0),
                ),
        );
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> Result<()> {
        // `staging_belt.finish()` must run regardless of the draw outcome,
        // otherwise a partial allocation can deadlock the next frame.
        let draw_result = self.glyph_brush.draw_queued(
            device,
            &mut self.staging_belt,
            encoder,
            view,
            size.width,
            size.height,
        );
        self.staging_belt.finish();
        draw_result.map_err(|e| anyhow::anyhow!("glyph_brush draw failed: {e}"))
    }

    pub fn recall(&mut self) {
        self.staging_belt.recall();
    }
}
