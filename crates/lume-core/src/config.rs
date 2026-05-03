use std::path::Path;

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

/// An sRGB hex color, deserialized from `"#rrggbb"` and stored in **linear**
/// space so it can be handed straight to `wgpu::Color` without further gamma
/// correction.
#[derive(Clone, Copy, Debug)]
pub struct HexColor {
    pub linear: [f64; 4],
}

impl HexColor {
    pub fn from_hex(s: &str) -> Result<Self> {
        let h = s.trim_start_matches('#');
        if h.len() != 6 {
            return Err(anyhow!("expected #rrggbb, got {s:?}"));
        }
        let r = u8::from_str_radix(&h[0..2], 16).context("invalid red byte")?;
        let g = u8::from_str_radix(&h[2..4], 16).context("invalid green byte")?;
        let b = u8::from_str_radix(&h[4..6], 16).context("invalid blue byte")?;
        Ok(Self {
            linear: [srgb_to_linear(r), srgb_to_linear(g), srgb_to_linear(b), 1.0],
        })
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Self::from_hex(&s).map_err(serde::de::Error::custom)
    }
}

fn srgb_to_linear(c: u8) -> f64 {
    let v = c as f64 / 255.0;
    if v <= 0.040_45 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Theme {
    pub background: HexColor,
    pub foreground: HexColor,
    pub accent: HexColor,
    pub glow_intensity: f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Behavior {
    pub default_repo: String,
    pub pr_fetch_limit: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub theme: Theme,
    pub behavior: Behavior,
}

impl AppConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(path)
            .with_context(|| format!("reading config from {}", path.display()))?;
        toml::from_str(&raw).context("parsing config TOML")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_round_trip_known_colors() {
        let bg = HexColor::from_hex("#1a1b26").unwrap();
        assert!((bg.linear[3] - 1.0).abs() < 1e-9);
        // 0x1a (26) → sRGB ~0.102 → linear ~0.01033
        assert!(
            (bg.linear[0] - 0.010_330).abs() < 1e-4,
            "r linear off: {}",
            bg.linear[0]
        );
    }

    #[test]
    fn rejects_short_or_long_hex() {
        assert!(HexColor::from_hex("#fff").is_err());
        assert!(HexColor::from_hex("#deadbeef").is_err());
    }

    #[test]
    fn rejects_non_hex_bytes() {
        assert!(HexColor::from_hex("#zzzzzz").is_err());
    }

    #[test]
    fn loads_workspace_config() {
        let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = std::path::PathBuf::from(manifest).join("../../config.toml");
        let cfg = AppConfig::load(&path).expect("config.toml at workspace root must parse");
        assert_eq!(cfg.behavior.pr_fetch_limit, 100);
        assert!((cfg.theme.glow_intensity - 0.5).abs() < 1e-6);
    }
}
