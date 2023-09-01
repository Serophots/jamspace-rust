use sfml::graphics::{Color, Drawable, RectangleShape, RenderStates, RenderTarget, Shape, Transformable};
use sfml::system::Vector2f;
use crate::ui_old::udim::UDim2;
use crate::ui_old::UIElement;

pub struct Frame<'s> {
    rectangle: RectangleShape<'s>,
    abs_parent_position: Vector2f,
    abs_parent_size: Vector2f,
    position: UDim2,
    size: UDim2,
    pub(crate) abs_position: Vector2f,
    pub(crate) abs_size: Vector2f,
    pub(crate) visible: bool,
}

impl<'s> Frame<'s> {
    pub fn new(position: UDim2, size: UDim2, abs_parent_position: Vector2f, abs_parent_size: Vector2f, color: Color) -> Self {
        let mut rectangle = RectangleShape::new();
        rectangle.set_fill_color(color);

        let mut this = Self {
            rectangle,
            position,
            size,
            abs_parent_position,
            abs_parent_size,
            abs_position: Vector2f::default(),
            abs_size: Vector2f::default(),
            visible: true
        };
        this.update_scaling();

        return this;
    }
}
impl<'s> UIElement for Frame<'s> {
    fn set_fill_color(&mut self, color: Color) {
        self.rectangle.set_fill_color(color);
    }

    fn get_sizing(&self) -> Vector2f {
        self.abs_size
    }
    fn get_positioning(&self) -> Vector2f {
        self.abs_position
    }
    fn update_sizing(&mut self) -> Vector2f {
        self.abs_size = self.size.compute_absolute_size(self.abs_parent_size);
        self.rectangle.set_size(self.abs_size);
        self.abs_size
    }
    fn update_positioning(&mut self) -> Vector2f {
        self.abs_position = self.position.compute_absolute_pos(self.abs_parent_position, self.abs_parent_size);
        self.rectangle.set_position(self.abs_position);
        self.abs_position
    }
    fn update_scaling(&mut self) {
        self.update_sizing();
        self.update_positioning();
    }
}

impl<'s> Drawable for Frame<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>
    ) {
        if self.visible {
            target.draw_with_renderstates(&self.rectangle, states);
        }
    }
}