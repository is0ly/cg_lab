use std::time::Duration;

use crate::{draw::DrawList, math::Vec2, random, render_data::Color};

const WALKER_STEP_SIZE: f32 = 2.0;
const WALKER_DOT_SIZE: f32 = 1.0;

// 60 fixed updates в секунду / 60 = 1 шаг walker'а в секунду.
const WALKER_STEP_EVERY_N_TICKS: u64 = 1;

pub struct Sketch {
    fixed_ticks: u64,
    canvas_size: Vec2,
    walker: Walker,
}

impl Sketch {
    #[must_use]
    pub fn new(canvas_size: Vec2) -> Self {
        Self {
            fixed_ticks: 0,
            canvas_size,
            walker: Walker::new(Vec2::new(canvas_size.x * 0.5, canvas_size.y * 0.5)),
        }
    }

    pub fn fixed_update(&mut self, _dt: Duration) {
        self.fixed_ticks += 1;

        if self.fixed_ticks % WALKER_STEP_EVERY_N_TICKS == 0 {
            self.walker.step(self.canvas_size);
        }
    }

    pub fn draw(&self, draw: &mut DrawList) {
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
    #[must_use]
    pub const fn new(position: Vec2) -> Self {
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
