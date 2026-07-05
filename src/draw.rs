use crate::{
    math::{Vec2, Viewport},
    render_data::{Color, ColoredMesh, ColoredVertex},
};

#[derive(Debug)]
pub struct DrawList {
    viewport: Viewport,
    vertices: Vec<ColoredVertex>,
    indices: Vec<u16>,
}

impl DrawList {
    #[must_use]
    pub fn new(viewport: Viewport) -> Self {
        Self {
            viewport,
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn triangle(&mut self, center: Vec2, size: f32, color: Color) {
        let half = size * 0.5;

        let top = Vec2::new(center.x, center.y - half);
        let left_bottom = Vec2::new(center.x - half, center.y + half);
        let right_bottom = Vec2::new(center.x + half, center.y + half);

        self.push_triangle(top, left_bottom, right_bottom, color);
    }

    pub fn rect(&mut self, center: Vec2, size: Vec2, color: Color) {
        let half_width = size.x * 0.5;
        let half_height = size.y * 0.5;

        let left_top = Vec2::new(center.x - half_width, center.y - half_height);
        let left_bottom = Vec2::new(center.x - half_width, center.y + half_height);
        let right_bottom = Vec2::new(center.x + half_width, center.y + half_height);
        let right_top = Vec2::new(center.x + half_width, center.y - half_height);

        self.push_quad(left_top, left_bottom, right_bottom, right_top, color);
    }

    pub fn line(&mut self, start: Vec2, end: Vec2, thickness: f32, color: Color) {
        if thickness <= 0.0 {
            return;
        }

        let dx = end.x - start.x;
        let dy = end.y - start.y;

        let length = dx.hypot(dy);

        if length <= f32::EPSILON {
            return;
        }

        let dir_x = dx / length;
        let dir_y = dy / length;

        let half_thickness = thickness * 0.5;

        let normal_x = -dy / length * half_thickness;
        let normal_y = dx / length * half_thickness;

        let cap_x = dir_x * half_thickness;
        let cap_y = dir_y * half_thickness;

        let extended_start = Vec2::new(start.x - cap_x, start.y - cap_y);
        let extended_end = Vec2::new(end.x + cap_x, end.y + cap_y);

        let start_left = Vec2::new(extended_start.x + normal_x, extended_start.y + normal_y);
        let start_right = Vec2::new(extended_start.x - normal_x, extended_start.y - normal_y);
        let end_right = Vec2::new(extended_end.x - normal_x, extended_end.y - normal_y);
        let end_left = Vec2::new(extended_end.x + normal_x, extended_end.y + normal_y);

        self.push_quad(start_left, start_right, end_right, end_left, color);
    }

    #[must_use]
    pub fn into_colored_mesh(self) -> ColoredMesh {
        ColoredMesh::new(self.vertices, self.indices)
    }

    fn push_triangle(&mut self, a: Vec2, b: Vec2, c: Vec2, color: Color) {
        let base_index = self.next_base_index();

        self.vertices
            .push(ColoredVertex::new(self.viewport.to_clip_space(a), color));
        self.vertices
            .push(ColoredVertex::new(self.viewport.to_clip_space(b), color));
        self.vertices
            .push(ColoredVertex::new(self.viewport.to_clip_space(c), color));

        self.indices
            .extend_from_slice(&[base_index, base_index + 1, base_index + 2]);
    }

    fn push_quad(
        &mut self,
        left_top: Vec2,
        left_bottom: Vec2,
        right_bottom: Vec2,
        right_top: Vec2,
        color: Color,
    ) {
        let base_index = self.next_base_index();

        self.vertices.push(ColoredVertex::new(
            self.viewport.to_clip_space(left_top),
            color,
        ));
        self.vertices.push(ColoredVertex::new(
            self.viewport.to_clip_space(left_bottom),
            color,
        ));
        self.vertices.push(ColoredVertex::new(
            self.viewport.to_clip_space(right_bottom),
            color,
        ));
        self.vertices.push(ColoredVertex::new(
            self.viewport.to_clip_space(right_top),
            color,
        ));

        self.indices.extend_from_slice(&[
            base_index,
            base_index + 1,
            base_index + 2,
            base_index,
            base_index + 2,
            base_index + 3,
        ]);
    }

    fn next_base_index(&self) -> u16 {
        u16::try_from(self.vertices.len()).expect("draw list vertex count must fit in u16")
    }
}
