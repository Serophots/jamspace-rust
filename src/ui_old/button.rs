use sfml::graphics::{Color, Drawable, RectangleShape, RenderStates, RenderTarget, Shape, Transformable};
use sfml::system::Vector2f;
use crate::ui_old::udim::UDim2;
use crate::ui_old::UIElement;

pub struct Button<'s> {
    rectangle: RectangleShape<'s>,
    on_hover: Box<dyn Fn()>,
    on_click: Box<dyn Fn()>,
    abs_parent_position: Vector2f,
    abs_parent_size: Vector2f,
    position: UDim2,
    size: UDim2,
    pub(crate) abs_position: Vector2f,
    pub(crate) abs_size: Vector2f,
    pub(crate) visible: bool,
}

impl <'s> Button<'s> {
    pub fn new(position: UDim2, size: UDim2, abs_parent_position: Vector2f, abs_parent_size: Vector2f, on_hover: impl Fn() + 'static, on_click: impl Fn() + 'static, color: Color) -> Self {
        let mut rectangle = RectangleShape::new();
        rectangle.set_fill_color(color);

        let mut this = Self {
            rectangle,
            position,
            size,
            abs_parent_position,
            abs_parent_size,
            on_hover: Box::new(on_hover),
            on_click: Box::new(on_click),
            abs_position: Vector2f::default(),
            abs_size: Vector2f::default(),
            visible: true
        };
        this.update_scaling();

        return this;
    }
}
impl<'s> UIElement for Button<'s> {
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

impl<'s> Drawable for Button<'s> {
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