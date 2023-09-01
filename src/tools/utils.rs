use sfml::system::Vector2f;

pub fn does_segment_intersect_segment((a, b): (Vector2f, Vector2f), (c,d): (Vector2f, Vector2f)) -> bool {
    //no co-linearity. thanks stackoverflow
    let ccw = |a: Vector2f, b: Vector2f, c: Vector2f| (c.y-a.y) * (b.x-a.x) > (b.y-a.y) * (c.x-a.x);
    ccw(a,c,d) != ccw(b,c,d) && ccw(a,b,c) != ccw(a,b,d)
}
pub fn does_segment_intersect_line((segment_a, segment_b): (Vector2f, Vector2f), (line_c, line_d): (Vector2f, Vector2f)) -> bool {
    //infinite line, finite segment. maths exchange
    ((line_d - line_c).cross(segment_a - line_c))*((line_d - line_c).cross(segment_b - line_c)) <= 0.
}
pub fn normalize_vector(vector: Vector2f) -> Vector2f {
    let len = vector.length_sq().sqrt(); //.sqrt() is supposedly optimised into a single cpu instruction. No requirement for anything fancy
    Vector2f::new(vector.x / len, vector.y / len)
}
pub fn compute_gradient(from: Vector2f, to: Vector2f) -> f32 {
    -(to.y - from.y) / (to.x - from.x) //We have to negate the Y because the game is top-left BUT THE LITERAL ENTIRETY OF THE REST OF MATHS IS BOTTOM LEFT!!
}