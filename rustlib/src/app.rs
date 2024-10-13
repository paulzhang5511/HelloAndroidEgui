use crate::UserEvent;
use egui_wgpu::{wgpu, ScreenDescriptor};
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JavaVM;
use log::debug;
use ndk::native_activity::NativeActivity;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::Key;
use winit::platform::android::activity::AndroidApp;
use winit::window::{Window, WindowId};

pub struct App {
    android_app: AndroidApp,
}

impl App {
    pub fn new(android_app: AndroidApp) -> Self {
        Self { android_app }
    }
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        debug!("生命周期: resumed");
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        debug!("生命周期: suspended");
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        debug!("生命周期: about_to_wait");
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        debug!("生命周期: window_event: {:?}", event);
        match event {
            WindowEvent::RedrawRequested => {
                debug!("事件: RedrawRequested");
            }
            WindowEvent::Resized(new_size) => {
                // 确认窗口大小
                debug!("事件: Resized: {:?}", new_size);
            }
            WindowEvent::CloseRequested => {
                debug!("事件: CloseRequested");
                event_loop.exit();
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
                        // event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
                        event_loop.exit();
                        // unsafe {
                        //     debug!("调用 unsafe...");
                        //     let vm_ptr = self.android_app.vm_as_ptr();
                        //     let vm = unsafe { JavaVM::from_raw(vm_ptr as *mut jni::sys::JavaVM) }
                        //         .unwrap();
                        //     let mut env = vm.get_env().unwrap();
                        //     let activity_ptr = self.android_app.activity_as_ptr();
                        //     if !activity_ptr.is_null() {
                        //         let activity =
                        //             unsafe { JObject::from_raw(activity_ptr as jobject) };
                        //         env.call_method(activity, "finishMe", "()V", &[]).unwrap();
                        //         debug!("调用 finishMe");
                        //     }
                        //     // event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
                        //     // event_loop.exit();
                        // }
                    }
                }
            }
            _ => (),
        }
    }
}
