use super::util;
use std::clone::Clone;
use std::ops;

#[derive(Debug)]
pub struct Color (f64, f64, f64);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(r, g, b)
    }

    pub fn to_ppm_string(&self) -> String {
        let &Color (r, g, b) = self;
        format!("{} {} {}", util::scale(r, 255), util::scale(g, 255), util::scale(b, 255))
    }
}

impl PartialEq<Color> for Color {
  fn eq(&self, &Color (rr, rg, rb): &Color) -> bool {
      let &Color (r, g, b) = self;
      util::approx_eq(r, rr)
      && util::approx_eq(g, rg)
      && util::approx_eq(b, rb)
  }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, Color (rr, rg, rb): Color) -> Color {
        let Color (r, g, b) = self;
        Color(r + rr, g + rg, b + rb)
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, Color (rr, rg, rb): Color) -> Color {
        let Color (r, g, b) = self;
        Color(r - rr, g - rg, b - rb)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, Color (rr, rg, rb): Color) -> Color {
        let Color (r, g, b) = self;
        Color(r * rr, g * rg, b * rb)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rf: f64) -> Color {
        let Color (r, g, b) = self;
        Color(r * rf, g * rf, b * rf)
    }
}

impl Clone for Color {
    fn clone(&self) -> Color {
        let &Color (r, g, b) = self;
        Color(r, g, b)
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
        let c1 = Color(0.2, 0.3, 0.4);
        assert_eq!(c1 * 2.0, Color(0.4, 0.6, 0.8));
    }

    #[test]
    fn to_ppm_string() {
        let c1 = Color(1.0, 0.2, 0.4);
        println!("{} {} {}", c1.0, c1.1, c1.2);
        assert_eq!(c1.to_ppm_string(), "255 51 102");
        let c2 = Color(0.9, 1.0, 0.1);
        assert_eq!(c2.to_ppm_string(), "229 255 25");
    }
}
