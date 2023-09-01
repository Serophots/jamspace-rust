use sfml::system::Vector2f;

pub trait UIPositionable {
    fn get_position(&self) -> UDim2;
    fn set_position(&mut self, position: UDim2);
}
pub trait UISizeable {
    fn get_size(&self) -> UDim2;
    fn set_size(&mut self, size: UDim2);
}

//Private, immutable fields
#[derive(Copy, Clone)]
pub struct UDim {
    // scale: f32,
    // offset: f32,
    absolute: f32,
}
impl UDim {
    pub fn default() -> Self {
        Self {
            // scale: 0.0,
            // offset: 0.0,
            absolute: 0.0,
        }
    }

    pub fn from_absolute(absolute: f32) -> Self {
        Self {
            // scale: 0.0,
            // offset: 0.0,
            absolute,
        }
    }
}

//Private, immutable fields
#[derive(Copy, Clone)]
pub struct UDim2 {
    x: UDim,
    y: UDim
}
impl UDim2 {
    pub fn default() -> Self {
        Self {
            x: UDim::default(),
            y: UDim::default(),
        }
    }

    pub fn from_absolute((x_absolute, y_absolute): (f32, f32)) -> Self {
        Self {
            x: UDim::from_absolute(x_absolute),
            y: UDim::from_absolute(y_absolute),
        }
    }

    pub fn get_absolute(&self) -> Vector2f {
        Vector2f::new(self.x.absolute, self.y.absolute)
    }

    pub fn position_from_parent((x_scale, x_offset): (f32, f32), (y_scale, y_offset): (f32, f32), parent: &(impl UIPositionable + UISizeable)) -> Self {
        Self {
            x: UDim {
                // scale: x_scale,
                // offset: x_offset,
                absolute: x_offset + parent.get_position().x.absolute + (x_scale * parent.get_size().x.absolute),
            },
            y: UDim {
                // scale: y_scale,
                // offset: y_offset,
                absolute: y_offset + parent.get_position().y.absolute + (y_scale * parent.get_size().y.absolute),
            }
        }
    }
    pub fn size_from_parent((x_scale, x_offset): (f32, f32), (y_scale, y_offset): (f32, f32), parent: &impl UISizeable) -> Self {
        Self {
            x: UDim {
                // scale: x_scale,
                // offset: x_offset,
                absolute: x_offset + (x_scale * parent.get_size().x.absolute)
            },
            y: UDim {
                // scale: y_scale,
                // offset: y_offset,
                absolute: y_offset + (y_scale * parent.get_size().y.absolute)
            }
        }
    }
}