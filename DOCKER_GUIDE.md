# Lume Development Guide

This project supports multiple environments. Because Lume is a GPU-accelerated application, the setup differs between Linux and macOS.

## 🍎 macOS Setup (GUI Development)
Since Docker on Mac cannot access the GPU for windowed applications, **native development is required for GUI work.**

### Quick Start
Run the setup script to install the Rust toolchain and dependencies:
```bash
./scripts/setup-mac.sh
```
Then run the app:
```bash
cargo run
```

---

## 🐧 Linux Setup (GUI Development)
Linux users can use Docker to maintain a clean host machine.

### 1. Prerequisites
- **Docker** and **Nvidia Container Toolkit**.
- **X11** server.

### 2. Prepare Host
```bash
xhost +local:docker
```

### 3. Start & Enter
```bash
docker compose up -d
docker exec -it lume-dev-container bash
```

---

## 🤖 Headless Testing (All Platforms)
The Docker environment is used across all platforms for **Headless Testing** (CI/CD and logic verification) where no physical window is required.

To run headless tests in Docker:
```bash
docker compose run --rm lume-dev cargo test
```

## 📦 Build Caching
We use a named volume `lume-target` to persist your build artifacts. This means when you stop the container, your `target/` folder is saved.
