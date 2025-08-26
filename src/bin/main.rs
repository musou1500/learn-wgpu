use std::time::Instant;

use winit::{
    application::ApplicationHandler,
    event::{self, DeviceEvent, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

use rust_terrain_codex::state::WindowState;

struct App {
    window_state: Option<WindowState>,
    last_render_time: Instant,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window_state.is_some() {
            return;
        }

        let window = std::sync::Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("Hello, Winit!"))
                .unwrap(),
        );
        self.window_state = Some(pollster::block_on(WindowState::new(window.clone())));
    }

    fn device_event(
        &mut self,
        _: &winit::event_loop::ActiveEventLoop,
        _: event::DeviceId,
        event: event::DeviceEvent,
    ) {
        if let Some(window_state) = self.window_state.as_mut()
            && let DeviceEvent::MouseMotion { delta } = event
            && window_state.mouse_pressed
        {
            window_state
                .camera_controller
                .handle_mouse(delta.0, delta.1)
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: event::WindowEvent,
    ) {
        if let Some(window_state) = self.window_state.as_mut() {
            match event {
                WindowEvent::Resized(size) => {
                    window_state.resize(size);
                }
                WindowEvent::RedrawRequested => {
                    let now = Instant::now();
                    let dt = now - self.last_render_time;
                    self.last_render_time = now;
                    window_state.update(dt);
                    match window_state.render() {
                        Ok(_) => {}
                        Err(e) => {
                            log::error!("Unable to render {}", e);
                        }
                    }
                }
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                _ => {
                    window_state.window_event(event);
                }
            }
        }
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut app = App {
        window_state: None,
        last_render_time: Instant::now(),
    };
    event_loop.run_app(&mut app).unwrap();
}
