use cg_lab::{
    draw::DrawList,
    math::Vec2,
    render_data::Color,
    sketch::{Sketch, SketchDrawContext, SketchInitContext, SketchUpdateContext},
    AppConfig,
};

const WALKER_STEP_SIZE: f32 = 2.0;
const WALKER_DOT_SIZE: f32 = 1.0;

const DIRECTION_CHANGE_EVERY_N_TICKS: u64 = 15;

struct AutonomousWalkerSketch {
    fixed_ticks: u64,
    walker: Walker,
    pending_stamps: Vec<Vec2>,
}

impl Sketch for AutonomousWalkerSketch {
    fn new(ctx: &SketchInitContext) -> Self {
        let start_position = Vec2::new(ctx.canvas_size.x * 0.5, ctx.canvas_size.y * 0.5);

        Self {
            fixed_ticks: 0,
            walker: Walker::new(start_position),
            pending_stamps: vec![start_position],
        }
    }

    fn fixed_update(&mut self, ctx: &SketchUpdateContext) {
        self.fixed_ticks += 1;

        if self.fixed_ticks % DIRECTION_CHANGE_EVERY_N_TICKS == 0 {
            self.walker.change_direction();
        }

        self.walker.step(ctx.canvas_size);
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

enum WalkerDirection {
    Left,
    Right,
    Up,
    Down,
}

struct Walker {
    position: Vec2,
    direction: WalkerDirection,
}

impl Walker {
    const fn new(position: Vec2) -> Self {
        Self {
            position,
            direction: WalkerDirection::Left,
        }
    }

    fn change_direction(&mut self) {
        self.direction = match cg_lab::random::range_u32(0, 4) {
            0 => WalkerDirection::Left,
            1 => WalkerDirection::Right,
            2 => WalkerDirection::Up,
            3 => WalkerDirection::Down,
            _ => unreachable!("direction must be in 0..4"),
        };
    }

    fn step(&mut self, canvas_size: Vec2) {
        match self.direction {
            WalkerDirection::Left => self.position.x -= WALKER_STEP_SIZE,
            WalkerDirection::Right => self.position.x += WALKER_STEP_SIZE,
            WalkerDirection::Up => self.position.y -= WALKER_STEP_SIZE,
            WalkerDirection::Down => self.position.y += WALKER_STEP_SIZE,
        }

        let half_dot = WALKER_DOT_SIZE * 0.5;

        self.position.x = self.position.x.clamp(half_dot, canvas_size.x - half_dot);
        self.position.y = self.position.y.clamp(half_dot, canvas_size.y - half_dot);
    }
}

fn main() {
    cg_lab::run::<AutonomousWalkerSketch>(AppConfig {
        title: "Autonomous Walker".to_string(),
        width: 800,
        height: 800,
    });
}
