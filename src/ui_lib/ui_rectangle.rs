use sfml::graphics::{Color, Drawable, RectangleShape, RenderStates, RenderTarget, Shape, Transformable};
use crate::ui_lib::udim::{UDim2, UIPositionable, UISizeable};

pub struct UIRectangle<'s> {
    rectangle: RectangleShape<'s>,
    position: UDim2,
    size: UDim2,
}
impl<'s> UIRectangle<'s> {
    pub fn new(position: UDim2, size: UDim2, color: Color) -> Self {
        let mut rectangle = RectangleShape::new();
        rectangle.set_fill_color(color);

        rectangle.set_position(position.get_absolute());
        rectangle.set_size(size.get_absolute());

        Self {
            rectangle,
            position,
            size,
        }
    }
}

impl<'s> Drawable for UIRectangle<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(&'a self, target: &mut dyn RenderTarget, states: &RenderStates<'texture, 'shader, 'shader_texture>) {
        target.draw_with_renderstates(&self.rectangle, states);
    }
}
impl<'s> UIPositionable for UIRectangle<'s> {
    fn get_position(&self) -> UDim2 {
        self.position
    }

    fn set_position(&mut self, position: UDim2) {
        self.position = position;
    }
}
impl<'s> UISizeable for UIRectangle<'s> {
    fn get_size(&self) -> UDim2 {
        self.size
    }

    fn set_size(&mut self, size: UDim2) {
        self.size = size;
    }
}