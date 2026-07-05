use std::mem::size_of;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub const RED: Self = Self::rgb(1.0, 0.2, 0.2);
    pub const GREEN: Self = Self::rgb(0.2, 1.0, 0.2);
    pub const BLUE: Self = Self::rgb(0.2, 0.4, 1.0);
    pub const YELLOW: Self = Self::rgb(1.0, 1.0, 0.2);
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);

    #[must_use]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    #[must_use]
    pub const fn to_array(self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColoredVertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

impl ColoredVertex {
    pub(crate) const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![
            0 => Float32x2,
            1 => Float32x3,
        ],
    };

    #[must_use]
    pub const fn new(position: [f32; 2], color: Color) -> Self {
        Self {
            position,
            color: color.to_array(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColoredMesh {
    vertices: Vec<ColoredVertex>,
    indices: Vec<u16>,
}

impl ColoredMesh {
    #[must_use]
    pub fn new(vertices: Vec<ColoredVertex>, indices: Vec<u16>) -> Self {
        Self { vertices, indices }
    }

    #[must_use]
    pub fn vertices(&self) -> &[ColoredVertex] {
        &self.vertices
    }

    #[must_use]
    pub fn indices(&self) -> &[u16] {
        &self.indices
    }

    #[must_use]
    pub fn index_count(&self) -> u32 {
        u32::try_from(self.indices.len()).expect("colored mesh index count must fit in u32")
    }
}
