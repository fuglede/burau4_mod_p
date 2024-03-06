use hashbrown::HashMap;
//use std::collections::HashMap;
use std::cmp;
use std::ops::{Add, Mul};

#[derive(Clone, Debug)]
pub struct Polynomial {
    pub data: HashMap<i32, i32>,
    pub p: i32,
}

impl Add for &Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: &Polynomial) -> Polynomial {
        let mut c = HashMap::new();
        for (a_pow, a_coef) in self.data.iter() {
            *c.entry(*a_pow).or_insert(0) += a_coef;
        }
        for (b_pow, b_coef) in rhs.data.iter() {
            *c.entry(*b_pow).or_insert(0) += b_coef;
        }
        for c_coef in c.values_mut() {
            *c_coef %= self.p;
        }
        c.retain(|_, v| *v != 0);
        Polynomial { data: c, p: self.p }
    }
}

impl Mul for &Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: &Polynomial) -> Polynomial {
        let mut c = HashMap::new();
        for (a_pow, a_coef) in self.data.iter() {
            for (b_pow, b_coef) in rhs.data.iter() {
                *c.entry(a_pow + b_pow).or_insert(0) += a_coef * b_coef;
            }
        }
        for c_coef in c.values_mut() {
            *c_coef %= self.p;
        }
        c.retain(|_, v| *v != 0);
        Polynomial { data: c, p: self.p }
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, rhs: &Polynomial) -> bool {
        self.data == rhs.data
    }
}

impl Polynomial {
    pub fn new(elements: Vec<(i32, i32)>, p: i32) -> Polynomial {
        let mut data = HashMap::new();
        for (a, b) in elements {
            data.insert(a, b);
        }
        Polynomial { data, p }
    }

    pub fn zero(p: i32) -> Polynomial {
        Self::new(vec![], p)
    }

    pub fn one(p: i32) -> Polynomial {
        Self::new(vec![(0, 1)], p)
    }

    pub fn is_zero(&self) -> bool {
        self.data.is_empty()
    }

    pub fn max_power(&self) -> i32 {
        *self.data.keys().max().unwrap()
    }

    pub fn min_power(&self) -> i32 {
        *self.data.keys().min().unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct Matrix {
    pub d: [[Polynomial; 3]; 3],
    pub p: i32,
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Matrix {
        let mut res = Matrix::zero(self.p);
        for i in 0..3 {
            for j in 0..3 {
                res.d[i][j] = (0..3)
                    .map(|k| &self.d[i][k] * &rhs.d[k][j])
                    .fold(Polynomial::zero(self.p), |sum, val| &sum + &val);
            }
        }
        res
    }
}

impl PartialEq for Matrix {
    fn eq(&self, rhs: &Matrix) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if self.d[i][j] != rhs.d[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

impl Matrix {
    pub fn zero(p: i32) -> Matrix {
        let d: [[Polynomial; 3]; 3] = [
            [
                Polynomial::zero(p),
                Polynomial::zero(p),
                Polynomial::zero(p),
            ],
            [
                Polynomial::zero(p),
                Polynomial::zero(p),
                Polynomial::zero(p),
            ],
            [
                Polynomial::zero(p),
                Polynomial::zero(p),
                Polynomial::zero(p),
            ],
        ];
        Matrix { d, p }
    }

    pub fn identity(p: i32) -> Matrix {
        let mut res = Self::zero(p);
        for i in 0..3 {
            res.d[i][i] = Polynomial::one(p);
        }
        res
    }

    pub fn projlen(&self) -> i32 {
        let mut min_power: i32 = i32::MAX;
        let mut max_power: i32 = i32::MIN;
        for i in 0..3 {
            for j in 0..3 {
                if !&self.d[i][j].is_zero() {
                    min_power = cmp::min(min_power, (&self.d[i][j]).min_power());
                    max_power = cmp::max(max_power, (&self.d[i][j]).max_power());
                }
            }
        }
        return max_power - min_power + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_polynomial_overlapping_coefficient() {
        let a = Polynomial::new(vec![(1, 1), (2, 2)], 41);
        let b = Polynomial::new(vec![(0, -1), (1, 1)], 41);
        let c = (&a + &b).data;
        assert_eq!(c.values().count(), 3);
        assert_eq!(c[&0], -1);
        assert_eq!(c[&1], 2);
        assert_eq!(c[&2], 2);
    }

    #[test]
    fn add_polynomial_terms_cancel() {
        let a = Polynomial::new(vec![(1, 1), (2, 2)], 41);
        let b = Polynomial::new(vec![(0, -1), (1, -1)], 41);
        let c = (&a + &b).data;
        assert_eq!(c.values().count(), 2);
        assert_eq!(c[&0], -1);
        assert_eq!(c[&2], 2);
    }

    #[test]
    fn multiply_polynomial_negative_coefficient() {
        let a = Polynomial::new(vec![(1, 1), (2, 2)], 41);
        let b = Polynomial::new(vec![(0, -1)], 41);
        let c = (&a * &b).data;
        // (t + 2t^2) * (-1) = -t - 2t^2
        assert_eq!(c.values().count(), 2);
        assert_eq!(c[&1], -1);
        assert_eq!(c[&2], -2);
    }

    #[test]
    fn multiply_polynomial_terms_cancel() {
        let a = Polynomial::new(vec![(-2, 2), (2, 2)], 41);
        let b = Polynomial::new(vec![(-2, 2), (2, 2)], 41);
        let c = (&a * &b).data;
        // (2t^{-2} + 2t^2) * (2t^{-2} + 2t^2) = 4t^{-4} + 8 + 4t^4
        assert_eq!(c.values().count(), 3);
        assert_eq!(c[&-4], 4);
        assert_eq!(c[&0], 8);
        assert_eq!(c[&-4], 4);
    }

    #[test]
    fn add_polynomial_mod_two() {
        let a = Polynomial::new(vec![(1, 1)], 2);
        let c = (&a + &a).data;
        assert_eq!(c.values().count(), 0);
    }

    #[test]
    fn multiply_polynomial_mod_three() {
        let a = Polynomial::new(vec![(1, 2)], 3);
        let c = (&a * &a).data;
        // 2x * 2x = x² over 𝔽₃.
        assert_eq!(c.values().count(), 1);
        assert_eq!(c[&2], 1);
    }

    #[test]
    fn zero_polynomial_is_zero() {
        let a = Polynomial::zero(41);
        assert!(a.is_zero());
    }

    #[test]
    fn non_zero_polynomial_is_non_zero() {
        let a = Polynomial::new(vec![(-10, 2), (1, 1), (5, 2)], 3);
        assert!(!a.is_zero());
    }

    #[test]
    fn multiply_matrix_two_identities() {
        let a = Matrix::identity(41);
        let b = Matrix::identity(41);
        let c = &a * &b;
        assert_eq!(c, Matrix::identity(41));
    }

    #[test]
    fn multiply_matrix_left_side_identity() {
        let a = Matrix::identity(41);
        let pol = Polynomial::new(vec![(-2, 2), (-2, 2)], 41);
        let pol2 = Polynomial::new(vec![(-2, 2), (-2, 2)], 41);
        let mut b = Matrix::zero(41);
        b.d[0][1] = Polynomial::one(41);
        b.d[0][2] = pol;
        b.d[2][1] = pol2;
        let c = &a * &b;
        assert_eq!(c, b);
    }

    #[test]
    fn multiply_matrix_left_side_zero() {
        let pol = Polynomial::new(vec![(-2, 2), (-2, 2)], 41);
        let pol2 = Polynomial::new(vec![(-2, 2), (-2, 2)], 41);
        let a = Matrix::zero(41);
        let mut b = Matrix::zero(41);
        b.d[0][1] = Polynomial::one(41);
        b.d[0][2] = pol;
        b.d[2][1] = pol2;
        let c = &a * &b;
        assert_eq!(c, a);
    }

    #[test]
    fn identity_matrix_projlen() {
        let mat = Matrix::identity(41);
        assert_eq!(mat.projlen(), 1);
    }

    #[test]
    fn multiply_matrix_non_trivial_example() {
        // Upper left 2x2 block:
        // [ t,       1 + t ] [ -t          0 ]     [ 2t^3 + t^4          5 + 5t ]
        // [ t^{-1},   2t^2 ] [ t^2 + t^3   5 ]  =  [ -1 + 2t^4 + 2t^5     10t^2 ]
        // Left:
        let mut mat1 = Matrix::identity(41);
        mat1.d[0][0] = Polynomial::new(vec![(1, 1)], 41);
        mat1.d[0][1] = Polynomial::new(vec![(0, 1), (1, 1)], 41);
        mat1.d[1][0] = Polynomial::new(vec![(-1, 1)], 41);
        mat1.d[1][1] = Polynomial::new(vec![(2, 2)], 41);

        // Right:
        let mut mat2 = Matrix::identity(41);
        mat2.d[0][0] = Polynomial::new(vec![(1, -1)], 41);
        mat2.d[1][0] = Polynomial::new(vec![(2, 1), (3, 1)], 41);
        mat2.d[1][1] = Polynomial::new(vec![(0, 5)], 41);

        // Expected product:
        let mut mat3 = Matrix::identity(41);
        mat3.d[0][0] = Polynomial::new(vec![(3, 2), (4, 1)], 41);
        mat3.d[0][1] = Polynomial::new(vec![(0, 5), (1, 5)], 41);
        mat3.d[1][0] = Polynomial::new(vec![(0, -1), (4, 2), (5, 2)], 41);
        mat3.d[1][1] = Polynomial::new(vec![(2, 10)], 41);

        let actual = &mat1 * &mat2;
        assert_eq!(actual, mat3);
        assert_eq!(actual.projlen(), 6);
    }
}
