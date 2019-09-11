use super::ray::Ray;
use super::material::Material;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
}

pub struct Intersection {
    pub mat: Material,
    pub t: f64,
}