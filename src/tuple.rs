use std::ops;
use super::util;

#[derive(Debug, Clone, Copy)]
pub struct Tuple (f64, f64, f64, f64);

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple(x, y, z, w)
    }

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
        let &Tuple(_, _, _, w) = self;
        util::approx_eq(w, 0.0)
    }

    pub fn get(&self, i: usize) -> f64 {
      // gross
      match i {
          0 => self.0,
          1 => self.1,
          2 => self.2,
          3 => self.3,
          _ => panic!("Out of Tuple bounds!"),
      }
    }

    pub fn magnitude(&self) -> f64 {
        let Tuple (x, y, z, w) = self;
        (x.powi(2) + y.powi(2) + z.powi(2) + w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();
        let Tuple (x, y, z, w) = self;
        Tuple(x / mag, y / mag, z / mag, w / mag)
    }

    pub fn dot(&self, Tuple (rx, ry, rz, rw): &Tuple) -> f64 {
        let Tuple (x, y, z, w) = self;
        (x * rx) + (y * ry) + (z * rz) + (w * rw)
    }

    // Assumes _rhs is a vector
    pub fn cross(&self, Tuple (rx, ry, rz, _rw): &Tuple) -> Tuple {
        let Tuple (x, y, z, _w) = self;
        Tuple::vector(
          (y * rz) - (z * ry),
          (z * rx) - (x * rz),
          (x * ry) - (y * rx),
        )
    }
}

impl PartialEq<Tuple> for Tuple {
  fn eq(&self, &Tuple (rx, ry, rz, rw): &Tuple) -> bool {
      let &Tuple (x, y, z, w) = self;
      util::approx_eq(x, rx)
      && util::approx_eq(y, ry)
      && util::approx_eq(z, rz)
      && util::approx_eq(w, rw)
  }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, Tuple (rx, ry, rz, rw): Tuple) -> Tuple {
        let Tuple (x, y, z, w) = self;
        Tuple(x + rx, y + ry, z + rz, w + rw)
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, Tuple (rx, ry, rz, rw): Tuple) -> Tuple {
        let Tuple (x, y, z, w) = self;
        Tuple(x - rx, y - ry, z - rz, w - rw)
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, r: f64) -> Tuple {
        let Tuple (x, y, z, w) = self;
        Tuple(x * r, y * r, z * r, w * r)
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, r: f64) -> Tuple {
        let Tuple (x, y, z, w) = self;
        Tuple(x / r, y / r, z / r, w / r)
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
