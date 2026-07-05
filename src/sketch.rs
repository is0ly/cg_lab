use std::time::Duration;

use crate::{draw::DrawList, input::Input, math::Vec2};

pub struct SketchInitContext {
    pub canvas_size: Vec2,
}

pub struct SketchUpdateContext<'a> {
    pub fixed_delta: Duration,
    pub canvas_size: Vec2,
    pub input: &'a Input,
}

pub struct SketchDrawContext<'a> {
    pub canvas_size: Vec2,
    pub input: &'a Input,
}

pub trait Sketch {
    fn new(ctx: &SketchInitContext) -> Self
    where
        Self: Sized;

    fn fixed_update(&mut self, ctx: &SketchUpdateContext);

    fn draw(&mut self, draw: &mut DrawList, ctx: &SketchDrawContext);
}
