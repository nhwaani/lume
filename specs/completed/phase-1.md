# Phase 1: Foundation

## Goal
Implement the basic GPU initialization and windowing for the Lume project.

## Requirements
- Setup WGPU Window
- Theme → GPU Clear Color
- Render "Lume" Brand Text
- Setup Headless WGPU Test Harness
- Verify DoD → Log to Wiki

## Implementation Details
- Use wgpu 0.19.4 for GPU rendering
- Use winit 0.29 for window management
- Use glyph_brush 0.2.4 for text rendering
- Use ab_glyph 0.2.0 for font handling
- Use raw-window-handle 0.6.2 for window handle management
- Use futures 0.3 for async/await support

## Acceptance Criteria
- `cargo fmt --check` passes
- `cargo clippy --workspace --all-targets -- -D warnings` passes
- `cargo build --workspace --all-targets` passes
- `cargo test --workspace` passes
- Window is visible and matches the configured size (800x600)
- Clear color is set to (0.1, 0.1, 0.15, 1.0)
- "Lume" text is rendered in the top-left corner
- Headless WGPU test passes

## Dev Notes
- The window must be declared after the surface in the Renderer struct to ensure proper lifetime management
- The clear color should be a dark background to make the text visible
- The text should be rendered using FiraCode font at 24pt size
- The headless test should verify that WGPU can initialize without a window
- The `verify-init` check in lume-tester should pass
- The `verify-render` check in lume-tester should pass

## Links
- [wgpu Documentation](https://wgpu.rs/)
- [winit Documentation](https://docs.rs/winit/)
- [glyph_brush Documentation](https://docs.rs/glyph_brush/)
- [ab_glyph Documentation](https://docs.rs/ab_glyph/)
- [raw-window-handle Documentation](https://docs.rs/raw-window-handle/)