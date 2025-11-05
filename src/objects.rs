#[derive(Copy, Clone)]
pub struct Vertex
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vertex
{
    pub fn new(x: f64, y: f64, z: f64) -> Vertex
    {
        Vertex { x, y, z }
    }
}

pub struct Face
{
    vertices: Vec<Vertex>,
    color: u32
}

impl Face
{
    pub fn new(vertices:Vec<Vertex>, color: u32) -> Face
    {
        Face{vertices, color}
    }



    pub fn set_color(&mut self, color:u32)
    {
        self.color = color;
    }
}

struct Coordinates
{
    x:u32,
    y:u32,
    z:u32
}
