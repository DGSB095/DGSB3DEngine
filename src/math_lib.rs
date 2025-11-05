use crate::objects::Vertex;
pub fn dot(a: &Vertex, b: &Vertex) -> f64
{
    a.x * b.x + a.y * b.y + a.z * b.z
}