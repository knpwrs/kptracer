use tuple::Tuple;
use ray::Ray;
use super::super::intersection::*;
use super::super::material::Material;

struct Sphere {
    origin: Tuple,
    radius: f64,
    mat: Material,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            origin: Tuple::point(0.0, 0.0, 0.0),
            radius: 1.0,
            mat: Material { },
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);
        let a = ray.direction.dot(&ray.direction);
        let b = (ray.direction.dot(&sphere_to_ray)) * 2.0;
        let c = (sphere_to_ray.dot(&sphere_to_ray)) - 1.0;
        let disciminant = b.powi(2) - 4.0 * a * c;
        if disciminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - disciminant.sqrt()) / (2.0 * a);
            let t2 = (-b + disciminant.sqrt()) / (2.0 * a);
            vec![Intersection { mat: self.mat, t: t1 }, Intersection { mat: self.mat, t: t2 }]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn create_sphere() {
        let s = Sphere::new();
        assert_eq!(s.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(s.radius, 1.0);
    }

    #[test]
    pub fn intersect() {
        let s = Sphere::new();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    pub fn intersect_tangent() {
        let s = Sphere::new();
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    pub fn intersect_miss() {
        let s = Sphere::new();
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    pub fn intersect_inside_sphere() {
        let s = Sphere::new();
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    pub fn intersect_sphere_behind_ray() {
        let s = Sphere::new();
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }
}