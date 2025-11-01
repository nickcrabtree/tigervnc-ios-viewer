# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

Cross-platform Rust VNC viewer for iOS/iPadOS with ContentCache and PersistentCache protocol support. Currently in **Phase 0** (scaffolding complete). Primary development happens on Linux; iOS builds require macOS with Xcode.

## Development Commands

### Linux Development (Primary Environment)

```bash
# Run desktop development viewer (windowed environment for testing)
cargo run -p viewer-dev

# Build all workspace crates
cargo build

# Type-check without full compilation
cargo check

# Run tests (currently no tests exist)
cargo test

# Format code
cargo fmt

# Lint with Clippy
cargo clippy

# Clean build artifacts
cargo clean
```

### iOS Building (macOS Only)

iOS Xcode project creation is pending (Phase 4). When ready:

```bash
# Add iOS targets (one-time setup)
rustup target add aarch64-apple-ios aarch64-apple-ios-sim

# Build iOS staticlib
cargo build --release --target aarch64-apple-ios --package ios-runner

# Then open ios/TigerVNCViewer.xcodeproj in Xcode to build and run
```

## Architecture

### Workspace Structure

```
tigervnc_ios_viewer/
├── crates/
│   ├── app-core/       # Shared viewer core (winit+wgpu+egui+VNC protocol)
│   │                   # Modules: input, cache, settings
│   ├── ios-runner/     # iOS FFI entrypoints (staticlib called from Swift)
│   │                   # Exports: rust_main(), rust_on_memory_warning(), rust_save_settings()
│   └── viewer-dev/     # Linux desktop dev runner
├── ios/                # Xcode project (to be created on macOS)
└── Cargo.toml         # Workspace root
```

### Key Architectural Patterns

**Dual Build System**: 
- Linux: Direct binary execution via `viewer-dev` crate for rapid iteration
- iOS: Rust compiled to staticlib (`ios-runner`), linked into Swift iOS app via FFI

**VNC Protocol Dependencies** (from sibling repo `../tigervnc/rust-vnc-viewer/`):
- `rfb-common` - Common types and geometry
- `rfb-pixelbuffer` - Pixel buffer management  
- `rfb-protocol` - RFB protocol implementation
- `rfb-encodings` - Encoding/decoding (Tight, ZRLE, etc.)
- `rfb-client` - High-level async VNC client
- `rfb-display` - Display rendering utilities
- `platform-input` - Input mapping and handling

**Rendering Pipeline**: wgpu (Metal on iOS) with egui overlay, managed through winit for cross-platform windowing.

**Async Runtime**: Tokio for networking and async VNC protocol handling.

### iOS Memory Constraints

iOS builds enforce strict memory limits compared to desktop:

- **ContentCache**: 256 MB in-RAM (vs 2 GB desktop)
- **PersistentCache**: 256 MB on-disk in `Library/Caches/`
- **Memory Pressure Response**: Swift calls `rust_on_memory_warning()` → Rust evicts 50%+ of ContentCache (LRU), flushes staging buffers, signals wgpu to release texture cache

This is currently implemented as stubs in `app-core/src/lib.rs` (`IosContentCacheStub`, `IosPersistentCacheStub`).

### Input Handling Strategy

**Touch Gestures** (winit):
- Tap = left click
- Double-tap = double-click  
- Long-press (>500ms) = right click
- Two-finger drag = scroll
- Pinch = client-side zoom

**External Keyboard/Mouse**: Primarily handled by winit events, with Swift `UIKeyCommand`/`UIPointerInteraction` fallbacks for system-reserved inputs.

**Apple Pencil**: Detected in Swift (`UITouch.type == .stylus`), forwarded via FFI with pressure/tilt data.

**Virtual Keyboard**: Hidden `UITextField` in view hierarchy; text insertions forwarded to Rust via FFI.

## Code Organization

### app-core modules:
- `input::*` - Gesture state machines, keyboard/pointer/pencil handlers
- `cache::*` - ContentCache and PersistentCache stub implementations with iOS limits
- `settings::*` - ViewerSettings struct; iOS uses UserDefaults via objc2, Linux uses in-memory stub

### FFI Boundary (ios-runner):
- `rust_main()` - Entrypoint called from Swift AppDelegate
- `rust_on_memory_warning()` - Memory pressure handler
- `rust_save_settings()` - Settings persistence trigger

## Important Context

### Current Development Phase

**Phase 0 Complete**: Scaffolding and workspace setup done.

**Next Steps (Phase 1)**: Wire actual VNC protocol crates from sibling repo, implement connection setup, framebuffer updates, and test with `viewer-dev` on Linux.

See `IMPLEMENTATION_PLAN.md` for detailed 8-phase roadmap.

### Toolchain Requirements

- **Rust**: Stable (pinned via `rust-toolchain.toml`, currently 1.81+)
- **Components**: rust-src, rustfmt, clippy (specified in rust-toolchain.toml)
- **macOS (for iOS)**: Xcode 15+ with iOS 17.0+ SDK

### iOS App Configuration (Pending)

When Xcode project is created:
- **Bundle ID**: `com.nickcrabtree.tigervnc.ios.viewer`
- **Minimum iOS**: 17.0
- **Orientation**: iPhone (portrait only), iPad (portrait + landscape)
- **Full-screen only**, light mode only

### UI Behavior Preference

The project prefers to rely on CSS and trust the browser's capabilities rather than using complex JavaScript for UI behavior.
