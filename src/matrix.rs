use super::{ tuple, util };
use std::ops;

#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    values: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            values: vec![0.0; rows * cols],
        }
    }

    pub fn new_with_values(rows: usize, values: Vec<f64>) -> Matrix {
        if values.len() % rows != 0 {
            panic!("{} is not divisible by {}", values.len(), rows);
        }
        Matrix {
            rows,
            cols: values.len() / rows,
            values,
        }
    }

    pub fn identity(rows: usize) -> Matrix {
        let mut values = Vec::new();
        for i in 0..rows {
            for _ in 0..i {
                values.push(0.0);
            }
            values.push(1.0);
            for _ in (i + 1)..rows {
                values.push(0.0);
            }
        }
        Matrix::new_with_values(rows, values)
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
        let mut m = Matrix::identity(4);
        m.write_value(0, 3, x);
        m.write_value(1, 3, y);
        m.write_value(2, 3, z);
        m
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
        let mut m = Matrix::identity(4);
        m.write_value(0, 0, x);
        m.write_value(1, 1, y);
        m.write_value(2, 2, z);
        m
    }

    pub fn rotation_x(rad: f64) -> Matrix {
        let mut m = Matrix::identity(4);
        m.write_value(1, 1, rad.cos());
        m.write_value(1, 2, -(rad.sin()));
        m.write_value(2, 1, rad.sin());
        m.write_value(2, 2, rad.cos());
        m
    }

    pub fn rotation_y(rad: f64) -> Matrix {
        let mut m = Matrix::identity(4);
        m.write_value(0, 0, rad.cos());
        m.write_value(0, 2, rad.sin());
        m.write_value(2, 0, -(rad.sin()));
        m.write_value(2, 2, rad.cos());
        m
    }

    pub fn rotation_z(rad: f64) -> Matrix {
        let mut m = Matrix::identity(4);
        m.write_value(0, 0, rad.cos());
        m.write_value(0, 1, -(rad.sin()));
        m.write_value(1, 0, rad.sin());
        m.write_value(1, 1, rad.cos());
        m
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
        let mut m = Matrix::identity(4);
        m.write_value(0, 1, xy);
        m.write_value(0, 2, xz);
        m.write_value(1, 0, yx);
        m.write_value(1, 2, yz);
        m.write_value(2, 0, zx);
        m.write_value(2, 1, zy);
        m
    }

    pub fn write_value(&mut self, row: usize, col: usize, v: f64) {
        self.values[row * self.cols + col] = v;
    }

    pub fn value_at(&self, row: usize, col: usize) -> f64 {
        self.values[row * self.cols + col]
    }

    pub fn transpose(&self) -> Matrix {
        let mut values = Vec::new();
        for col in 0..self.cols {
            for row in 0..self.rows {
                values.push(self.value_at(row, col));
            }
        }
        Matrix::new_with_values(self.rows, values)
    }

    pub fn determinant(&self) -> f64 {
        if self.rows == 2 && self.cols == 2 {
            (self.value_at(0, 0) * self.value_at(1, 1)) - (self.value_at(0, 1) * self.value_at(1, 0))
        } else {
            let mut sum = 0.0;
            for col in 0..self.cols {
                sum += self.value_at(0, col) * self.cofactor(0, col);
            }
            sum
        }
    }

    pub fn submatrix(&self, rrow: usize, rcol: usize) -> Matrix {
        let mut values = Vec::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                if row != rrow && col != rcol {
                    values.push(self.value_at(row, col));
                }
            }
        }
        Matrix::new_with_values(self.rows - 1, values)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let factor = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };
        factor * self.minor(row, col)
    }

    pub fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Matrix {
        let mut values = Vec::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                values.push(self.cofactor(row, col));
            }
        }
        let tc = Matrix::new_with_values(self.rows, values).transpose();
        let det = self.determinant();
        let mut i_values = Vec::new();
        for row in 0..tc.rows {
            for col in 0..tc.cols {
                i_values.push(tc.value_at(row, col) / det);
            }
        }
        Matrix::new_with_values(tc.rows, i_values)
    }
}

impl PartialEq<Matrix> for Matrix {
  fn eq(&self, other: &Matrix) -> bool {
      if self.rows != other.rows {
          false
      } else if self.cols != other.cols {
          false
      } else {
          for i in 0..self.values.len() {
              if !util::approx_eq(self.values[i], other.values[i]) {
                  return false
              }
          }
          true
      }
  }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("These matrices cannot be multiplied!");
        }
        let mut values = Vec::new();
        for s_row in 0..self.rows {
            for o_col in 0..other.cols {
                let mut sum = 0.0;
                for i in 0..self.cols {
                    sum += self.value_at(s_row, i) * other.value_at(i, o_col);
                }
                values.push(sum);
            }
        }
        Matrix::new_with_values(self.rows, values)
    }
}

impl ops::Mul<tuple::Tuple> for Matrix {
    type Output = tuple::Tuple;

    // this whole method is gross
    fn mul(self, other: tuple::Tuple) -> tuple::Tuple {
        if self.cols != 4 {
            panic!("This matrix cannot be multiplied by a 4-tuple!");
        }
        let mut values = Vec::new();
        for s_row in 0..self.rows {
            values.push(
                (self.value_at(s_row, 0) * other.get(0))
                + (self.value_at(s_row, 1) * other.get(1))
                + (self.value_at(s_row, 2) * other.get(2))
                + (self.value_at(s_row, 3) * other.get(3))
            );
        }
        tuple::Tuple::new(values[0], values[1], values[2], values[3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn root_2() -> f64 {
        (2.0 as f64).sqrt()
    }

    #[test]
    fn create_matrix() {
        let m = Matrix::new(10, 20);
        assert_eq!(m.rows, 10);
        assert_eq!(m.cols, 20);
        for v in m.values {
            assert_eq!(v, 0.0);
        }
    }

    #[test]
    fn create_2_2_matrix_with_values() {
        let m = Matrix::new_with_values(2, vec![-3.0, 5.0, 1.0, -2.0]);
        assert_eq!(m.value_at(0, 0), -3.0);
        assert_eq!(m.value_at(0, 1), 5.0);
        assert_eq!(m.value_at(1, 0), 1.0);
        assert_eq!(m.value_at(1, 1), -2.0);
    }

    #[test]
    fn create_3_3_matrix_with_values() {
        let m = Matrix::new_with_values(3, vec![-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);
        assert_eq!(m.value_at(0, 0), -3.0);
        assert_eq!(m.value_at(0, 1), 5.0);
        assert_eq!(m.value_at(0, 2), 0.0); 
        assert_eq!(m.value_at(1, 0), 1.0);
        assert_eq!(m.value_at(1, 1), -2.0);
        assert_eq!(m.value_at(1, 2), -7.0);
        assert_eq!(m.value_at(2, 0), 0.0);
        assert_eq!(m.value_at(2, 1), 1.0);
        assert_eq!(m.value_at(2, 2), 1.0);
    }

    #[test]
    fn create_4_4_matrix_with_values() {
        let m = Matrix::new_with_values(4, vec![1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5]);
        assert_eq!(m.value_at(0, 0), 1.0);
        assert_eq!(m.value_at(0, 3), 4.0);
        assert_eq!(m.value_at(1, 0), 5.5);
        assert_eq!(m.value_at(1, 2), 7.5);
        assert_eq!(m.value_at(2, 2), 11.0);
        assert_eq!(m.value_at(3, 0), 13.5);
        assert_eq!(m.value_at(3, 2), 15.5);
    }

    #[test]
    #[should_panic]
    fn panics_with_invalid_values() {
       let _m = Matrix::new_with_values(4, vec![1.0, 2.0, 3.0, 4.0, 5.0]); 
    }

    #[test]
    fn equality() {
        let m1 = Matrix::new_with_values(4, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]);
        let m2 = Matrix::new_with_values(4, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]);
        let m3 = Matrix::new_with_values(4, vec![1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5, 8.5, 7.5, 6.5, 5.5, 4.5, 3.5, 2.5]);
        assert_eq!(m1, m2);
        assert_eq!(m2, m1);
        assert_ne!(m1, m3);
        assert_ne!(m2, m3);
    }

     #[test]
    fn multiply() {
        let m1 = Matrix::new_with_values(4, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]);
        let m2 = Matrix::new_with_values(4, vec![-2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0]);
        let m3 = Matrix::new_with_values(4, vec![20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0, 46.0, 42.0]);
        assert_eq!(m1 * m2, m3);
    }

    #[test]
    fn multiply_tuple() {
        let m1 = Matrix::new_with_values(4, vec![1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        let t1 = tuple::Tuple::new(1.0, 2.0, 3.0, 1.0);
        let t2 = tuple::Tuple::new(18.0, 24.0, 33.0, 1.0);
        assert_eq!(m1 * t1, t2);
    }

    #[test]
    fn identity() {
        let id1 = Matrix::identity(1);
        assert_eq!(id1, Matrix::new_with_values(1, vec![1.0]));
        let id2 = Matrix::identity(2);
        assert_eq!(id2, Matrix::new_with_values(2, vec![1.0, 0.0, 0.0, 1.0]));
        let id3 = Matrix::identity(3);
        assert_eq!(id3, Matrix::new_with_values(3, vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]));
        let id4 = Matrix::identity(4);
        assert_eq!(id4, Matrix::new_with_values(4, vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]));
        let m1 = Matrix::new_with_values(4, vec![1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        let m2 = m1.clone();
        assert_eq!(m1 * id4, m2);
    }

    #[test]
    fn transpose() {
        let id4 = Matrix::identity(4);
        assert_eq!(id4.transpose(), id4);
        let m1 = Matrix::new_with_values(4, vec![0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0]);
        let m2 = Matrix::new_with_values(4, vec![0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0]);
        assert_eq!(m1.transpose(), m2);
    }

    #[test]
    fn determinant() {
        let m2 = Matrix::new_with_values(2, vec![1.0, 5.0, -3.0, 2.0]);
        assert_eq!(m2.determinant(), 17.0);
        let m3 = Matrix::new_with_values(3, vec![1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);
        assert_eq!(m3.cofactor(0, 0), 56.0);
        assert_eq!(m3.cofactor(0, 1), 12.0);
        assert_eq!(m3.cofactor(0, 2), -46.0);
        assert_eq!(m3.determinant(), -196.0);
        let m4 = Matrix::new_with_values(4, vec![-2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0]);
        assert_eq!(m4.cofactor(0, 0), 690.0);
        assert_eq!(m4.cofactor(0, 1), 447.0);
        assert_eq!(m4.cofactor(0, 2), 210.0);
        assert_eq!(m4.cofactor(0, 3), 51.0);
        assert_eq!(m4.determinant(), -4071.0); 
    }

    #[test]
    fn submatrix() {
        let m1 = Matrix::new_with_values(3, vec![1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
        let sm1 = Matrix::new_with_values(2, vec![-3.0, 2.0, 0.0, 6.0]);
        assert_eq!(m1.submatrix(0, 2), sm1);
        let m2 = Matrix::new_with_values(4, vec![-6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0]);
        let sm2 = Matrix::new_with_values(3, vec![-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0]);
        assert_eq!(m2.submatrix(2, 1), sm2);
    }

    #[test]
    fn minor() {
        let m1 = Matrix::new_with_values(3, vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        assert_eq!(m1.submatrix(1, 0).determinant(), 25.0);
        assert_eq!(m1.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor() {
        let m1 = Matrix::new_with_values(3, vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        assert_eq!(m1.minor(0, 0), -12.0);
        assert_eq!(m1.cofactor(0, 0), -12.0);
        assert_eq!(m1.minor(1, 0), 25.0);
        assert_eq!(m1.cofactor(1, 0), -25.0);
    }

    #[test]
    fn invertible() {
        let m1 = Matrix::new_with_values(4, vec![6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0]);
        assert_eq!(m1.invertible(), true);
        let m2 = Matrix::new_with_values(4, vec![-4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(m2.invertible(), false);
    }

    #[test]
    fn inverse() {
        let m1 = Matrix::new_with_values(4, vec![-5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0]);
        let mi = m1.inverse();
        assert_eq!(m1.determinant(), 532.0);
        assert_eq!(m1.cofactor(2, 3), -160.0);
        assert_eq!(util::approx_eq(mi.value_at(3, 2), -160.0 / 532.0), true);
        assert_eq!(m1.cofactor(3, 2), 105.0);
        assert_eq!(util::approx_eq(mi.value_at(2, 3), 105.0 / 532.0), true);
        assert_eq!(mi, Matrix::new_with_values(4, vec![0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068, -0.07895, -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639]));
        let mi = Matrix::new_with_values(4, vec![8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0]).inverse();
        assert_eq!(mi, Matrix::new_with_values(4, vec![-0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077, 0.35897, 0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308]));
        let mi = Matrix::new_with_values(4, vec![9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0, 2.0]).inverse();
        assert_eq!(mi, Matrix::new_with_values(4, vec![-0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333, -0.02901, -0.14630, -0.10926, 0.12963, 0.17778, 0.06667, -0.26667, 0.33333]));
        let m2 = Matrix::new_with_values(4, vec![3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0, 1.0]);
        let m2c = m2.clone();
        let m3 = Matrix::new_with_values(4, vec![8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0]);
        let m3i = m3.inverse();
        assert_eq!(m2 * m3 * m3i, m2c);
    }

    #[test]
    fn translation_points() {
        let p = tuple::Tuple::point(-3.0, 4.0, 5.0);
        let pc = p.clone();
        let tm = Matrix::translation(5.0, -3.0, 2.0);
        let tmi = tm.inverse();
        let tp = tm * p;
        assert_eq!(tp, tuple::Tuple::point(2.0, 1.0, 7.0));
        let op = tmi * tp;
        assert_eq!(op, pc);
    }

    #[test]
    fn translation_vectors() {
        let v = tuple::Tuple::vector(-3.0, 4.0, 5.0);
        let vc = v.clone();
        let tm = Matrix::translation(5.0, -3.0, 2.0);
        let tv = tm * v;
        assert_eq!(tv, vc);
    }

    #[test]
    fn scaling_points() {
        let p = tuple::Tuple::point(-4.0, 6.0, 8.0);
        let pc = p.clone();
        let tm = Matrix::scaling(2.0, 3.0, 4.0);
        let tmi = tm.inverse();
        let tp = tm * p;
        assert_eq!(tp, tuple::Tuple::point(-8.0, 18.0, 32.0));
        let op = tmi * tp;
        assert_eq!(op, pc); 
    }

    #[test]
    fn scaling_vectors() {
        let v = tuple::Tuple::vector(-4.0, 6.0, 8.0);
        let vc = v.clone();
        let tm = Matrix::scaling(2.0, 3.0, 4.0);
        let tmi = tm.inverse();
        let tv = tm * v;
        assert_eq!(tv, tuple::Tuple::vector(-8.0, 18.0, 32.0));
        let ov = tmi * tv;
        assert_eq!(ov, vc); 
    }

    #[test]
    fn reflecting_points() {
        let p = tuple::Tuple::point(2.0, 3.0, 4.0);
        let pc = p.clone();
        let tm = Matrix::scaling(-1.0, 1.0, 1.0);
        let tmi = tm.inverse();
        let tp = tm * p;
        assert_eq!(tp, tuple::Tuple::point(-2.0, 3.0, 4.0));
        let op = tmi * tp;
        assert_eq!(op, pc); 
    }

    #[test]
    fn reflecting_vectors() {
        let v = tuple::Tuple::vector(2.0, 3.0, 4.0);
        let vc = v.clone();
        let tm = Matrix::scaling(-1.0, 1.0, 1.0);
        let tmi = tm.inverse();
        let tv = tm * v;
        assert_eq!(tv, tuple::Tuple::vector(-2.0, 3.0, 4.0));
        let ov = tmi * tv;
        assert_eq!(ov, vc); 
    }

    #[test]
    fn rotation_x() {
        let p1 = tuple::Tuple::point(0.0, 1.0, 0.0);
        let p2 = p1.clone();
        let p3 = p1.clone();
        let p4 = p1.clone();
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let half_quarter_i = half_quarter.inverse();
        let full_quarter = Matrix::rotation_x(PI / 2.0);
        let full_quarter_i = full_quarter.inverse();
        let p1hq = half_quarter * p1;
        assert_eq!(p1hq, tuple::Tuple::point(0.0, root_2() / 2.0, root_2() / 2.0));
        assert_eq!(half_quarter_i * p1hq, p3);
        let p2hq = full_quarter * p2;
        assert_eq!(p2hq, tuple::Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(full_quarter_i * p2hq, p4);
    }

    #[test]
    fn rotation_y() {
        let p1 = tuple::Tuple::point(0.0, 0.0, 1.0);
        let p2 = p1.clone();
        let p3 = p1.clone();
        let p4 = p1.clone();
        let half_quarter = Matrix::rotation_y(PI / 4.0);
        let half_quarter_i = half_quarter.inverse();
        let full_quarter = Matrix::rotation_y(PI / 2.0);
        let full_quarter_i = full_quarter.inverse();
        let p1hq = half_quarter * p1;
        assert_eq!(p1hq, tuple::Tuple::point(root_2() / 2.0, 0.0, root_2() / 2.0));
        assert_eq!(half_quarter_i * p1hq, p3);
        let p2hq = full_quarter * p2;
        assert_eq!(p2hq, tuple::Tuple::point(1.0, 0.0, 0.0));
        assert_eq!(full_quarter_i * p2hq, p4);
    }

    #[test]
    fn rotation_z() {
        let p1 = tuple::Tuple::point(0.0, 1.0, 0.0);
        let p2 = p1.clone();
        let p3 = p1.clone();
        let p4 = p1.clone();
        let half_quarter = Matrix::rotation_z(PI / 4.0);
        let half_quarter_i = half_quarter.inverse();
        let full_quarter = Matrix::rotation_z(PI / 2.0);
        let full_quarter_i = full_quarter.inverse();
        let p1hq = half_quarter * p1;
        assert_eq!(p1hq, tuple::Tuple::point(-(root_2() / 2.0), root_2() / 2.0, 0.0));
        assert_eq!(half_quarter_i * p1hq, p3);
        let p2hq = full_quarter * p2;
        assert_eq!(p2hq, tuple::Tuple::point(-1.0, 0.0, 0.0));
        assert_eq!(full_quarter_i * p2hq, p4);
    }

    #[test]
    fn shearing_xy() {
        let t = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = tuple::Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(t * p, tuple::Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_xz() {
        let t = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = tuple::Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(t * p, tuple::Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_yx() {
        let t = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = tuple::Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(t * p, tuple::Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_yz() {
        let t = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = tuple::Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(t * p, tuple::Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_zx() {
        let t = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = tuple::Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(t * p, tuple::Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_zy() {
        let t = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = tuple::Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(t * p, tuple::Tuple::point(2.0, 3.0, 7.0));
    }
}