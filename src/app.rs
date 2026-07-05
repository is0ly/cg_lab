use std::sync::Arc;

use crate::engine::Engine;
use crate::AppConfig;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::ElementState,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

struct App {
    engine: Option<Engine>,

    config: AppConfig,
}

pub fn run(config: AppConfig) {
    env_logger::init();

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App {
        engine: None,
        config,
    };

    event_loop.run_app(&mut app).expect("Failed to run app");
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.engine.is_some() {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(self.config.title.clone())
                        .with_inner_size(LogicalSize::new(
                            f64::from(self.config.width),
                            f64::from(self.config.height),
                        )),
                )
                .expect("Failed to create window"),
        );

        let engine = pollster::block_on(Engine::new(
            event_loop.owned_display_handle(),
            window.clone(),
            self.config.clone(),
        ));

        self.engine = Some(engine);

        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(engine) = self.engine.as_mut() else {
            return;
        };

        if window_id != engine.window_id() {
            return;
        }

        engine.handle_window_event(&event);

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput { event, .. }
                if event.state == ElementState::Pressed
                    && !event.repeat
                    && matches!(event.physical_key, PhysicalKey::Code(KeyCode::Escape)) =>
            {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                engine.resize(size);
            }
            WindowEvent::RedrawRequested => {
                engine.frame();
                engine.request_redraw();
            }
            _ => {}
        }
    }
}
