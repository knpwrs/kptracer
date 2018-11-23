use super::util;
use std::ops;

#[derive(Debug)]
pub struct Color (f64, f64, f64);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(r, g, b)
    }
}

impl PartialEq<Color> for Color {
  fn eq(&self, other: &Color) -> bool {
      util::approx_eq(self.0, other.0)
      && util::approx_eq(self.1, other.1)
      && util::approx_eq(self.2, other.2)
  }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, _rhs: Color) -> Color {
        Color(self.0 + _rhs.0, self.1 + _rhs.1, self.2 + _rhs.2)
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, _rhs: Color) -> Color {
        Color(self.0 - _rhs.0, self.1 - _rhs.1, self.2 - _rhs.2)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color {
        Color(self.0 * _rhs.0, self.1 * _rhs.1, self.2 * _rhs.2)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, _rhs: f64) -> Color {
        Color(self.0 * _rhs, self.1 * _rhs, self.2 * _rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        assert_eq!(Color(0.9, 0.6, 0.75), Color(0.9, 0.6, 0.75));
        assert_ne!(Color(0.9, 0.6, 0.75), Color(0.8, 0.6, 0.75));
    }

    #[test]
    fn add_two_colors() {
        let c1 = Color(0.9, 0.6, 0.75);
        let c2 = Color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtract_two_colors() {
        let c1 = Color(0.9, 0.6, 0.75);
        let c2 = Color(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Color(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiply_two_colors() {
        let c1 = Color(1.0, 0.2, 0.4);
        let c2 = Color(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, Color(0.9, 0.2, 0.04));
    }

    #[test]
    fn multiply_by_scalar() {
        let a1 = Color(0.2, 0.3, 0.4);
        assert_eq!(a1 * 2.0, Color(0.4, 0.6, 0.8));
    }
}
