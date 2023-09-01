pub mod udim;
pub mod ui_rectangle;

use sfml::graphics::{Drawable};
use crate::ui_lib::udim::{UDim2, UIPositionable, UISizeable};

pub trait UIElement: Drawable + UIPositionable + UISizeable {}

pub struct UIScreen {
    screen_size: UDim2,
}
impl UIScreen {
    pub fn new(window_size: (f32, f32)) -> Self {
        Self {
            screen_size: UDim2::from_absolute(window_size),
        }
    }
}

impl UIPositionable for UIScreen {
    fn get_position(&self) -> UDim2 {
        UDim2::default()
    }

    fn set_position(&mut self, _position: UDim2) {
        unreachable!()
    }
}

impl UISizeable for UIScreen {
    fn get_size(&self) -> UDim2 {
        self.screen_size
    }

    fn set_size(&mut self, _size: UDim2) {
        unreachable!()
    }
}