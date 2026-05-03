use std::path::Path;

use anyhow::{Context, Result};
use futures::executor::block_on;
use lume_core::{AppConfig, HexColor};
use lume_gpu::{init, LumeGpu};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn clear_color_from(bg: HexColor) -> wgpu::Color {
    wgpu::Color {
        r: bg.linear[0],
        g: bg.linear[1],
        b: bg.linear[2],
        a: bg.linear[3],
    }
}

fn brand_color_from(fg: HexColor) -> [f32; 4] {
    [
        fg.linear[0] as f32,
        fg.linear[1] as f32,
        fg.linear[2] as f32,
        fg.linear[3] as f32,
    ]
}

fn main() -> Result<()> {
    println!("Starting Lume...");
    lume_core::init();
    init();

    let config = AppConfig::load(Path::new("config.toml")).context("loading config.toml")?;
    let clear_color = clear_color_from(config.theme.background);
    let brand_color = brand_color_from(config.theme.accent);

    let event_loop = EventLoop::new().context("creating event loop")?;
    let window = WindowBuilder::new()
        .with_title("Lume")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .context("building window")?;

    let mut lume_gpu =
        block_on(LumeGpu::new(window, clear_color, brand_color)).context("initializing GPU")?;

    event_loop
        .run(move |event, target| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => lume_gpu.resize(size),
                WindowEvent::CloseRequested => target.exit(),
                WindowEvent::RedrawRequested => match lume_gpu.render() {
                    Ok(()) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        lume_gpu.reconfigure();
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => target.exit(),
                    Err(e) => eprintln!("frame error: {e:?}"),
                },
                _ => {}
            },
            Event::AboutToWait => lume_gpu.request_redraw(),
            _ => {}
        })
        .context("event loop exited with error")
}
