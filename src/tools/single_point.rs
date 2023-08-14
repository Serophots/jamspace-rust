use sfml::graphics::{Color, Vertex};
use sfml::system::Vector2f;
use crate::tools::DrawingTool;

const CIRCLE_POINTS: usize = 32;

pub struct SinglePoint {
    point: Vector2f,
    rendered_points: Vec<Vector2f>,
    rendered_vertexes: Vec<Vertex>,
    point_weight: u32, //Point radius
}

impl DrawingTool for SinglePoint {
    fn new(defining_points: Vec<Vector2f>, color: Color) -> Self where Self: Sized {
        let point = *defining_points.get(0).unwrap();
        let mut rendered_points: Vec<Vector2f> = Vec::with_capacity(CIRCLE_POINTS * 2);

        let point_weight: u32 = 10;

        for i in 0..CIRCLE_POINTS + 1 {
            let theta = i as f32 * std::f32::consts::TAU / CIRCLE_POINTS as f32;
            let position = point + Vector2f::new(theta.cos() * point_weight as f32, -theta.sin() * point_weight as f32); //-y because games use top-left coordinate system

            rendered_points.push(position);
            rendered_points.push(point);
        }

        let mut s = Self {
            rendered_vertexes: Vec::with_capacity(rendered_points.len()),
            point,
            rendered_points,
            point_weight,
        };
        s.rerender_vertexes(color);

        s
    }

    fn rerender_vertexes(&mut self, color: Color) {
        let tex_coords = Vector2f::default();

        self.rendered_vertexes.clear();
        for rendered_point in &self.rendered_points {
            self.rendered_vertexes.push(Vertex::new(
                *rendered_point,
                color,
                tex_coords
            ))
        }
    }

    fn get_rendered_vertexes(&self) -> &Vec<Vertex> {
        &self.rendered_vertexes
    }
}