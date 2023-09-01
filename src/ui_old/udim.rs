use sfml::system::Vector2f;

#[derive(Copy, Clone)]
pub struct UDim {
    scale: f32, //0-1 relative to parent
    offset: f32, //absolute pixel offset
}
impl UDim {
    pub fn new(scale: f32, offset: f32) -> Self {
        Self { scale, offset }
    }

    pub fn compute_absolute(&self, parent: f32) -> f32 {
        parent * self.scale + self.offset
    }
}

#[derive(Copy, Clone)]
pub struct UDim2 {
    x: UDim,
    y: UDim,
}
impl UDim2 {
    pub fn new((x_scale, x_offset): (f32, f32), (y_scale, y_offset): (f32, f32)) -> Self {
        Self {
            x: UDim::new(x_scale, x_offset),
            y: UDim::new(y_scale, y_offset),
        }
    }

    pub fn compute_absolute_size(&self, absolute_parent_size: Vector2f) -> Vector2f {
        Vector2f::new(self.x.compute_absolute(absolute_parent_size.x), self.y.compute_absolute(absolute_parent_size.y))
    }

    pub fn compute_absolute_pos(&self, absolute_parent_position: Vector2f, absolute_parent_size: Vector2f) -> Vector2f {
        absolute_parent_position + Vector2f::new(self.x.compute_absolute(absolute_parent_size.x),self.y.compute_absolute(absolute_parent_size.y))
    }
}