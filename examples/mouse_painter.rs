use cg_lab::{
    draw::DrawList,
    math::Vec2,
    render_data::Color,
    sketch::{Sketch, SketchDrawContext, SketchInitContext, SketchUpdateContext},
    AppConfig,
};
use winit::event::MouseButton;

const BRUSH_SIZE: f32 = 3.0;
const BRUSH_SPACING: f32 = 4.0;

struct MousePainterSketch {
    brush: Brush,
    pending_stamps: Vec<Vec2>,
}

impl Sketch for MousePainterSketch {
    fn new(_ctx: &SketchInitContext) -> Self {
        Self {
            brush: Brush::new(),
            pending_stamps: Vec::new(),
        }
    }

    fn fixed_update(&mut self, ctx: &SketchUpdateContext) {
        if !ctx.input.mouse_down(MouseButton::Left) {
            self.brush.end_stroke();
            return;
        }

        let Some(mouse_position) = ctx.input.mouse_position() else {
            self.brush.end_stroke();
            return;
        };

        self.brush.move_to(mouse_position, &mut self.pending_stamps);
    }

    fn draw(&mut self, draw: &mut DrawList, _ctx: &SketchDrawContext) {
        for position in self.pending_stamps.drain(..) {
            draw.rect(position, Vec2::new(BRUSH_SIZE, BRUSH_SIZE), Color::WHITE);
        }
    }
}

struct Brush {
    last_position: Option<Vec2>,
    distance_since_last_stamp: f32,
}

impl Brush {
    const fn new() -> Self {
        Self {
            last_position: None,
            distance_since_last_stamp: 0.0,
        }
    }

    fn end_stroke(&mut self) {
        self.last_position = None;
        self.distance_since_last_stamp = 0.0;
    }

    fn move_to(&mut self, position: Vec2, stamps: &mut Vec<Vec2>) {
        let Some(last_position) = self.last_position else {
            stamps.push(position);
            self.last_position = Some(position);
            self.distance_since_last_stamp = 0.0;
            return;
        };

        self.push_stamps_between(last_position, position, stamps);
        self.last_position = Some(position);
    }

    fn push_stamps_between(&mut self, from: Vec2, to: Vec2, stamps: &mut Vec<Vec2>) {
        let delta = Vec2::new(to.x - from.x, to.y - from.y);
        let distance = (delta.x * delta.x + delta.y * delta.y).sqrt();

        if distance == 0.0 {
            return;
        }

        let direction = Vec2::new(delta.x / distance, delta.y / distance);

        let mut travelled = BRUSH_SPACING - self.distance_since_last_stamp;

        while travelled <= distance {
            stamps.push(Vec2::new(
                from.x + direction.x * travelled,
                from.y + direction.y * travelled,
            ));

            travelled += BRUSH_SPACING;
        }

        let total = self.distance_since_last_stamp + distance;
        self.distance_since_last_stamp = total % BRUSH_SPACING;
    }
}

fn main() {
    cg_lab::run::<MousePainterSketch>(AppConfig {
        title: "Mouse Painter".to_string(),
        width: 800,
        height: 800,
    });
}
