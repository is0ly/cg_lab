use std::{sync::Arc, time::Duration};

use winit::{
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::OwnedDisplayHandle,
    keyboard::KeyCode,
    window::{Window, WindowId},
};

use crate::{
    draw::DrawList,
    input::Input,
    math::{Vec2, Viewport},
    renderer::Renderer,
    sketch::{Sketch, SketchDrawContext, SketchInitContext, SketchUpdateContext},
    time::Time,
    AppConfig,
};

pub struct Engine<S: Sketch> {
    renderer: Renderer,
    sketch: S,
    time: Time,
    input: Input,
    canvas_size: Vec2,
    viewport: Viewport,
    paused: bool,
}

impl<S> Engine<S>
where
    S: Sketch,
{
    pub async fn new(display: OwnedDisplayHandle, window: Arc<Window>, config: AppConfig) -> Self {
        let mut renderer = Renderer::new(display, window).await;

        let canvas_size = Vec2::new(f32::from(config.width), f32::from(config.height));
        let viewport = Viewport::new(canvas_size.x, canvas_size.y);

        let init_ctx = SketchInitContext { canvas_size };
        let mut sketch = S::new(&init_ctx);

        let time = Time::new(Duration::from_secs_f32(1.0 / 60.0));
        let input = Input::new();

        let mut draw = DrawList::new(viewport);
        let draw_ctx = SketchDrawContext {
            canvas_size,
            input: &input,
        };

        sketch.draw(&mut draw, &draw_ctx);

        let mesh = draw.into_colored_mesh();
        renderer.upload_colored_mesh(&mesh);

        Self {
            renderer,
            sketch,
            time,
            input,
            canvas_size,
            viewport,
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
                let update_ctx = SketchUpdateContext {
                    fixed_delta: self.time.fixed_delta(),
                    canvas_size: self.canvas_size,
                    input: &self.input,
                };

                self.sketch.fixed_update(&update_ctx);
                self.time.consume_fixed_update();
            }
        }

        let mut draw = DrawList::new(self.viewport);

        let draw_ctx = SketchDrawContext {
            canvas_size: self.canvas_size,
            input: &self.input,
        };

        self.sketch.draw(&mut draw, &draw_ctx);

        let mesh = draw.into_colored_mesh();
        self.renderer.upload_colored_mesh(&mesh);

        self.renderer.render();

        self.input.end_frame();
    }
}
