use sfml::graphics::{Color, Vertex};
use sfml::system::Vector2f;

pub mod rounded_line;
// pub mod single_point;
pub mod utils;
pub mod single_point;

pub trait DrawingTool {
    fn new(defining_points: Vec<Vector2f>, color: Color) -> Self where Self: Sized;
    fn rerender_vertexes(&mut self, color: Color);
    fn get_rendered_vertexes(&self) -> &Vec<Vertex>;
}