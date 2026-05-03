# Lume Tester Skill (Foundation Phase)

This skill ensures the stability of the Lume project. For Phase 1, it focuses on basic GPU initialization and windowing.

## Usage

### 1. Verify Window Initialization
```
/skill:lume-tester verify-init
```
- Verifies that the WGPU instance and surface are created without panic.
- Checks that the window is visible and has the correct dimensions.

### 2. Verify Render Loop
```
/skill:lume-tester verify-render
```
- Ensures the render loop is running.
- Checks that a frame is successfully submitted to the GPU.

## Testing Standards
- **Basic Checks**: Use native Rust `#[test]` to verify logic.
- **Visuals**: Manual verification of the clear color and text during Phase 1.

## Integration
A phase is considered verified when `lume-tester` confirms basic initialization and rendering success.
