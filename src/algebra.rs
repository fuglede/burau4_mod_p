use std::cmp;
use std::ops::Add;

#[derive(Clone, Debug)]
pub struct Polynomial {
    pub powers: Vec<u32>,
    pub coefs: Vec<u8>,
    pub p: u8,
}

fn add(a_keys: &Vec<u32>, a_values: &Vec<u8>, b_keys: &Vec<u32>, b_values: &Vec<u8>, p: u8) -> (Vec<u32>, Vec<u8>) {
    let mut i = 0;
    let mut j = 0;
    let mut c_keys = Vec::new();
    let mut c_values = Vec::new();

    while i < a_keys.len() && j < b_keys.len() {
        if a_keys[i] < b_keys[j] {
            c_keys.push(a_keys[i]);
            c_values.push(a_values[i]);
            i += 1;
        } else if a_keys[i] > b_keys[j] {
            c_keys.push(b_keys[j]);
            c_values.push(b_values[j]);
            j += 1;
        } else {
            let new_val = (a_values[i] + b_values[j]) % p;
            if new_val != 0 {
                c_keys.push(a_keys[i]);
                c_values.push(new_val);
            }
            i += 1;
            j += 1;
        }
    }

    while i < a_keys.len() {
        c_keys.push(a_keys[i]);
        c_values.push(a_values[i]);
        i += 1;
    }

    while j < b_keys.len() {
        c_keys.push(b_keys[j]);
        c_values.push(b_values[j]);
        j += 1;
    }

    (c_keys, c_values)
}

impl Add for &Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: &Polynomial) -> Polynomial {
        let (new_powers, new_coefs) = add(&self.powers, &self.coefs, &rhs.powers, &rhs.coefs, self.p);
        Polynomial { powers: new_powers, coefs: new_coefs, p: self.p }
    }
}
impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Polynomial) -> Polynomial {
        &self + &rhs
    }
}

impl Polynomial {
    pub fn new(elements: Vec<(u32, u8)>, p: u8) -> Polynomial {
        let mut powers: Vec<u32> = Vec::new();
        let mut coefs: Vec<u8> = Vec::new();
        for (a, b) in elements {
            powers.push(a);
            coefs.push(b);
        }
        Polynomial { powers, coefs, p }
    }

    pub fn zero(p: u8) -> Polynomial {
        Self::new(vec![], p)
    }

    pub fn one(p: u8) -> Polynomial {
        Self::new(vec![(0, 1)], p)
    }

    pub fn is_zero(&self) -> bool {
        self.powers.is_empty()
    }

    pub fn max_power(&self) -> u32 {
        *self.powers.last().unwrap()
    }

    pub fn min_power(&self) -> u32 {
        *self.powers.first().unwrap()
    }

    pub fn mult(&self, power: u32, neg: bool) -> Polynomial {
        let new_powers: Vec<u32> = if self.is_zero() {
            Vec::new()
        } else if power == 0 {
            self.powers.clone()
        } else {
            self.powers.iter().map(|x| x + power).collect()
        };
        let new_coefs = if neg {
            self.coefs.iter().map(|x| (self.p - x) % self.p).collect()
        } else {
            self.coefs.clone()
        };
        Polynomial {
            powers: new_powers,
            coefs: new_coefs,
            p: self.p,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Matrix {
    pub d: [Polynomial; 9],
    pub p: u8,
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
        let mut min_power: u32 = u32::MAX;
        let mut max_power: u32 = u32::MIN;
        for i in 0..9 {
            if !&self.d[i].is_zero() {
                min_power = cmp::min(min_power, (&self.d[i]).min_power());
                max_power = cmp::max(max_power, (&self.d[i]).max_power());
            }
        }
        return max_power - min_power + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_handles_overlap() {
        let a_keys: Vec<u32> = vec![2, 4, 8];
        let b_keys: Vec<u32> = vec![1, 4, 8];
        let a_vals: Vec<u8> = vec![10, 20, 30];
        let b_vals: Vec<u8> = vec![5, 3, 1];
        let (c_keys, c_vals) = add(&a_keys, &a_vals, &b_keys, &b_vals, 41);
        assert_eq!(c_keys, vec![1, 2, 4, 8]);
        assert_eq!(c_vals, vec![5, 10, 23, 31]);
    }

    #[test]
    fn add_polynomial_overlapping_coefficient() {
        let a = Polynomial::new(vec![(1, 1), (2, 2)], 41);
        let b = Polynomial::new(vec![(0, 41 - 1), (1, 1)], 41);
        let c = (&a + &b).powers;
        assert_eq!(c.len(), 3);
        assert_eq!(c[0], 0);
        assert_eq!(c[1], 1);
        assert_eq!(c[2], 2);
    }

    #[test]
    fn add_polynomial_terms_cancel() {
        let a = Polynomial::new(vec![(1, 1), (2, 2)], 41);
        let b = Polynomial::new(vec![(0, 41 - 1), (1, 41 - 1)], 41);
        let c = (&a + &b).powers;
        assert_eq!(c.len(), 2);
        assert_eq!(c[0], 0);
        assert_eq!(c[1], 2);
    }

    #[test]
    fn add_polynomial_mod_two() {
        let a = Polynomial::new(vec![(1, 1)], 2);
        let c = &a + &a;
        println!("{:?}", c);
        assert!(c.is_zero());
    }

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
