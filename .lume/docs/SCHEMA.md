# Lume Data Schemas

## AppConfig (TOML)
```toml
[theme]
background = "String (Hex)"
foreground = "String (Hex)"
accent = "String (Hex)"
glow_intensity = "f32 (0.0 - 1.0)"

[behavior]
default_repo = "String"
pr_fetch_limit = "usize"

[keybindings]
# Map: "Command Name" -> "Key Combo"
"open_palette" = "ctrl-p"
```

## PullRequest (Internal Rust Struct)
- `number`: u32
- `title`: String
- `author`: String
- `status`: Enum (Open, Merged, Closed)
- `check_status`: Enum (Passing, Failing, Pending)
