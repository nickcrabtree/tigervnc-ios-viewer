pub mod input;
pub mod cache;
pub mod settings;

use log::info;

/// Placeholder core app, wraps winit+wgpu+egui integration.
pub struct VncApp {
    // TODO: renderer, egui ctx, connection, etc.
}

impl VncApp {
    pub fn new() -> Self {
        info!("VncApp::new()");
        Self {}
    }

    /// Handle a single winit event (skeleton)
    pub fn handle_event<T>(&mut self, _event: &winit::event::Event<T>) {
        // TODO: map events to egui, VNC input, etc.
    }

    /// Render one frame (skeleton)
    pub fn render(&mut self) {
        // TODO: wgpu render
    }
}

impl Default for VncApp {
    fn default() -> Self {
        Self::new()
    }
}

// Basic modules as stubs
pub mod input {
    //! Input handling: gestures, keyboard, mouse/trackpad, Apple Pencil
    
    /// Gesture state machine for touch input
    /// Handles: tap=click, two-finger drag=scroll, pinch=zoom
    pub struct GestureState;
    
    impl GestureState {
        pub fn new() -> Self { Self {} }
    }
    
    impl Default for GestureState {
        fn default() -> Self {
            Self::new()
        }
    }
    
    pub mod keyboard {
        //! Keyboard input handling
        pub fn handle_text_input(_s: &str) {}
        pub fn handle_keycode(_code: u32, _down: bool) {}
    }
    
    pub mod pointer {
        //! Mouse/trackpad pointer input
        pub fn mouse_move(_dx: f32, _dy: f32) {}
        pub fn mouse_button(_button: u8, _down: bool) {}
        pub fn scroll(_dx: f32, _dy: f32) {}
    }
    
    pub mod pencil {
        //! Apple Pencil stylus input
        pub fn stylus_event(_x: f32, _y: f32, _pressure: f32) {}
    }
}

pub mod cache {
    //! iOS-stubbed ContentCache and PersistentCache with reduced limits (256MB vs 2GB desktop).
    
    use log::info;
    
    pub trait ContentCache {
        fn memory_limit_bytes(&self) -> usize;
    }
    
    pub trait PersistentCache {
        fn disk_limit_bytes(&self) -> usize;
    }
    
    /// Stub implementation of ContentCache for iOS with 256MB memory limit
    pub struct IosContentCacheStub;
    
    impl ContentCache for IosContentCacheStub {
        fn memory_limit_bytes(&self) -> usize { 
            256 * 1024 * 1024  // 256 MB
        }
    }
    
    impl IosContentCacheStub {
        pub fn new() -> Self {
            info!("IosContentCacheStub::new() - 256MB memory limit");
            Self
        }
    }
    
    impl Default for IosContentCacheStub {
        fn default() -> Self {
            Self::new()
        }
    }
    
    /// Stub implementation of PersistentCache for iOS with 256MB disk limit
    pub struct IosPersistentCacheStub;
    
    impl PersistentCache for IosPersistentCacheStub {
        fn disk_limit_bytes(&self) -> usize { 
            256 * 1024 * 1024  // 256 MB
        }
    }
    
    impl IosPersistentCacheStub {
        pub fn new() -> Self {
            info!("IosPersistentCacheStub::new() - 256MB disk limit");
            Self
        }
    }
    
    impl Default for IosPersistentCacheStub {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub mod settings {
    //! Settings storage: UserDefaults on iOS, in-memory stub on Linux/macOS dev
    
    /// Viewer configuration settings
    #[derive(Default, Clone, Debug)]
    pub struct ViewerSettings {
        pub server: String,
        pub width: u32,
        pub height: u32,
        pub scale: f32,
    }

    #[cfg(target_os = "ios")]
    pub fn load() -> ViewerSettings {
        // TODO: call into objc2 UserDefaults when compiling for iOS.
        ViewerSettings::default()
    }

    #[cfg(target_os = "ios")]
    pub fn save(_s: &ViewerSettings) {
        // TODO: call into objc2 UserDefaults to persist.
    }

    #[cfg(not(target_os = "ios"))]
    pub fn load() -> ViewerSettings { 
        ViewerSettings::default() 
    }

    #[cfg(not(target_os = "ios"))]
    pub fn save(_s: &ViewerSettings) {}
}
