use crate::input::Input;
use crate::{
    draw::DrawList,
    math::{Vec2, Viewport},
    renderer::Renderer,
    sketch::Sketch,
    time::Time,
    AppConfig,
};
use std::sync::Arc;
use std::time::Duration;
use winit::{
    dpi::PhysicalSize, event::WindowEvent, event_loop::OwnedDisplayHandle, keyboard::KeyCode,
    window::Window, window::WindowId,
};

pub struct Engine {
    renderer: Renderer,
    sketch: Sketch,
    time: Time,
    viewport: Viewport,
    input: Input,
    paused: bool,
}

impl Engine {
    pub async fn new(display: OwnedDisplayHandle, window: Arc<Window>, config: AppConfig) -> Self {
        let mut renderer = Renderer::new(display, window).await;

        let canvas_size = Vec2::new(f32::from(config.width), f32::from(config.height));
        let sketch = Sketch::new(canvas_size);

        let time = Time::new(Duration::from_secs_f32(1.0 / 60.0));
        let viewport = Viewport::new(canvas_size.x, canvas_size.y);

        let mut draw = DrawList::new(viewport);
        sketch.draw(&mut draw);

        let mesh = draw.into_colored_mesh();
        renderer.upload_colored_mesh(&mesh);

        Self {
            renderer,
            sketch,
            time,
            viewport,
            input: Input::new(),
            paused: false,
        }
    }
    pub fn window_id(&self) -> WindowId {
        self.renderer.window_id()
    }

    pub fn request_redraw(&self) {
        self.renderer.request_redraw();
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.renderer.resize(size);
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        self.input.handle_window_event(event);
    }

    pub fn frame(&mut self) {
        self.time.begin_frame();

        if self.input.key_just_pressed(KeyCode::KeyC) {
            self.renderer.request_canvas_clear();
        }

        if self.input.key_just_pressed(KeyCode::Space) {
            self.paused = !self.paused;
        }

        if self.paused {
            self.time.clear_accumulator();
        } else {
            while self.time.should_run_fixed_update() {
                self.sketch.fixed_update(self.time.fixed_delta());
                self.time.consume_fixed_update();
            }
        }

        let mut draw = DrawList::new(self.viewport);
        self.sketch.draw(&mut draw);

        let mesh = draw.into_colored_mesh();
        self.renderer.upload_colored_mesh(&mesh);

        self.renderer.render();

        self.input.end_frame();
    }
}
