use cg_lab::{
    draw::DrawList,
    math::Vec2,
    render_data::Color,
    sketch::{Sketch, SketchDrawContext, SketchInitContext, SketchUpdateContext},
    AppConfig,
};
use winit::keyboard::KeyCode;

const WALKER_STEP_SIZE: f32 = 2.0;
const WALKER_DOT_SIZE: f32 = 1.0;

struct ManualWalkerSketch {
    walker: Walker,
    pending_stamps: Vec<Vec2>,
}

impl Sketch for ManualWalkerSketch {
    fn new(ctx: &SketchInitContext) -> Self {
        let start_position = Vec2::new(ctx.canvas_size.x * 0.5, ctx.canvas_size.y * 0.5);

        Self {
            walker: Walker::new(start_position),
            pending_stamps: vec![start_position],
        }
    }

    fn fixed_update(&mut self, ctx: &SketchUpdateContext) {
        let mut direction = Vec2::ZERO;

        if ctx.input.key_down(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }

        if ctx.input.key_down(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        if ctx.input.key_down(KeyCode::ArrowUp) {
            direction.y -= 1.0;
        }

        if ctx.input.key_down(KeyCode::ArrowDown) {
            direction.y += 1.0;
        }

        if direction.x == 0.0 && direction.y == 0.0 {
            return;
        }

        self.walker.step(direction, ctx.canvas_size);
        self.pending_stamps.push(self.walker.position);
    }

    fn draw(&mut self, draw: &mut DrawList, _ctx: &SketchDrawContext) {
        for position in self.pending_stamps.drain(..) {
            draw.rect(
                position,
                Vec2::new(WALKER_DOT_SIZE, WALKER_DOT_SIZE),
                Color::WHITE,
            );
        }
    }
}

struct Walker {
    position: Vec2,
}

impl Walker {
    const fn new(position: Vec2) -> Self {
        Self { position }
    }

    fn step(&mut self, mut direction: Vec2, canvas_size: Vec2) {
        let length = (direction.x * direction.x + direction.y * direction.y).sqrt();

        direction.x /= length;
        direction.y /= length;

        self.position.x += direction.x * WALKER_STEP_SIZE;
        self.position.y += direction.y * WALKER_STEP_SIZE;

        let half_dot = WALKER_DOT_SIZE * 0.5;

        self.position.x = self.position.x.clamp(half_dot, canvas_size.x - half_dot);
        self.position.y = self.position.y.clamp(half_dot, canvas_size.y - half_dot);
    }
}

fn main() {
    cg_lab::run::<ManualWalkerSketch>(AppConfig {
        title: "Manual Walker".to_string(),
        width: 800,
        height: 800,
    });
}
