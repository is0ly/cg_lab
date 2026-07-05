use cg_lab::{
    draw::DrawList,
    math::Vec2,
    render_data::Color,
    sketch::{Sketch, SketchDrawContext, SketchInitContext, SketchUpdateContext},
    AppConfig,
};
use winit::keyboard::KeyCode;

const WALKER_SPEED: f32 = 180.0;
const WALKER_DOT_SIZE: f32 = 2.0;

struct ManualWalkerSketch {
    walker: Walker,
}

impl Sketch for ManualWalkerSketch {
    fn new(ctx: &SketchInitContext) -> Self {
        Self {
            walker: Walker::new(Vec2::new(ctx.canvas_size.x * 0.5, ctx.canvas_size.y * 0.5)),
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

        self.walker
            .move_by(direction, ctx.fixed_delta.as_secs_f32(), ctx.canvas_size);
    }

    fn draw(&self, draw: &mut DrawList, _ctx: &SketchDrawContext) {
        draw.rect(
            self.walker.position,
            Vec2::new(WALKER_DOT_SIZE, WALKER_DOT_SIZE),
            Color::WHITE,
        );
    }
}

struct Walker {
    position: Vec2,
}

impl Walker {
    const fn new(position: Vec2) -> Self {
        Self { position }
    }

    fn move_by(&mut self, direction: Vec2, dt: f32, canvas_size: Vec2) {
        self.position.x += direction.x * WALKER_SPEED * dt;
        self.position.y += direction.y * WALKER_SPEED * dt;

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
