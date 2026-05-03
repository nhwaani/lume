---
description: Verifies the stability of the Lume project — runs the workspace test gates and the phase-specific render checks consumed by lume-manager.
---

# Lume Tester Skill (Foundation Phase)

This skill ensures the stability of the Lume project. For Phase 1, it focuses on basic GPU initialization and windowing.

## Pass Contract

A run is **PASS** only if all of these are clean:
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets`
- `cargo test --workspace`
- The phase-specific `verify-*` checks below.

Anything else is **FAIL**. Report which gate failed; do not summarize as "mostly passing."

## Usage

### 1. Verify Window Initialization
```
/skill:lume-tester verify-init
```
- WGPU instance + surface created without panic.
- Window is visible and matches the configured size.

### 2. Verify Render Loop
```
/skill:lume-tester verify-render
```
- Render loop runs and submits at least one frame.
- Runs through the headless WGPU harness in `crates/lume-gpu/tests/` so it works in CI.

## Standards
- **Unit tests**: native Rust `#[test]`.
- **Visuals**: Phase 1 allows manual verification of clear color and brand text; Phase 5 replaces this with golden-image regression.
- **Flakes**: a check that fails intermittently is FAIL — fix the root cause, don't retry.

## Integration
A phase is verified when this skill returns PASS. `lume-manager complete` consumes that verdict directly.
