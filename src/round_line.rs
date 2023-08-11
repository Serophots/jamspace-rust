use sfml::graphics::{Color, Vertex};
use sfml::system::Vector2f;

const NUM_OF_ROUNDED_POINTS: i32 = 16;

fn do_segments_intersect((a, b): (Vector2f, Vector2f), (c,d): (Vector2f, Vector2f)) -> bool {
    //no co-linearity, thanks stackoverflow
    let ccw = |a: Vector2f, b: Vector2f, c: Vector2f| (c.y-a.y) * (b.x-a.x) > (b.y-a.y) * (c.x-a.x);
    ccw(a,c,d) != ccw(b,c,d) && ccw(a,b,c) != ccw(a,b,d)
}

fn normalize_vector(vector: Vector2f) -> Vector2f {
    let len = vector.length_sq().sqrt(); //.sqrt() is supposedly optimised into a single cpu instruction. No requirement for anything fancy
    Vector2f::new(vector.x / len, vector.y / len)
}

fn compute_gradient(from: Vector2f, to: Vector2f) -> f32 {
    -(to.y - from.y) / (to.x - from.x) //We have to negate the Y because the game is top-left BUT THE LITERAL ENTIRETY OF THE REST OF MATHS IS BOTTOM LEFT!!
}


struct RoundedLineVertex {
    position: Vector2f,
    a: Vector2f, //a,b are the two computed rounded positions. For a connecting line that is the acute pos and obtuse pos
    b: Vector2f, //For terminating line that's the starting and ending vertexes of the semi-circle
}

impl RoundedLineVertex {
    pub fn new_terminating(rounded_points: &mut Vec<Vertex>, line_weight: u32, terminating_vertex: Vertex, too_vertex: Vertex, previous_rounded_line_vertex: Option<&RoundedLineVertex>) -> Self {
        //Creating a semi-circle of points about the terminating_vertex of the line
        let terminating_vector = terminating_vertex.position - too_vertex.position;
        let gradient = compute_gradient(terminating_vertex.position, too_vertex.position);
        #[allow(unused_assignments)] //Assignment to = 0.; is never read
            let mut direction = 0.;
        let mut perpendicular_angle = 0.;

        //Figure out semi-circle direction based on line direction
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

        let get_position_from_theta = |theta: f32| terminating_vertex.position + Vector2f::new(theta.cos() * line_weight as f32, -theta.sin() * line_weight as f32); //-y because games use top-left coordinate system
        let step = direction * std::f32::consts::PI / NUM_OF_ROUNDED_POINTS as f32;

        let mut a = get_position_from_theta(perpendicular_angle);
        let mut b = get_position_from_theta(perpendicular_angle + std::f32::consts::PI);

        if let Some(previous) = previous_rounded_line_vertex {
            //Swap A & B if they intersect with the previous vertex's a & b. Not necessary if there is no previous
            if do_segments_intersect((previous.a, a), (previous.b, b)) {
                let ac = a;
                a = b;
                b = ac;
            }

            //Write some points to catch the line up to its end
            // Last center, new A, new B, last B, last A,   current A to begin next cycle
            rounded_points.push(Vertex::new(
                previous.position,
                Color::RED,
                terminating_vertex.tex_coords,
            ));
            rounded_points.push(Vertex::new(
                a,
                Color::RED,
                terminating_vertex.tex_coords,
            ));
            rounded_points.push(Vertex::new(
                b,
                Color::RED,
                terminating_vertex.tex_coords,
            ));
            rounded_points.push(Vertex::new(
                previous.b,
                Color::RED,
                terminating_vertex.tex_coords,
            ));
            rounded_points.push(Vertex::new(
                previous.a,
                Color::RED,
                terminating_vertex.tex_coords,
            ));
            rounded_points.push(Vertex::new(
                a,
                Color::RED,
                terminating_vertex.tex_coords,
            ));
        }

        //Compute semi-circle
        for i in 0..NUM_OF_ROUNDED_POINTS+1 {
            let theta = perpendicular_angle + (step * i as f32);
            let position = get_position_from_theta(theta);

            rounded_points.push(Vertex::new(
                position,
                Color::RED,
                terminating_vertex.tex_coords
            ));
            rounded_points.push(Vertex::new(
                terminating_vertex.position,
                Color::RED,
                terminating_vertex.tex_coords
            ));
        }

        rounded_points.push(Vertex::new(
            a,
            Color::RED,
            terminating_vertex.tex_coords,
        ));

        Self {
            position: terminating_vertex.position,
            a,
            b,
        }
    }
    pub fn new_connecting(rounded_points: &mut Vec<Vertex>, line_weight: u32, connecting_vertex: Vertex, from_vertex: Vertex, too_vertex: Vertex, previous_rounded_line_vertex: &RoundedLineVertex) -> Option<Self> {
        let outgoing_from_vector = normalize_vector(from_vertex.position - connecting_vertex.position);
        let outgoing_to_vector = normalize_vector(too_vertex.position - connecting_vertex.position);
        let bisecting_vector_acute = normalize_vector(outgoing_from_vector + outgoing_to_vector) * line_weight as f32;
        let bisecting_vector_obtuse = -bisecting_vector_acute;

        if (outgoing_from_vector + outgoing_to_vector).length_sq() == 0. {
            return None; //Straight line - doesn't need marking out
        }

        //Set a,b such that they pair with last a,b in that they're on the same side of the line
        let mut a = connecting_vertex.position + bisecting_vector_acute;
        let mut b = connecting_vertex.position + bisecting_vector_obtuse;
        if do_segments_intersect((previous_rounded_line_vertex.a, a), (previous_rounded_line_vertex.position, connecting_vertex.position)) {
            a = connecting_vertex.position + bisecting_vector_obtuse;
            b = connecting_vertex.position + bisecting_vector_acute;
        }

        //We are at the previous defining points position.
        // Last center, new A, new B, last B, last A,   current A to begin next cycle
        rounded_points.push(Vertex::new(
            previous_rounded_line_vertex.position,
            Color::RED,
            connecting_vertex.tex_coords,
        ));
        rounded_points.push(Vertex::new(
            a,
            Color::RED,
            connecting_vertex.tex_coords,
        ));
        rounded_points.push(Vertex::new(
            b,
            Color::RED,
            connecting_vertex.tex_coords,
        ));
        rounded_points.push(Vertex::new(
            previous_rounded_line_vertex.b,
            Color::RED,
            connecting_vertex.tex_coords,
        ));
        rounded_points.push(Vertex::new(
            previous_rounded_line_vertex.a,
            Color::RED,
            connecting_vertex.tex_coords,
        ));
        rounded_points.push(Vertex::new(
            a,
            Color::RED,
            connecting_vertex.tex_coords,
        ));

        Some(Self {
            position: connecting_vertex.position,
            a,
            b,
        })
    }
}

pub struct RoundedLine {
    defining_points: Vec<RoundedLineVertex>,
    pub(crate) rounded_points: Vec<Vertex>,
    line_weight: u32,
}

impl RoundedLine {
    pub fn new(defining_vertexes: Vec<Vertex>, line_weight: u32) -> Option<Self> {
        let mut defining_points: Vec<RoundedLineVertex> = Vec::with_capacity(defining_vertexes.len());
        let mut rounded_points: Vec<Vertex> = Vec::with_capacity(defining_vertexes.len()); //TODO at least this capacity

        if defining_vertexes.len() < 2 {
            return None;
        }

        for i in 0..defining_vertexes.len() {
            if i == 0 {
                //Terminating vertex (start)
                let terminating_vertex = defining_vertexes[i];
                let too_vertex = defining_vertexes[i + 1];

                defining_points.push(RoundedLineVertex::new_terminating(&mut rounded_points, line_weight, terminating_vertex, too_vertex, None));
            } else if i == defining_vertexes.len()-1 {
                //Terminating vertex (end) + Connecting vertex so that things line up
                let terminating_vertex = defining_vertexes[i];
                let too_vertex = defining_vertexes[i - 1];

                defining_points.push(RoundedLineVertex::new_terminating(&mut rounded_points, line_weight, terminating_vertex, too_vertex, defining_points.last()));
            } else {
                //Connecting vertex
                let from_vertex = defining_vertexes[i-1];
                let connecting_vertex = defining_vertexes[i];
                let too_vertex = defining_vertexes[i+1];


                match RoundedLineVertex::new_connecting(&mut rounded_points, line_weight, connecting_vertex, from_vertex, too_vertex, defining_points.last().expect("Internal failure when computing rounded line")) {
                    Some(rounded_line_vertex) => {
                        defining_points.push(rounded_line_vertex);
                    }
                    None => {}
                }
            }
        }

        Some(Self {
            defining_points,
            rounded_points,
            line_weight,
        })
    }
}
