use hashbrown::HashMap;
use std::cmp;
use std::ops::{Add, Mul};

#[derive(Clone, Debug)]
pub struct Polynomial {
    pub data: Vec<(u16, u8)>,
    pub p: u8,
}

impl Add for &Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: &Polynomial) -> Polynomial {
        let mut c = Vec::new();
        let mut l = self.data.iter();
        let mut le = l.next();
        let mut r = rhs.data.iter();
        let mut re = r.next();

        while let (Some(lee), Some(ree)) = (le, re) {
            if lee.0 > ree.0 {
                c.push((ree.0, ree.1));
                re = r.next();
            } else if lee.0 < ree.0 {
                c.push((lee.0, lee.1));
                le = l.next();
            } else {
                let coeff = ((lee.1 + ree.1) % self.p);
                if coeff != 0 {
                    c.push((lee.0, coeff));
                }
                le = l.next();
                re = r.next();
            }
        }
        if let Some(x) = re {
            c.push(*x);
            c.extend(r);
        }
        if let Some(x) = le {
            c.push(*x);
            c.extend(l);
        }
        Polynomial { data: c, p: self.p }
    }
}
impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Polynomial) -> Polynomial {
        &self + &rhs
    }
}

//impl Mul for &Polynomial {
//    type Output = Polynomial;
//
//    fn mul(self, rhs: &Polynomial) -> Polynomial {
//        let mut c = HashMap::new();
//        for (a_pow, a_coef) in self.data.iter() {
//            for (b_pow, b_coef) in rhs.data.iter() {
//                *c.entry(a_pow + b_pow).or_insert(0) += a_coef * b_coef;
//            }
//        }
//        for c_coef in c.values_mut() {
//            *c_coef %= self.p;
//        }
//        c.retain(|_, v| *v != 0);
//        Polynomial { data: c, p: self.p }
//    }
//}

impl PartialEq for Polynomial {
    fn eq(&self, rhs: &Polynomial) -> bool {
        self.data == rhs.data
    }
}

impl Polynomial {
    pub fn new(elements: Vec<(u16, u8)>, p: u8) -> Polynomial {
        Polynomial { data: elements, p }
    }

    pub fn zero(p: u8) -> Polynomial {
        Self::new(vec![], p)
    }

    pub fn one(p: u8) -> Polynomial {
        Self::new(vec![(0, 1)], p)
    }

    pub fn is_zero(&self) -> bool {
        self.data.is_empty()
    }

    pub fn max_power(&self) -> u16 {
        self.data.last().unwrap().0
    }

    pub fn min_power(&self) -> u16 {
        self.data.first().unwrap().0
    }

    pub fn mult(&self, power: u16, neg: bool) -> Polynomial {
        let new_data: Vec<(u16, u8)> = if power == 0 && !neg {
            self.data.clone()
        } else {
            self.data
                .iter()
                .map(|(k, v)| (k + power, (if neg { self.p - v } else { *v }) % self.p))
                .collect()
        };
        Polynomial {
            data: new_data,
            p: self.p,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Matrix {
    pub d: [Polynomial; 9],
    pub p: u8,
}

impl PartialEq for Matrix {
    fn eq(&self, rhs: &Matrix) -> bool {
        for i in 0..9 {
            if self.d[i] != rhs.d[i] {
                return false;
            }
        }
        true
    }
}

impl Matrix {
    pub fn zero(p: u8) -> Matrix {
        let d: [Polynomial; 9] = [
            Polynomial::zero(p),
            Polynomial::zero(p),
            Polynomial::zero(p),
            Polynomial::zero(p),
            Polynomial::zero(p),
            Polynomial::zero(p),
            Polynomial::zero(p),
            Polynomial::zero(p),
            Polynomial::zero(p),
        ];
        Matrix { d, p }
    }

    pub fn identity(p: u8) -> Matrix {
        let mut res = Self::zero(p);
        res.d[0] = Polynomial::one(p);
        res.d[4] = Polynomial::one(p);
        res.d[8] = Polynomial::one(p);
        res
    }

    pub fn projlen(&self) -> u32 {
        let mut min_power: u16 = u16::MAX;
        let mut max_power: u16 = u16::MIN;
        for i in 0..9 {
            if !&self.d[i].is_zero() {
                min_power = cmp::min(min_power, (&self.d[i]).min_power());
                max_power = cmp::max(max_power, (&self.d[i]).max_power());
            }
        }
        return (max_power - min_power + 1).into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_polynomial_overlapping_coefficient() {
        let a = Polynomial::new(vec![(1, 1), (2, 2)], 41);
        let b = Polynomial::new(vec![(0, 41 - 1), (1, 1)], 41);
        let c = (&a + &b).data;
        assert_eq!(c.len(), 3);
        assert_eq!(c[0].1, 41 - 1);
        assert_eq!(c[1].1, 2);
        assert_eq!(c[2].1, 2);
        assert_eq!(c[2].0, 2);
    }

    #[test]
    fn add_polynomial_terms_cancel() {
        let a = Polynomial::new(vec![(1, 1), (2, 2)], 41);
        let b = Polynomial::new(vec![(0, 41 - 1), (1, 41 - 1)], 41);
        let c = (&a + &b).data;
        assert_eq!(c.len(), 2);
        assert_eq!(c[0].1, 41 - 1);
        assert_eq!(c[1].0, 2);
        assert_eq!(c[1].1, 2);
    }

    //    #[test]
    //    fn multiply_polynomial_negative_coefficient() {
    //        let a = Polynomial::new(vec![(1, 1), (2, 2)], 41);
    //        let b = Polynomial::new(vec![(0, 41 - 1)], 41);
    //        let c = (&a * &b).data;
    //        // (t + 2t^2) * (-1) = -t - 2t^2
    //        assert_eq!(c.values().count(), 2);
    //        assert_eq!(c[&1], 41 - 1);
    //        assert_eq!(c[&2], 41 - 2);
    //    }
    #[test]
    fn add_polynomial_mod_two() {
        let a = Polynomial::new(vec![(1, 1)], 2);
        let c = (&a + &a).data;
        assert_eq!(c.len(), 0);
    }

    //    #[test]
    //    fn multiply_polynomial_mod_three() {
    //        let a = Polynomial::new(vec![(1, 2)], 3);
    //        let c = (&a * &a).data;
    //        // 2x * 2x = x² over 𝔽₃.
    //        assert_eq!(c.values().count(), 1);
    //        assert_eq!(c[&2], 1);
    //    }
    //
    #[test]
    fn zero_polynomial_is_zero() {
        let a = Polynomial::zero(41);
        assert!(a.is_zero());
    }

    #[test]
    fn non_zero_polynomial_is_non_zero() {
        let a = Polynomial::new(vec![(0, 2), (1, 1), (5, 2)], 3);
        assert!(!a.is_zero());
    }

    #[test]
    fn identity_matrix_projlen() {
        let mat = Matrix::identity(41);
        assert_eq!(mat.projlen(), 1);
    }
}
