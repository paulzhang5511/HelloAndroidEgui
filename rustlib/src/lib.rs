use egui_winit::winit::platform::android::activity::AndroidApp;
mod app;

/// Android入口点
#[allow(dead_code)]
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(android_app: AndroidApp) {
    use android_logger::FilterBuilder;
    use app::App;
    use egui_winit::winit::platform::android::EventLoopBuilderExtAndroid;
    use log::LevelFilter;

    std::env::set_var("RUST_LOG", "rustlib=debug,winit=debug");
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag("rustlib")
            .with_filter(FilterBuilder::new().parse("debug").build()),
    );

    let android_app1 = android_app.clone();
    let event_loop = winit::event_loop::EventLoop::<UserEvent>::with_user_event()
        .with_android_app(android_app1)
        .build();
    match event_loop {
        Ok(event_loop) => {
            event_loop.listen_device_events(winit::event_loop::DeviceEvents::Never);
            let mut app = App::new(android_app);
            event_loop.run_app(&mut app).unwrap();
        }
        Err(e) => {
            log::error!("Failed to create event loop: {:?}", e);
            return;
        }
    }
}

/// A custom event type for the winit app.
#[derive(Debug, Clone, Copy)]
pub enum UserEvent {
    RequestRedraw,
}
