# Lume Roadmap

## 🗺 High-Level Vision
A GPU-accelerated, agent-customizable GitHub PR client.

## 🚩 Current Milestone: The Luminous Foundation
**Target Version**: v0.1.1
**Status**: 🟡 In Progress

### Phase 1: Foundation
- [x] Setup WGPU Window
- [x] Theme $\to$ GPU Clear Color
- [x] Render "Lume" Brand Text
- [x] Setup Headless WGPU Test Harness
- [x] Verify DoD $\to$ Log to Wiki

### Phase 2: GitHub Bridge
- [ ] `gh` CLI wrapper
- [ ] JSON $\to$ Struct parsing
- [ ] Terminal data verification
- [ ] Implement Mock GitHub Response Tests

### Phase 3: Dashboard View
- [ ] GPU Vertical Layout
- [ ] PR Row components
- [ ] Minimalist spacing/padding

### Phase 4: Agent Vibe Check
- [ ] `modify_config` manual test
- [ ] Live hot-reload theme (lerping)

### Phase 5: Quality Assurance
- [ ] Implement Golden Image Regression Suite
- [ ] Setup CI pipeline (GitHub Actions) with headless rendering
- [ ] Property-based testing for config parsing