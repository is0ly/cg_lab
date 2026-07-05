use cg_lab::{
    draw::DrawList,
    math::Vec2,
    random,
    render_data::Color,
    sketch::{Sketch, SketchDrawContext, SketchInitContext, SketchUpdateContext},
    AppConfig,
};

const WALKER_STEP_SIZE: f32 = 2.0;
const WALKER_DOT_SIZE: f32 = 1.0;

// 60 fixed updates в секунду / 60 = 1 шаг в секунду.
const WALKER_STEP_EVERY_N_TICKS: u64 = 1;

struct RandomWalkerSketch {
    fixed_ticks: u64,
    canvas_size: Vec2,
    walker: Walker,
}

impl Sketch for RandomWalkerSketch {
    fn new(ctx: &SketchInitContext) -> Self {
        Self {
            fixed_ticks: 0,
            canvas_size: ctx.canvas_size,
            walker: Walker::new(Vec2::new(ctx.canvas_size.x * 0.5, ctx.canvas_size.y * 0.5)),
        }
    }

    fn fixed_update(&mut self, _ctx: &SketchUpdateContext) {
        self.fixed_ticks += 1;

        if self.fixed_ticks % WALKER_STEP_EVERY_N_TICKS == 0 {
            self.walker.step(self.canvas_size);
        }
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

    fn step(&mut self, canvas_size: Vec2) {
        let direction = random::range_u32(0, 4);

        match direction {
            0 => self.position.x += WALKER_STEP_SIZE,
            1 => self.position.x -= WALKER_STEP_SIZE,
            2 => self.position.y += WALKER_STEP_SIZE,
            3 => self.position.y -= WALKER_STEP_SIZE,
            _ => unreachable!("direction must be in 0..4"),
        }

        let half_dot = WALKER_DOT_SIZE * 0.5;

        self.position.x = self.position.x.clamp(half_dot, canvas_size.x - half_dot);

        self.position.y = self.position.y.clamp(half_dot, canvas_size.y - half_dot);
    }
}

fn main() {
    cg_lab::run::<RandomWalkerSketch>(AppConfig {
        title: "Random Walker".to_string(),
        width: 800,
        height: 800,
    });
}
