# Lume: Project Manifest

## Vision
Lume is a hyper-minimalistic, GPU-accelerated GitHub PR review client. 
It is designed to feel like a "Luminous" instrument—fluid, fast, and precise.

## Core Philosophy
- **Minimalism**: No clutter. Only the essential information.
- **Luminous**: GPU-rendered with soft glows, smooth color lerping, and high-frame-rate animations.
- **Agentic**: The UI is a living organism that can be customized via natural language.
- **Performance**: Written in Rust to ensure zero-lag interactions.

## Tech Stack
- **Language**: Rust
- **Graphics**: WGPU + Winit
- **Text**: `glyph_brush` (Phase 1). Re-evaluate Vello once vector UI elements are needed.
- **Async**: `futures` today; Tokio when concurrent I/O lands in `lume-core`.
- **Config**: TOML (with hot-reloading)
- **Data**: GitHub CLI (`gh`) wrapper for auth/API.
