use sfml::graphics::{Color, Drawable, RenderStates, RenderTarget};
use sfml::system::Vector2f;

pub mod frame;
pub mod udim;
pub mod button;

pub trait UIElement: Drawable {
    fn set_fill_color(&mut self, color: Color);
    fn get_sizing(&self) -> Vector2f;
    fn get_positioning(&self) -> Vector2f;

    fn update_sizing(&mut self) -> Vector2f;
    fn update_positioning(&mut self) -> Vector2f;
    fn update_scaling(&mut self);
}

pub struct UIBuilder {
    pub(crate) registered_elements: Vec<Box<dyn UIElement>>
}
impl UIBuilder {
    pub fn new() -> Self { Self { registered_elements: Vec::new()} }
    pub fn add_element(&mut self, element: Box<dyn UIElement>) -> &Box<dyn UIElement> {
        //Dynamic or static dispatch?
        self.registered_elements.push(element);
        &self.registered_elements.last().unwrap()
    }
}

impl Drawable for UIBuilder {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>
    ) {
        for registered_element in &self.registered_elements {
            registered_element.draw(target, states);
        }
    }
}