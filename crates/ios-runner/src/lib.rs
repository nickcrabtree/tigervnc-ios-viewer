use app_core::VncApp;
use log::info;
use winit::{event::Event, event_loop::EventLoop, window::WindowBuilder};

#[no_mangle]
pub extern "C" fn rust_main() {
    // Entrypoint called from Swift to start the app on iOS
    ios_run();
}

#[cfg(target_os = "ios")]
fn ios_run() {
    env_logger::init();
    info!("ios_run() starting");

    let event_loop = EventLoop::new().expect("EventLoop::new");
    let _window = WindowBuilder::new()
        .with_title("TigerVNC iOS Viewer")
        .build(&event_loop)
        .expect("window");

    let mut app = VncApp::new();

    let _ = event_loop.run(move |event, _control_flow| {
        match &event {
            Event::AboutToWait => {}
            Event::WindowEvent { event: winit::event::WindowEvent::RedrawRequested, .. } => {
                app.render();
            }
            _ => {}
        }
        app.handle_event(&event);
    });
}

#[cfg(not(target_os = "ios"))]
fn ios_run() {
    // no-op when not on iOS
    eprintln!("Warning: rust_main() called on non-iOS platform");
}

#[no_mangle]
pub extern "C" fn rust_on_memory_warning() {
    info!("rust_on_memory_warning() - TODO: aggressively evict caches");
    // TODO: notify caches to aggressively evict
}

#[no_mangle]
pub extern "C" fn rust_save_settings() {
    info!("rust_save_settings() - TODO: persist settings");
    // TODO: forward to app_core::settings::save(...)
}
