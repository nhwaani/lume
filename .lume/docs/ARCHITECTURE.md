# Lume Architecture

## Data Flow
`GH CLI` $\to$ `Lume-Core (Tokio)` $\to$ `App State` $\to$ `Lume-GPU (WGPU)`

## Key Modules
1. **Core**: Handles the `std::process::Command` calls to `gh`, parses JSON, and manages the `AppConfig`.
2. **GPU**: A custom rendering pipeline. 
   - Uses a **Virtual Canvas** for text.
   - Implements a **Color Interpolator** for smooth theme transitions.
3. **Agent**: An LLM bridge that translates Natural Language $\to$ TOML updates.

## Coding Standards
- Use **Functional-Reactive** patterns for state updates.
- Prefer **Composition** over inheritance.
- All GPU calls must be non-blocking to the main event loop.
- Use `serde` for all data transformation.
