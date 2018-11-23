pub fn approx_eq(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.00001
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approx_equal() {
        assert_eq!(approx_eq(0.9 - 0.7, 0.2), true);
    }
}