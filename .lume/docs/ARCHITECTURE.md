# Lume Architecture

## Data Flow
`GH CLI` $\to$ `Lume-Core` $\to$ `App State` $\to$ `Lume-GPU (WGPU)`

Async runtime: `futures::executor::block_on` for now; Tokio planned once `lume-core` actually performs concurrent I/O.

## Key Modules
1. **Core**: Handles the `std::process::Command` calls to `gh`, parses JSON, and manages the `AppConfig`.
2. **GPU**: A custom rendering pipeline.
   - Uses a **Virtual Canvas** for text.
   - Implements a **Color Interpolator** for smooth theme transitions.
3. **Agent**: An LLM bridge that translates Natural Language $\to$ TOML updates.

## Coding Standards
- State flows one way: input event → core → app state → GPU. No back-edges.
- Prefer composition over inheritance.
- GPU calls must not block the winit event loop.
- Use `serde` for all data transformation.
- No `unwrap`/`expect` on runtime paths (see `lume-reviewer` hard blocks).
