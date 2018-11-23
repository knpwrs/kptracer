use std::ops;
use super::util;

#[derive(Debug)]
pub struct Tuple (f64, f64, f64, f64);

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple(x, y, z, 1.0)
    }

    pub fn is_point(&self) -> bool {
        self.3 == 1.0
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple(x, y, z, 0.0)
    }

    pub fn is_vector(&self) -> bool {
        self.3 == 0.0
    }

    pub fn magnitude(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2) + self.3.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();
        Tuple(self.0 / mag, self.1 / mag, self.2 / mag, self.3 / mag)
    }

    pub fn dot(&self, _rhs: &Tuple) -> f64 {
        (self.0 * _rhs.0) + (self.1 * _rhs.1) + (self.2 * _rhs.2) + (self.3 * _rhs.3)
    }

    // Assumes _rhs is a vector
    pub fn cross(&self, _rhs: &Tuple) -> Tuple {
        Tuple::vector(
          (self.1 * _rhs.2) - (self.2 * _rhs.1),
          (self.2 * _rhs.0) - (self.0 * _rhs.2),
          (self.0 * _rhs.1) - (self.1 * _rhs.0),
        )
    }
}

impl PartialEq<Tuple> for Tuple {
  fn eq(&self, other: &Tuple) -> bool {
      util::approx_eq(self.0, other.0)
      && util::approx_eq(self.1, other.1)
      && util::approx_eq(self.2, other.2)
      && util::approx_eq(self.3, other.3)
  }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, _rhs: Tuple) -> Tuple {
        Tuple(self.0 + _rhs.0, self.1 + _rhs.1, self.2 + _rhs.2, self.3 + _rhs.3)
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, _rhs: Tuple) -> Tuple {
        Tuple(self.0 - _rhs.0, self.1 - _rhs.1, self.2 - _rhs.2, self.3 - _rhs.3)
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, _rhs: f64) -> Tuple {
        Tuple(self.0 * _rhs, self.1 * _rhs, self.2 * _rhs, self.3 * _rhs)
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, _rhs: f64) -> Tuple {
        Tuple(self.0 / _rhs, self.1 / _rhs, self.2 / _rhs, self.3 / _rhs)
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple(0.0, 0.0, 0.0, 0.0) - self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_point() {
        let point = Tuple::point(4.3, -4.2, 3.1);
        assert_eq!(point, Tuple(4.3, -4.2, 3.1, 1.0));
        assert_eq!(point.is_point(), true);
        assert_eq!(point.is_vector(), false);
    }

    #[test]
    fn create_vector() {
        let vector = Tuple::vector(4.3, -4.2, 3.1);
        assert_eq!(vector, Tuple(4.3, -4.2, 3.1, 0.0));
        assert_eq!(vector.is_point(), false);
        assert_eq!(vector.is_vector(), true);
    }

    #[test]
    fn add_two_tuples() {
        let a1 = Tuple(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a1 + a2, Tuple(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn subtract_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Tuple(-2.0, -4.0, -6.0, 0.0));
    }

    #[test]
    fn subtract_vector_from_point() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_two_vectors() {
        let p1 = Tuple::vector(3.0, 2.0, 1.0);
        let p2 = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn multiply_by_scalar() {
        let a1 = Tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a1 * 3.5, Tuple(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiply_by_fraction() {
        let a1 = Tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a1 * 0.5, Tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn divide_by_scalar() {
        let a1 = Tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a1 / 2.0, Tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn negate_tuple() {
        let a1 = Tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a1, Tuple(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn magnitude() {
        let v1 = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v1.magnitude(), 1.0);
        let v2 = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(v2.magnitude(), 1.0);
        let v3 = Tuple::vector(0.0, 0.0, 1.0);
        assert_eq!(v3.magnitude(), 1.0);
        let v4 = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v4.magnitude(), (14.0 as f64).sqrt());
        let v5 = Tuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(v5.magnitude(), (14.0 as f64).sqrt());
    }

    #[test]
    fn normalize() {
        let v1 = Tuple::vector(4.0, 0.0, 0.0);
        assert_eq!(v1.normalize(), Tuple::vector(1.0, 0.0, 0.0));
        let v2 = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(
            v2.normalize(),
            Tuple::vector(
              1.0 / (14.0 as f64).sqrt(),
              2.0 / (14.0 as f64).sqrt(),
              3.0 / (14.0 as f64).sqrt(),
           ),
       );
    }

    #[test]
    fn dot() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(v1.dot(&v2), 20.0);
    }

    #[test]
    fn cross() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(v1.cross(&v2), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(&v1), Tuple::vector(1.0, -2.0, 1.0));
    }
}
