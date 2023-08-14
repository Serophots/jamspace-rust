use sfml::graphics::{Color, Vertex};
use sfml::system::Vector2f;
use crate::tools::DrawingTool;
use crate::tools::utils::{compute_gradient, do_segments_intersect, normalize_vector};

const SEMICIRCLE_POINTS: usize = 16;

pub struct RoundedLine { //TODO: Put some of these on the heap
    defining_points: Vec<RoundedLineVertex>,
    rendered_vertexes: Vec<Vertex>,
    line_weight: u32, //Line thickness
}

//TODO: If newA-newB intersects oldA-oldB then do a 360 degree
// or more refined, if either newA,newB fall behind oldA,oldB

impl DrawingTool for RoundedLine {
    fn new(defining_points: Vec<Vector2f>, color: Color) -> Self {
        let mut processed_defining_points: Vec<RoundedLineVertex> = Vec::with_capacity(defining_points.len());
        let defining_points_len = defining_points.len();
        let line_weight = 10;

        //Process each of the defining points as either: A terminating vertex, or a connecting vertex
        for i in 0..defining_points_len {
            if i == 0 {
                //Terminating vertex (start)
                let terminating_point = defining_points[i];
                let too_point = defining_points[i + 1];

                processed_defining_points.push(RoundedLineVertex::new_terminating(line_weight, terminating_point, too_point, None));
            } else if i == defining_points_len-1 {
                //Terminating vertex (end) + Connecting vertex so that things line up
                let terminating_point = defining_points[i];
                let too_point = defining_points[i - 1];

                processed_defining_points.push(RoundedLineVertex::new_terminating(line_weight, terminating_point, too_point, processed_defining_points.last()));
            } else {
                //Connecting vertex
                let from_point = defining_points[i-1];
                let connecting_point = defining_points[i];
                let too_point = defining_points[i+1];

                match RoundedLineVertex::new_connecting(line_weight, connecting_point, from_point, too_point, processed_defining_points.last().unwrap()) {
                    Some(rounded_line_vertex) => {
                        processed_defining_points.push(rounded_line_vertex);
                    }
                    None => {}
                }
            }
        }

        let mut s = Self {
            defining_points: processed_defining_points,
            rendered_vertexes: Vec::new(), //with capacity
            line_weight,
        };
        s.rerender_vertexes(color);

        s
    }

    fn rerender_vertexes(&mut self, color: Color) {
        let tex_coords = Vector2f::default();
        self.rendered_vertexes.clear();

        let plot_semicircle = |vertex: &RoundedLineVertex, rendered_vertexes: &mut Vec<Vertex>| {
            let step = vertex.direction * std::f32::consts::PI / SEMICIRCLE_POINTS as f32;

            for i in 0..SEMICIRCLE_POINTS + 1 {
                let theta = vertex.perpendicular_angle + (step * i as f32);

                rendered_vertexes.push(Vertex::new(
                    vertex.point + Vector2f::new(theta.cos() * self.line_weight as f32, -theta.sin() * self.line_weight as f32), //-y because games use top-left coordinate system
                    color,
                    tex_coords
                ));
                rendered_vertexes.push(Vertex::new(
                    vertex.point,
                    color,
                    tex_coords
                ));
            }
        };

        plot_semicircle(&self.defining_points.get(0).unwrap(), &mut self.rendered_vertexes);

        //Connecting points
        for rounded_line_vertex in &self.defining_points {
            self.rendered_vertexes.push(Vertex::new(
                rounded_line_vertex.a,
                color,
                tex_coords
            ));
            self.rendered_vertexes.push(Vertex::new(
                rounded_line_vertex.b,
                color,
                tex_coords
            ));
        }

        plot_semicircle(&self.defining_points.get(self.defining_points.len()-1).unwrap(), &mut self.rendered_vertexes);
    }

    fn get_rendered_vertexes(&self) -> &Vec<Vertex> {
        &self.rendered_vertexes
    }
}

pub struct RoundedLineVertex {
    point: Vector2f,
    a: Vector2f, //a and b are computed positions along the bisectors of a line intersection
    b: Vector2f, //OR the top and bottom defining points of a terminating semi-circle
    perpendicular_angle: f32,
    direction: f32,
}

impl RoundedLineVertex {
    pub fn new_terminating(line_weight: u32, terminating_point: Vector2f, too_point: Vector2f, previous_rounded_point: Option<&RoundedLineVertex>) -> Self {
        //Creating a semi-circle of points about the terminating_vertex of the line
        let terminating_vector = terminating_point - too_point;
        let gradient = compute_gradient(terminating_point, too_point);
        let direction: f32;
        let mut perpendicular_angle = 0.;

        //Direction = 1 for clockwise, -1 for anticlockwise:
        if terminating_vector.y == 0. {
            //Special case for horizontal straight
            perpendicular_angle = std::f32::consts::FRAC_PI_2;
            direction = if terminating_vector.x > 0. {-1.} else {1.};
        } else if terminating_vector.x == 0. {
            //Special case for vertical straight
            direction = if terminating_vector.y > 0. {-1.} else {1.};
        }else{
            direction = (gradient / gradient.abs()) * (terminating_vector.x / terminating_vector.x.abs());
            perpendicular_angle = (-1. / gradient).atan();
        }

        let get_position_from_theta = |theta: f32| terminating_point + Vector2f::new(theta.cos() * line_weight as f32, -theta.sin() * line_weight as f32); //-y because games use top-left coordinate system

        //Define a,b
        let mut a = get_position_from_theta(perpendicular_angle);
        let mut b = get_position_from_theta(perpendicular_angle + std::f32::consts::PI);

        //Swap a,b if they intersect with a previous a,b
        if let Some(previous) = previous_rounded_point {
            if do_segments_intersect((previous.a, a), (previous.b, b)) {
                let ac = a;
                a = b;
                b = ac;
            }
        }

        Self {
            point: terminating_point,
            a,
            b,
            perpendicular_angle,
            direction
        }
    }

    pub fn new_connecting(line_weight: u32, connecting_point: Vector2f, from_point: Vector2f, too_point: Vector2f, previous_rounded_point: &RoundedLineVertex) -> Option<Self> {
        let outgoing_from_vector = normalize_vector(from_point - connecting_point);
        let outgoing_to_vector = normalize_vector(too_point - connecting_point);
        let bisecting_vector_acute = normalize_vector(outgoing_from_vector + outgoing_to_vector) * line_weight as f32;
        let bisecting_vector_obtuse = -bisecting_vector_acute;

        //TODO: Better method to check straight line. Currently requires two extra normalizations:
        if (outgoing_from_vector + outgoing_to_vector).length_sq() == 0. {
            return None; //Straight line - doesn't need marking out
        }

        if outgoing_from_vector.cross(outgoing_to_vector) == 0. {
            //Forms a straight line or otherwise has no bisectors
            return None;
        }

        //Set a,b such that they pair with last a,b in that they're on the same side of the line
        let mut a = connecting_point + bisecting_vector_acute;
        let mut b = connecting_point + bisecting_vector_obtuse;
        if do_segments_intersect((previous_rounded_point.a, a), (previous_rounded_point.point, connecting_point)) {
            a = connecting_point + bisecting_vector_obtuse;
            b = connecting_point + bisecting_vector_acute;
        }

        Some(Self {
            point: connecting_point,
            a,
            b,
            perpendicular_angle: 0.,
            direction: 0.,
        })
    }
}