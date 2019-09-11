use super::tuple::Tuple;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + (self.direction * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn create_ray() {
        let p = Tuple::point(2.0, 3.0, 4.0);
        let v = Tuple::vector(1.0, 0.0, 0.0);
        let r = Ray::new(p, v);
        assert_eq!(r.origin, p);
        assert_eq!(r.direction, v);
    }

    #[test]
    pub fn position() {
        let r = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }
}