pub fn approx_eq(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.00001
}

pub fn clamp(input: i32, min: i32, max: i32) -> i32 {
  if input > max {
      max
  } else if input < min {
      min
  } else {
      input
  }
}

pub fn scale(input: f64, max: i32) -> i32 {
    clamp((input * (max as f64)) as i32, 0, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approx_equal() {
        assert_eq!(approx_eq(0.9 - 0.7, 0.2), true);
        assert_eq!(approx_eq(0.9, 0.7), false);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(256, 0, 255), 255);
        assert_eq!(clamp(-1, 0, 255), 0);
        assert_eq!(clamp(55, 0, 255), 55);
    }

    #[test]
    fn test_scale() {
        assert_eq!(scale(0.25, 255), 63);
        assert_eq!(scale(-1.25, 255), 0);
        assert_eq!(scale(1.25, 255), 255);
    }
}