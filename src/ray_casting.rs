use crate::objects::Vertex;
use crate::math_lib::dot;
#[derive(Copy, Clone)]
pub struct Ray
{
    origin: Vertex,
    direction: Vertex
}

impl Ray
{
    pub fn new(origin: Vertex, direction: Vertex) -> Ray
    {
        Ray {origin, direction}
    }
}

pub struct HitInfo {
    pub hit: bool,
    pub color: u32,
    pub distance: f64,
    pub normal: Vertex,
}


impl HitInfo
{
    pub fn new(hit:bool, color:u32, distance: f64, normal: Vertex) -> Self
    {
        HitInfo{hit, color, distance, normal}
    }
}

pub struct Sphere
{
    pub center: Vertex,
    pub radius: f64,
    pub color: u32
}

impl Sphere {
    pub fn new(center: Vertex, radius: f64, color: u32) -> Self {
        Self { center, radius, color }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        let oc = Vertex {
            x: ray.origin.x - self.center.x,
            y: ray.origin.y - self.center.y,
            z: ray.origin.z - self.center.z,
        };

        let a = dot(&ray.direction, &ray.direction);
        let b = 2.0 * dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t < 0.0 {
                return None;
            }

            // Intersection point
            let hit_point = Vertex {
                x: ray.origin.x + ray.direction.x * t,
                y: ray.origin.y + ray.direction.y * t,
                z: ray.origin.z + ray.direction.z * t,
            };

            // Normal at intersection
            let normal = Vertex {
                x: (hit_point.x - self.center.x) / self.radius,
                y: (hit_point.y - self.center.y) / self.radius,
                z: (hit_point.z - self.center.z) / self.radius,
            };

            Some(HitInfo {
                hit: true,
                color: self.color,
                distance: t,
                normal,
            })
        }
    }
}
