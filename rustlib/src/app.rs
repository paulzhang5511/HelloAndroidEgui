use crate::UserEvent;
use egui_wgpu::{wgpu, ScreenDescriptor};
use log::debug;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::Key;
use winit::window::{Window, WindowId};

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        debug!("生命周期: resumed");
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        debug!("生命周期: suspended");
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        debug!("生命周期: window_event: {:?}", event);
        match event {
            WindowEvent::CloseRequested => {
                debug!("事件: CloseRequested");
            }
            WindowEvent::RedrawRequested => {
                debug!("事件: RedrawRequested");
            }
            WindowEvent::Resized(new_size) => {
                debug!("事件: Resized: {:?}", new_size);
            }
            WindowEvent::KeyboardInput {
                event,
                is_synthetic: false,
                ..
            } => {
                // Dispatch actions only on press.
                if event.state.is_pressed() {
                    debug!("事件: KeyboardInput-physical_key: {:?}", event.physical_key);
                    debug!("事件: KeyboardInput-logical_key: {:?}", event.logical_key);

                    if let Key::Named(ch) = event.logical_key.as_ref() {
                        debug!("事件: Named: {:?}", ch);
                        event_loop.exit();
                    }
                }
            }
            _ => (),
        }
    }
}
