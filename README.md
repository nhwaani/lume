# Lume

A GPU-accelerated, agent-customizable GitHub PR review client written in Rust.

Lume is designed to feel like a luminous instrument — fluid, fast, precise. The UI is rendered directly with WGPU; the configuration is a TOML file that an agent can hot-reload at runtime; data comes from the GitHub CLI.

**Status:** v0.1.1 — Phase 1: Foundation shipped. See [ROADMAP.md](ROADMAP.md) for what's next and [CHANGELOG.md](CHANGELOG.md) for what's landed.

---

## Why

Most PR clients are HTML chrome over a REST API. Lume is the opposite — every glyph is rasterized to a texture, every theme transition is a `Color` lerp on the GPU, and every UI rule is a TOML field that you (or an LLM agent) can rewrite without rebuilding.

Three commitments:

- **Minimalism** — only the essential information; no clutter.
- **Performance** — Rust + WGPU; zero-lag interactions are non-negotiable.
- **Agentic** — the UI is a living organism. Natural-language requests translate into config edits via the agent bridge.

## Tech Stack

| Layer        | Choice                                 |
| ------------ | -------------------------------------- |
| Language     | Rust (edition 2021)                    |
| Graphics     | [`wgpu`](https://crates.io/crates/wgpu) 23 + [`winit`](https://crates.io/crates/winit) 0.29 |
| Text         | [`wgpu_glyph`](https://crates.io/crates/wgpu_glyph) 0.23 |
| Async        | `futures::executor::block_on` (Tokio adopted when `lume-core` needs concurrent I/O) |
| Config       | TOML via `serde` + `toml`              |
| Data source  | GitHub CLI (`gh`) wrapper (Phase 2)    |

## Quick Start

### Prerequisites

- Rust toolchain (`rustup` recommended; edition 2021)
- macOS or Linux. Windows untested but DX12 backend is wired in.
- [`gh`](https://cli.github.com) — only required from Phase 2 onward.

### Build & Run

```bash
git clone git@github.com:nhwaani/lume.git
cd lume
cargo run -p lume-cli
```

You should see an 800×600 window titled "Lume" with a dark background and the brand text rendered in the accent color from `config.toml`.

### Run the Test Suite

The `lume-tester` skill defines the PASS contract:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets
cargo test --workspace
```

All four must be green before any phase is considered done.

## Configuration

Lume reads `config.toml` at the workspace root on startup. Hex colors are stored as sRGB strings and converted to linear space before reaching `wgpu::Color` — colors look the way they're written.

```toml
[theme]
background     = "#1a1b26"   # sRGB hex
foreground     = "#a9b1d6"
accent         = "#7aa2f7"
glow_intensity = 0.5         # f32, 0.0..=1.0

[behavior]
default_repo   = "owner/repo"
pr_fetch_limit = 100
```

Hot-reloading is wired (`set_clear_color`) but not yet driven by anything live; the agent-driven hot-reload arrives alongside the agent bridge.

## Project Layout

```
lume/
├── crates/
│   ├── lume-cli/         # binary entry point; owns the winit event loop
│   ├── lume-core/        # config, github bridge, agent tools
│   └── lume-gpu/         # WGPU renderer + text rendering
├── .lume/docs/           # project manifest, architecture, schemas, agent prompt
├── .pi/skills/           # agent skills (manager, tester, reviewer)
├── .wiki/                # phase completion logs
├── specs/                # active and completed phase specs
├── config.toml           # runtime configuration
├── ROADMAP.md            # milestone & phase tracking
└── CHANGELOG.md          # release history
```

The data flow: `gh CLI → lume-core → app state → lume-gpu (WGPU)`. State flows one way; no back-edges.

## Development Workflow

Three skills coordinate the SDLC. Invoke them from a Claude Code session running in the repo:

- **`/skill:lume-manager`** — orchestrator. `status`, `help`, `complete <phase>`. Refuses to mark a phase done unless the tester PASSes and the reviewer LGTMs.
- **`/skill:lume-tester`** — runs the PASS contract above plus phase-specific `verify-init` / `verify-render` checks.
- **`/skill:lume-reviewer`** — three-dimension code review (Craft / Technical Rigor / Maintainability) with a hard-block list. Returns `LGTM` or `Request Changes`.

All three live in `.pi/skills/` and are project-scoped — they ship with the repo and stay in sync with the codebase.

## Roadmap

Five phases in *The Luminous Foundation* milestone:

1. **Foundation** — WGPU window, theme→clear color, brand text, headless harness. *(Shipped in v0.1.1.)*
2. **GitHub Bridge** — `gh` wrapper, JSON parsing, mock response tests.
3. **Dashboard View** — vertical layout, PR row components.
4. **Agent Vibe Check** — `modify_config` tool, hot-reload theme via lerp.
5. **Quality Assurance** — golden image regression suite, CI with headless rendering.

See [ROADMAP.md](ROADMAP.md) for the detailed checklist.

## Contributing

Lume is in active development and the API is unstable, but contributions are welcome — issues and pull requests both. The non-negotiable gates apply equally to maintainer and outside PRs:

- `lume-tester` PASS (formatting, clippy, build, tests, verify-* checks)
- `lume-reviewer` LGTM
- Diff matches the active spec's Definition of Done; no scope creep

For larger changes, open an issue first so the work fits the current phase rather than colliding with it.

## License

Dual-licensed under either of, at your option:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.
