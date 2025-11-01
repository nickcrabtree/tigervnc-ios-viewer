use app_core::VncApp;
use log::info;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    env_logger::init();
    info!("Starting TigerVNC iOS Viewer (Linux dev shell)");
    
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("TigerVNC iOS Viewer (Linux dev shell)")
        .with_inner_size(winit::dpi::LogicalSize::new(1024, 768))
        .build(&event_loop)
        .unwrap();

    let mut app = VncApp::new();

    let _ = event_loop.run(move |event, control_flow| {
        match &event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                info!("Close requested, exiting");
                control_flow.exit();
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                app.render();
            }
            _ => {}
        }
        app.handle_event(&event);
    });
}
