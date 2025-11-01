# TigerVNC iOS Viewer

Cross-platform Rust VNC viewer for iOS/iPadOS with ContentCache and PersistentCache protocol support.

## Overview

Rust-based VNC viewer using `winit` + `wgpu` + `egui`, sharing protocol code with the desktop TigerVNC Rust viewer. Primary development on Linux; iOS builds require macOS with Xcode.

## Project Structure

```
tigervnc_ios_viewer/
├── crates/
│   ├── app-core/          # Shared viewer core (winit+wgpu+egui+VNC protocol)
│   ├── ios-runner/        # iOS FFI entrypoints (staticlib called from Swift)
│   └── viewer-dev/        # Linux desktop dev runner
├── ios/                   # Xcode project (to be created on macOS)
└── Cargo.toml            # Workspace root
```

## Features

- **Universal iOS App**: iPhone (portrait) + iPad (portrait/landscape)
- **Full-screen only**: No multitasking, light mode only
- **Input Support**:
  - Touch gestures (tap, two-finger scroll, pinch, long-press)
  - External keyboard and mouse/trackpad
  - Apple Pencil with pressure sensitivity
  - Virtual keyboard
- **Cache Protocols**: Stubs for ContentCache and PersistentCache (256MB limits for iOS)
- **Settings**: UserDefaults integration via Swift/Rust FFI

## Prerequisites

### Linux (Development)
- Rust toolchain (`rustup`)
- Git + GitHub CLI (`gh`)
- Standard build tools

### macOS (iOS Builds)
- Xcode 15+ with iOS 17.0+ SDK
- Rust toolchain with iOS targets:
  ```bash
  rustup target add aarch64-apple-ios aarch64-apple-ios-sim
  ```

## Building

### Linux Development

Run the desktop development viewer:

```bash
cargo run -p viewer-dev
```

This provides a windowed environment for testing core logic without iOS.

### iOS (on macOS)

The iOS Xcode project will be created manually on macOS. See `IMPLEMENTATION_PLAN.md` for detailed steps.

Basic workflow:
1. Clone repo on Mac
2. Open `ios/TigerVNCViewer.xcodeproj` in Xcode
3. Set signing team
4. Build & Run on simulator or device

## Dependencies

This workspace depends on VNC protocol crates from the sibling `../tigervnc/rust-vnc-viewer/` repository:

- `rfb-common` - Common types and geometry
- `rfb-pixelbuffer` - Pixel buffer management
- `rfb-protocol` - RFB protocol implementation
- `rfb-encodings` - Encoding/decoding (Tight, ZRLE, etc.)
- `rfb-client` - High-level async VNC client

## Architecture

- **Rendering**: wgpu (Metal on iOS) with egui overlay
- **Windowing**: winit with iOS-specific extensions
- **Networking**: Tokio async runtime
- **FFI**: Rust staticlib called from Swift (AppDelegate)

## Memory Constraints (iOS)

- **ContentCache**: 256 MB in-RAM (vs 2 GB desktop)
- **PersistentCache**: 256 MB on-disk in `Library/Caches/`
- **Memory Pressure**: Responds to `UIApplicationDidReceiveMemoryWarning` with aggressive eviction

## Development Roadmap

See `IMPLEMENTATION_PLAN.md` for detailed phased roadmap.

- **Phase 0**: ✅ Scaffolding and workspace setup
- **Phase 1**: Wire VNC protocol crates
- **Phase 2**: Rendering pipeline (wgpu + egui)
- **Phase 3**: Input mapping (Linux parity)
- **Phase 4**: iOS Xcode project and basic touch input
- **Phase 5**: Advanced iOS input (gestures, Pencil, external devices)
- **Phase 6**: Cache stubs with memory pressure handling
- **Phase 7**: Device QA and performance tuning
- **Phase 8**: Full ContentCache/PersistentCache implementation

## License

GPL-2.0-or-later (matching TigerVNC)

## Authors

Nick Crabtree <nickcrabtree@gmail.com>
