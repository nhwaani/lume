# Lume Data Schemas

## AppConfig (TOML)
Example values shown — types are what `serde` will deserialize into.

```toml
[theme]
background     = "#0d0d12"   # hex string
foreground     = "#e6e6f0"   # hex string
accent         = "#aaa2f7"   # hex string
glow_intensity = 0.6         # f32, clamped 0.0..=1.0

[behavior]
default_repo    = "owner/repo"  # String
pr_fetch_limit  = 25            # usize

[keybindings]
# command name -> key combo
"open_palette" = "ctrl-p"
```

## PullRequest (Internal Rust Struct)
- `number`: `u32`
- `title`: `String`
- `author`: `String`
- `status`: `enum { Open, Merged, Closed }`
- `check_status`: `enum { Passing, Failing, Pending }`
