# Implementation Plan: TigerVNC iOS Viewer

## Goals

- Rust core (winit + wgpu + egui) reusing TigerVNC protocol crates from `../tigervnc/rust-vnc-viewer`
- iOS-specific input handling (touch/keyboard/mouse/pencil) and settings (UserDefaults)
- Universal app: iPhone (portrait only) + iPad (portrait + landscape)
- Full-screen only, light mode, iOS 17.0+
- ContentCache and PersistentCache protocols with iOS-appropriate resource limits

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Rust Workspace                            │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  app-core (shared logic)                             │   │
│  │  • winit + wgpu + egui integration                   │   │
│  │  • VNC protocol (rfb-* crates from ../tigervnc)      │   │
│  │  • Input/cache/settings modules                      │   │
│  └──────────────┬────────────────────────┬───────────────┘   │
│                 │                        │                    │
│                 ▼                        ▼                    │
│  ┌──────────────────────┐   ┌────────────────────────────┐  │
│  │  ios-runner          │   │  viewer-dev                │  │
│  │  (staticlib/cdylib)  │   │  (Linux desktop runner)    │  │
│  │  • FFI entrypoints   │   │  • Dev testing             │  │
│  └──────────┬───────────┘   └────────────────────────────┘  │
└─────────────┼───────────────────────────────────────────────┘
              │ FFI (C ABI)
              ▼
┌─────────────────────────────────────────────────────────────┐
│                    iOS Native (Swift/UIKit)                  │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  AppDelegate                                         │   │
│  │  • Calls rust_main() on launch                       │   │
│  │  • Forwards memory warnings                          │   │
│  │  • Manages app lifecycle                             │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  UserDefaults Bridge                                 │   │
│  │  • Swift FFI helpers (ios_get_string, ios_set_string)│   │
│  │  • Persistent settings storage                       │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Input Extensions (Swift)                            │   │
│  │  • Apple Pencil touch detection                      │   │
│  │  • Virtual keyboard management (UITextField)         │   │
│  │  • Advanced gesture handling                         │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Metal View (from winit)                             │   │
│  │  • CAMetalLayer for wgpu rendering                   │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Build System

### Linux Development
- Primary development environment
- `cargo run -p viewer-dev` launches desktop window
- Full Rust development experience (clippy, rustfmt, tests)

### macOS/iOS
- Xcode project in `ios/` directory (manual creation)
- Rust compiled to staticlib (`libios_runner.a`)
- Linked into Swift iOS app
- Build process:
  1. `cargo build --release --target aarch64-apple-ios --package ios-runner`
  2. Open Xcode project
  3. Build & Run

### Toolchain
- Rust stable (pinned via `rust-toolchain.toml`)
- Components: `rust-src`, `rustfmt`, `clippy`
- iOS targets: `aarch64-apple-ios`, `aarch64-apple-ios-sim`

## Memory Strategy (iOS Constraints)

### ContentCache (In-Memory)
- **Limit**: 256 MB (vs 2 GB desktop)
- **Eviction**: LRU with size accounting
- **Triggers**:
  - Memory warning from iOS
  - Cache size threshold exceeded
  - App backgrounding

### PersistentCache (On-Disk)
- **Limit**: 256 MB
- **Location**: `Library/Caches/com.nickcrabtree.tigervnc.ios.viewer/`
- **Excluded from iCloud backup**: Set `NSURLIsExcludedFromBackupKey`
- **Trimming**: Background task + app lifecycle hooks

### Memory Pressure Response
When `UIApplicationDidReceiveMemoryWarning()` fires:
1. Swift calls `rust_on_memory_warning()`
2. Rust evicts 50%+ of ContentCache (least-recently-used)
3. Flush staging buffers
4. Signal wgpu to release texture cache
5. Compact PersistentCache index

## Input Architecture

### Touch Gestures (winit)
- **Tap**: Left click at touch point
- **Double-tap**: Double-click
- **Long-press** (>500ms): Right click
- **Two-finger drag**: Scroll (map to VNC wheel events)
- **Pinch**: Client-side zoom/scale adjustment

### External Keyboard (winit + Swift)
- **Primary**: winit `KeyboardInput` events
- **Fallback**: Swift `UIKeyCommand` for system-reserved combos
- **Mapping**: Reuse keysym mapping from `platform-input` crate

### Mouse/Trackpad (iPadOS)
- **Primary**: winit pointer events
- **Fallback**: Swift `UIPointerInteraction` if gaps exist
- **Cursor**: System cursor + VNC cursor rendering

### Apple Pencil (Swift FFI)
- Detect `UITouch.type == .stylus` in Swift
- Forward `(x, y, pressure, altitude, azimuth)` via FFI:
  ```c
  void rust_pencil_event(float x, float y, float pressure, float altitude, float azimuth);
  ```
- Map to pointer event with pressure in Rust

### Virtual Keyboard (Swift + Hidden UITextField)
- Place hidden `UITextField` in view hierarchy
- On canvas tap: `becomeFirstResponder()` to show keyboard
- Forward text insertions to Rust:
  ```c
  void rust_text_input(const char* utf8_text);
  ```

## iOS Project Structure (to be created on macOS)

```
ios/
├── TigerVNCViewer.xcodeproj/
├── TigerVNCViewer/
│   ├── AppDelegate.swift           # Main entry, calls rust_main()
│   ├── RustBridge.h                # C header for Rust FFI
│   ├── UserDefaultsBridge.swift   # Settings FFI helpers
│   ├── PencilInputBridge.swift    # Apple Pencil forwarding
│   ├── Info.plist                  # Orientation, full-screen, iOS 17.0
│   └── Assets.xcassets/            # App icon, launch images
└── libios_runner.a                 # Rust staticlib (built separately)
```

### Info.plist Configuration
```xml
<key>UIRequiresFullScreen</key>
<true/>
<key>UIUserInterfaceStyle</key>
<string>Light</string>
<key>UISupportedInterfaceOrientations</key>
<array>
  <string>UIInterfaceOrientationPortrait</string>
</array>
<key>UISupportedInterfaceOrientations~ipad</key>
<array>
  <string>UIInterfaceOrientationPortrait</string>
  <string>UIInterfaceOrientationLandscapeLeft</string>
  <string>UIInterfaceOrientationLandscapeRight</string>
</array>
<key>MinimumOSVersion</key>
<string>17.0</string>
```

## Testing Strategy

### Linux
- Unit tests for:
  - Cache eviction logic
  - Gesture state machines
  - Input mapping
- Integration tests for protocol stacks (where possible without server)

### iOS Simulator
- Smoke tests: launch, render, orientation changes
- Basic gesture testing (tap, drag)
- Keyboard input

### iOS Device
- Apple Pencil pressure/tilt
- External keyboard with all modifier keys
- External mouse/trackpad
- Performance profiling (Instruments)
- Battery drain testing
- Memory pressure scenarios

## Deployment & Signing

- **Bundle ID**: `com.nickcrabtree.tigervnc.ios.viewer`
- **Minimum iOS**: 17.0
- **Entitlements**: Network client only (VPN handled externally)
- **Signing**: Automatic signing in Xcode (set Team)
- **Distribution**: Not targeting App Store initially (development builds only)

## Phased Roadmap

### Phase 0: Scaffolding ✅ (Current)
- [x] Git repo initialization
- [x] Cargo workspace setup
- [x] Crate structure (app-core, ios-runner, viewer-dev)
- [x] Stub modules (input, cache, settings)
- [x] README and IMPLEMENTATION_PLAN

### Phase 1: VNC Protocol Integration (1-2 weeks)
- [ ] Wire actual VNC crates from `../tigervnc/rust-vnc-viewer`
- [ ] Implement connection setup in app-core
- [ ] Framebuffer updates and rendering to in-memory buffer
- [ ] Test with `viewer-dev` on Linux against VNC server

### Phase 2: Rendering Pipeline (1 week)
- [ ] wgpu swapchain setup
- [ ] Framebuffer texture upload
- [ ] egui overlay for connection dialog
- [ ] Scaling/viewport management

### Phase 3: Linux Input Parity (3-4 days)
- [ ] Mouse pointer mapping
- [ ] Keyboard input (keysym translation)
- [ ] Verify against desktop Rust viewer behavior

### Phase 4: iOS Project Creation (1 week, on macOS)
- [ ] Create Xcode project manually
- [ ] Swift AppDelegate calling `rust_main()`
- [ ] RustBridge.h with FFI declarations
- [ ] Build script to compile and link Rust staticlib
- [ ] Confirm window renders on iOS simulator

### Phase 5: iOS Advanced Input (1-2 weeks)
- [ ] Touch gesture state machine
- [ ] Two-finger scroll, pinch, long-press
- [ ] Apple Pencil Swift bridge
- [ ] Virtual keyboard management
- [ ] External keyboard/mouse passthrough

### Phase 6: Cache Integration (1 week)
- [ ] Implement LRU ContentCache with 256MB cap
- [ ] Implement PersistentCache with disk I/O
- [ ] Memory warning handler
- [ ] Background trimming task

### Phase 7: QA & Performance (1-2 weeks)
- [ ] Device testing (iPhone + iPad)
- [ ] Orientation handling
- [ ] Battery profiling
- [ ] Network interruption handling
- [ ] Crash reporting

### Phase 8: Full Cache Protocol Implementation (TBD)
- [ ] Replace stubs once protocols stabilized in `../tigervnc/rust-vnc-viewer`
- [ ] Integration testing with ContentCache/PersistentCache servers
- [ ] Performance validation (97-99% bandwidth reduction)

## Migration Path for Caches

### Current State (Phase 0-6)
- `IosContentCacheStub` and `IosPersistentCacheStub` with 256MB caps
- Log invocations, no-op eviction

### Future Implementation
- Async read-through caches with Tokio
- On-disk sharding for PersistentCache
- Background trimming with iOS `BGTask` framework
- Warmup of hot entries on cold start
- Feature flags to toggle desktop (2GB) vs iOS (256MB) limits

## Next Steps (Immediate)

1. **Verify build**: `cargo build` on Linux
2. **Test dev runner**: `cargo run -p viewer-dev` (expect empty window)
3. **Initial commit**: Push to GitHub
4. **macOS setup** (later): Create Xcode project following Phase 4 plan

---

**Last Updated**: 2025-11-01
**Status**: Phase 0 Complete, ready for Phase 1
