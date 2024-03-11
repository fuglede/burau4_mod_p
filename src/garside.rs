use hashbrown::HashMap;

use crate::algebra::{Matrix, Polynomial};

pub fn generate_descendants() -> HashMap<u32, Vec<u32>> {
    let mut d = HashMap::new();
    d.insert(1, vec![1, 4, 18]);
    d.insert(2, vec![2, 3, 12, 13, 16]);
    d.insert(3, vec![1, 4, 18]);
    d.insert(4, vec![2, 3, 12, 13, 16]);
    d.insert(5, vec![1, 2, 3, 4, 5, 12, 13, 16, 18, 19, 22]);
    d.insert(6, vec![6, 8, 9]);
    d.insert(7, vec![1, 4, 6, 7, 8, 9, 10, 11, 18, 20, 21]);
    d.insert(8, vec![2, 3, 12, 13, 16]);
    d.insert(9, vec![1, 4, 18]);
    d.insert(10, vec![2, 3, 12, 13, 16]);
    d.insert(11, vec![1, 2, 3, 4, 5, 12, 13, 16, 18, 19, 22]);
    d.insert(12, vec![6, 8, 9]);
    d.insert(13, vec![1, 4, 6, 7, 8, 9, 10, 11, 18, 20, 21]);
    d.insert(14, vec![2, 3, 6, 8, 9, 12, 13, 14, 15, 16, 17]);
    d.insert(15, vec![1, 4, 6, 7, 8, 9, 10, 11, 18, 20, 21]);
    d.insert(16, vec![2, 3, 12, 13, 16]);
    d.insert(17, vec![1, 2, 3, 4, 5, 12, 13, 16, 18, 19, 22]);
    d.insert(18, vec![6, 8, 9]);
    d.insert(19, vec![1, 4, 6, 7, 8, 9, 10, 11, 18, 20, 21]);
    d.insert(20, vec![2, 3, 6, 8, 9, 12, 13, 14, 15, 16, 17]);
    d.insert(21, vec![1, 4, 6, 7, 8, 9, 10, 11, 18, 20, 21]);
    d.insert(22, vec![2, 3, 6, 8, 9, 12, 13, 14, 15, 16, 17]);
    d
}

pub fn act_by(mat: &Matrix, element: u32, p: u8) -> Matrix {
    let d = match element {
        1 => [
            mat.d[0].mult(0, false),
            mat.d[1].mult(0, false) + mat.d[2].mult(1, true),
            mat.d[2].mult(2, true),
            mat.d[3].mult(0, false),
            mat.d[4].mult(0, false) + mat.d[5].mult(1, true),
            mat.d[5].mult(2, true),
            mat.d[6].mult(0, false),
            mat.d[7].mult(0, false) + mat.d[8].mult(1, true),
            mat.d[8].mult(2, true),
        ],
        2 => [
            mat.d[0].mult(0, false) + mat.d[1].mult(1, true),
            mat.d[1].mult(2, true),
            mat.d[1].mult(1, true) + mat.d[2].mult(0, false),
            mat.d[3].mult(0, false) + mat.d[4].mult(1, true),
            mat.d[4].mult(2, true),
            mat.d[4].mult(1, true) + mat.d[5].mult(0, false),
            mat.d[6].mult(0, false) + mat.d[7].mult(1, true),
            mat.d[7].mult(2, true),
            mat.d[7].mult(1, true) + mat.d[8].mult(0, false),
        ],
        3 => [
            mat.d[0].mult(0, false) + mat.d[1].mult(1, true),
            mat.d[2].mult(1, true),
            mat.d[1].mult(3, false) + mat.d[2].mult(2, true),
            mat.d[3].mult(0, false) + mat.d[4].mult(1, true),
            mat.d[5].mult(1, true),
            mat.d[4].mult(3, false) + mat.d[5].mult(2, true),
            mat.d[6].mult(0, false) + mat.d[7].mult(1, true),
            mat.d[8].mult(1, true),
            mat.d[7].mult(3, false) + mat.d[8].mult(2, true),
        ],
        4 => [
            mat.d[0].mult(0, false) + mat.d[1].mult(1, true) + mat.d[2].mult(2, false),
            mat.d[1].mult(2, true) + mat.d[2].mult(3, false),
            mat.d[1].mult(1, true),
            mat.d[3].mult(0, false) + mat.d[4].mult(1, true) + mat.d[5].mult(2, false),
            mat.d[4].mult(2, true) + mat.d[5].mult(3, false),
            mat.d[4].mult(1, true),
            mat.d[6].mult(0, false) + mat.d[7].mult(1, true) + mat.d[8].mult(2, false),
            mat.d[7].mult(2, true) + mat.d[8].mult(3, false),
            mat.d[7].mult(1, true),
        ],
        5 => [
            mat.d[0].mult(0, false) + mat.d[1].mult(1, true) + mat.d[2].mult(2, false),
            mat.d[2].mult(3, false),
            mat.d[1].mult(3, false),
            mat.d[3].mult(0, false) + mat.d[4].mult(1, true) + mat.d[5].mult(2, false),
            mat.d[5].mult(3, false),
            mat.d[4].mult(3, false),
            mat.d[6].mult(0, false) + mat.d[7].mult(1, true) + mat.d[8].mult(2, false),
            mat.d[8].mult(3, false),
            mat.d[7].mult(3, false),
        ],
        6 => [
            mat.d[0].mult(2, true),
            mat.d[0].mult(1, true) + mat.d[1].mult(0, false),
            mat.d[2].mult(0, false),
            mat.d[3].mult(2, true),
            mat.d[3].mult(1, true) + mat.d[4].mult(0, false),
            mat.d[5].mult(0, false),
            mat.d[6].mult(2, true),
            mat.d[6].mult(1, true) + mat.d[7].mult(0, false),
            mat.d[8].mult(0, false),
        ],
        7 => [
            mat.d[0].mult(2, true),
            mat.d[0].mult(1, true) + mat.d[1].mult(0, false) + mat.d[2].mult(1, true),
            mat.d[2].mult(2, true),
            mat.d[3].mult(2, true),
            mat.d[3].mult(1, true) + mat.d[4].mult(0, false) + mat.d[5].mult(1, true),
            mat.d[5].mult(2, true),
            mat.d[6].mult(2, true),
            mat.d[6].mult(1, true) + mat.d[7].mult(0, false) + mat.d[8].mult(1, true),
            mat.d[8].mult(2, true),
        ],
        8 => [
            mat.d[1].mult(1, true),
            mat.d[0].mult(3, false) + mat.d[1].mult(2, true),
            mat.d[0].mult(2, false) + mat.d[1].mult(1, true) + mat.d[2].mult(0, false),
            mat.d[4].mult(1, true),
            mat.d[3].mult(3, false) + mat.d[4].mult(2, true),
            mat.d[3].mult(2, false) + mat.d[4].mult(1, true) + mat.d[5].mult(0, false),
            mat.d[7].mult(1, true),
            mat.d[6].mult(3, false) + mat.d[7].mult(2, true),
            mat.d[6].mult(2, false) + mat.d[7].mult(1, true) + mat.d[8].mult(0, false),
        ],
        9 => [
            mat.d[1].mult(1, true),
            mat.d[2].mult(1, true),
            mat.d[0].mult(4, true) + mat.d[1].mult(3, false) + mat.d[2].mult(2, true),
            mat.d[4].mult(1, true),
            mat.d[5].mult(1, true),
            mat.d[3].mult(4, true) + mat.d[4].mult(3, false) + mat.d[5].mult(2, true),
            mat.d[7].mult(1, true),
            mat.d[8].mult(1, true),
            mat.d[6].mult(4, true) + mat.d[7].mult(3, false) + mat.d[8].mult(2, true),
        ],
        10 => [
            mat.d[1].mult(1, true) + mat.d[2].mult(2, false),
            mat.d[0].mult(3, false) + mat.d[1].mult(2, true) + mat.d[2].mult(3, false),
            mat.d[0].mult(2, false) + mat.d[1].mult(1, true),
            mat.d[4].mult(1, true) + mat.d[5].mult(2, false),
            mat.d[3].mult(3, false) + mat.d[4].mult(2, true) + mat.d[5].mult(3, false),
            mat.d[3].mult(2, false) + mat.d[4].mult(1, true),
            mat.d[7].mult(1, true) + mat.d[8].mult(2, false),
            mat.d[6].mult(3, false) + mat.d[7].mult(2, true) + mat.d[8].mult(3, false),
            mat.d[6].mult(2, false) + mat.d[7].mult(1, true),
        ],
        11 => [
            mat.d[1].mult(1, true) + mat.d[2].mult(2, false),
            mat.d[2].mult(3, false),
            mat.d[0].mult(4, true) + mat.d[1].mult(3, false),
            mat.d[4].mult(1, true) + mat.d[5].mult(2, false),
            mat.d[5].mult(3, false),
            mat.d[3].mult(4, true) + mat.d[4].mult(3, false),
            mat.d[7].mult(1, true) + mat.d[8].mult(2, false),
            mat.d[8].mult(3, false),
            mat.d[6].mult(4, true) + mat.d[7].mult(3, false),
        ],
        12 => [
            mat.d[0].mult(2, true) + mat.d[1].mult(3, false),
            mat.d[0].mult(1, true),
            mat.d[1].mult(1, true) + mat.d[2].mult(0, false),
            mat.d[3].mult(2, true) + mat.d[4].mult(3, false),
            mat.d[3].mult(1, true),
            mat.d[4].mult(1, true) + mat.d[5].mult(0, false),
            mat.d[6].mult(2, true) + mat.d[7].mult(3, false),
            mat.d[6].mult(1, true),
            mat.d[7].mult(1, true) + mat.d[8].mult(0, false),
        ],
        13 => [
            mat.d[0].mult(2, true) + mat.d[1].mult(3, false),
            mat.d[0].mult(1, true) + mat.d[1].mult(2, false) + mat.d[2].mult(1, true),
            mat.d[1].mult(3, false) + mat.d[2].mult(2, true),
            mat.d[3].mult(2, true) + mat.d[4].mult(3, false),
            mat.d[3].mult(1, true) + mat.d[4].mult(2, false) + mat.d[5].mult(1, true),
            mat.d[4].mult(3, false) + mat.d[5].mult(2, true),
            mat.d[6].mult(2, true) + mat.d[7].mult(3, false),
            mat.d[6].mult(1, true) + mat.d[7].mult(2, false) + mat.d[8].mult(1, true),
            mat.d[7].mult(3, false) + mat.d[8].mult(2, true),
        ],
        14 => [
            mat.d[1].mult(3, false),
            mat.d[0].mult(3, false),
            mat.d[0].mult(2, false) + mat.d[1].mult(1, true) + mat.d[2].mult(0, false),
            mat.d[4].mult(3, false),
            mat.d[3].mult(3, false),
            mat.d[3].mult(2, false) + mat.d[4].mult(1, true) + mat.d[5].mult(0, false),
            mat.d[7].mult(3, false),
            mat.d[6].mult(3, false),
            mat.d[6].mult(2, false) + mat.d[7].mult(1, true) + mat.d[8].mult(0, false),
        ],
        15 => [
            mat.d[1].mult(3, false),
            mat.d[1].mult(2, false) + mat.d[2].mult(1, true),
            mat.d[0].mult(4, true) + mat.d[1].mult(3, false) + mat.d[2].mult(2, true),
            mat.d[4].mult(3, false),
            mat.d[4].mult(2, false) + mat.d[5].mult(1, true),
            mat.d[3].mult(4, true) + mat.d[4].mult(3, false) + mat.d[5].mult(2, true),
            mat.d[7].mult(3, false),
            mat.d[7].mult(2, false) + mat.d[8].mult(1, true),
            mat.d[6].mult(4, true) + mat.d[7].mult(3, false) + mat.d[8].mult(2, true),
        ],
        16 => [
            mat.d[2].mult(2, false),
            mat.d[0].mult(3, false) + mat.d[1].mult(4, true) + mat.d[2].mult(3, false),
            mat.d[0].mult(2, false),
            mat.d[5].mult(2, false),
            mat.d[3].mult(3, false) + mat.d[4].mult(4, true) + mat.d[5].mult(3, false),
            mat.d[3].mult(2, false),
            mat.d[8].mult(2, false),
            mat.d[6].mult(3, false) + mat.d[7].mult(4, true) + mat.d[8].mult(3, false),
            mat.d[6].mult(2, false),
        ],
        17 => [
            mat.d[2].mult(2, false),
            mat.d[1].mult(4, true) + mat.d[2].mult(3, false),
            mat.d[0].mult(4, true),
            mat.d[5].mult(2, false),
            mat.d[4].mult(4, true) + mat.d[5].mult(3, false),
            mat.d[3].mult(4, true),
            mat.d[8].mult(2, false),
            mat.d[7].mult(4, true) + mat.d[8].mult(3, false),
            mat.d[6].mult(4, true),
        ],
        18 => [
            mat.d[0].mult(2, true) + mat.d[1].mult(3, false) + mat.d[2].mult(4, true),
            mat.d[0].mult(1, true),
            mat.d[1].mult(1, true),
            mat.d[3].mult(2, true) + mat.d[4].mult(3, false) + mat.d[5].mult(4, true),
            mat.d[3].mult(1, true),
            mat.d[4].mult(1, true),
            mat.d[6].mult(2, true) + mat.d[7].mult(3, false) + mat.d[8].mult(4, true),
            mat.d[6].mult(1, true),
            mat.d[7].mult(1, true),
        ],
        19 => [
            mat.d[0].mult(2, true) + mat.d[1].mult(3, false) + mat.d[2].mult(4, true),
            mat.d[0].mult(1, true) + mat.d[1].mult(2, false),
            mat.d[1].mult(3, false),
            mat.d[3].mult(2, true) + mat.d[4].mult(3, false) + mat.d[5].mult(4, true),
            mat.d[3].mult(1, true) + mat.d[4].mult(2, false),
            mat.d[4].mult(3, false),
            mat.d[6].mult(2, true) + mat.d[7].mult(3, false) + mat.d[8].mult(4, true),
            mat.d[6].mult(1, true) + mat.d[7].mult(2, false),
            mat.d[7].mult(3, false),
        ],
        20 => [
            mat.d[1].mult(3, false) + mat.d[2].mult(4, true),
            mat.d[0].mult(3, false),
            mat.d[0].mult(2, false) + mat.d[1].mult(1, true),
            mat.d[4].mult(3, false) + mat.d[5].mult(4, true),
            mat.d[3].mult(3, false),
            mat.d[3].mult(2, false) + mat.d[4].mult(1, true),
            mat.d[7].mult(3, false) + mat.d[8].mult(4, true),
            mat.d[6].mult(3, false),
            mat.d[6].mult(2, false) + mat.d[7].mult(1, true),
        ],
        21 => [
            mat.d[1].mult(3, false) + mat.d[2].mult(4, true),
            mat.d[1].mult(2, false),
            mat.d[0].mult(4, true) + mat.d[1].mult(3, false),
            mat.d[4].mult(3, false) + mat.d[5].mult(4, true),
            mat.d[4].mult(2, false),
            mat.d[3].mult(4, true) + mat.d[4].mult(3, false),
            mat.d[7].mult(3, false) + mat.d[8].mult(4, true),
            mat.d[7].mult(2, false),
            mat.d[6].mult(4, true) + mat.d[7].mult(3, false),
        ],
        _ => [
            mat.d[2].mult(4, true),
            mat.d[0].mult(3, false) + mat.d[1].mult(4, true),
            mat.d[0].mult(2, false),
            mat.d[5].mult(4, true),
            mat.d[3].mult(3, false) + mat.d[4].mult(4, true),
            mat.d[3].mult(2, false),
            mat.d[8].mult(4, true),
            mat.d[6].mult(3, false) + mat.d[7].mult(4, true),
            mat.d[6].mult(2, false),
        ],
    };

    Matrix { d, p }
}

pub fn generate_matrix_map(p: u8) -> HashMap<u32, Matrix> {
    let mut d = HashMap::new();
    let mut mat = Matrix::zero(p);
    mat.d[0] = Polynomial::new(vec![(0, 1)], p);
    mat.d[4] = Polynomial::new(vec![(0, 1)], p);
    mat.d[7] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[8] = Polynomial::new(vec![(2, p - 1)], p);
    d.insert(1, mat);
    let mut mat = Matrix::zero(p);
    mat.d[0] = Polynomial::new(vec![(0, 1)], p);
    mat.d[3] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[4] = Polynomial::new(vec![(2, p - 1)], p);
    mat.d[5] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[8] = Polynomial::new(vec![(0, 1)], p);
    d.insert(2, mat);
    let mut mat = Matrix::zero(p);
    mat.d[0] = Polynomial::new(vec![(0, 1)], p);
    mat.d[3] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[5] = Polynomial::new(vec![(3, 1)], p);
    mat.d[7] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[8] = Polynomial::new(vec![(2, p - 1)], p);
    d.insert(3, mat);
    let mut mat = Matrix::zero(p);
    mat.d[0] = Polynomial::new(vec![(0, 1)], p);
    mat.d[3] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[4] = Polynomial::new(vec![(2, p - 1)], p);
    mat.d[5] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[6] = Polynomial::new(vec![(2, 1)], p);
    mat.d[7] = Polynomial::new(vec![(3, 1)], p);
    d.insert(4, mat);
    let mut mat = Matrix::zero(p);
    mat.d[0] = Polynomial::new(vec![(0, 1)], p);
    mat.d[3] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[5] = Polynomial::new(vec![(3, 1)], p);
    mat.d[6] = Polynomial::new(vec![(2, 1)], p);
    mat.d[7] = Polynomial::new(vec![(3, 1)], p);
    d.insert(5, mat);
    let mut mat = Matrix::zero(p);
    mat.d[0] = Polynomial::new(vec![(2, p - 1)], p);
    mat.d[1] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[4] = Polynomial::new(vec![(0, 1)], p);
    mat.d[8] = Polynomial::new(vec![(0, 1)], p);
    d.insert(6, mat);
    let mut mat = Matrix::zero(p);
    mat.d[0] = Polynomial::new(vec![(2, p - 1)], p);
    mat.d[1] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[4] = Polynomial::new(vec![(0, 1)], p);
    mat.d[7] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[8] = Polynomial::new(vec![(2, p - 1)], p);
    d.insert(7, mat);
    let mut mat = Matrix::zero(p);
    mat.d[1] = Polynomial::new(vec![(3, 1)], p);
    mat.d[2] = Polynomial::new(vec![(2, 1)], p);
    mat.d[3] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[4] = Polynomial::new(vec![(2, p - 1)], p);
    mat.d[5] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[8] = Polynomial::new(vec![(0, 1)], p);
    d.insert(8, mat);
    let mut mat = Matrix::zero(p);
    mat.d[2] = Polynomial::new(vec![(4, p - 1)], p);
    mat.d[3] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[5] = Polynomial::new(vec![(3, 1)], p);
    mat.d[7] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[8] = Polynomial::new(vec![(2, p - 1)], p);
    d.insert(9, mat);
    let mut mat = Matrix::zero(p);
    mat.d[1] = Polynomial::new(vec![(3, 1)], p);
    mat.d[2] = Polynomial::new(vec![(2, 1)], p);
    mat.d[3] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[4] = Polynomial::new(vec![(2, p - 1)], p);
    mat.d[5] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[6] = Polynomial::new(vec![(2, 1)], p);
    mat.d[7] = Polynomial::new(vec![(3, 1)], p);
    d.insert(10, mat);
    let mut mat = Matrix::zero(p);
    mat.d[2] = Polynomial::new(vec![(4, p - 1)], p);
    mat.d[3] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[5] = Polynomial::new(vec![(3, 1)], p);
    mat.d[6] = Polynomial::new(vec![(2, 1)], p);
    mat.d[7] = Polynomial::new(vec![(3, 1)], p);
    d.insert(11, mat);
    let mut mat = Matrix::zero(p);
    mat.d[0] = Polynomial::new(vec![(2, p - 1)], p);
    mat.d[1] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[3] = Polynomial::new(vec![(3, 1)], p);
    mat.d[5] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[8] = Polynomial::new(vec![(0, 1)], p);
    d.insert(12, mat);
    let mut mat = Matrix::zero(p);
    mat.d[0] = Polynomial::new(vec![(2, p - 1)], p);
    mat.d[1] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[3] = Polynomial::new(vec![(3, 1)], p);
    mat.d[4] = Polynomial::new(vec![(2, 1)], p);
    mat.d[5] = Polynomial::new(vec![(3, 1)], p);
    mat.d[7] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[8] = Polynomial::new(vec![(2, p - 1)], p);
    d.insert(13, mat);
    let mut mat = Matrix::zero(p);
    mat.d[1] = Polynomial::new(vec![(3, 1)], p);
    mat.d[2] = Polynomial::new(vec![(2, 1)], p);
    mat.d[3] = Polynomial::new(vec![(3, 1)], p);
    mat.d[5] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[8] = Polynomial::new(vec![(0, 1)], p);
    d.insert(14, mat);
    let mut mat = Matrix::zero(p);
    mat.d[2] = Polynomial::new(vec![(4, p - 1)], p);
    mat.d[3] = Polynomial::new(vec![(3, 1)], p);
    mat.d[4] = Polynomial::new(vec![(2, 1)], p);
    mat.d[5] = Polynomial::new(vec![(3, 1)], p);
    mat.d[7] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[8] = Polynomial::new(vec![(2, p - 1)], p);
    d.insert(15, mat);
    let mut mat = Matrix::zero(p);
    mat.d[1] = Polynomial::new(vec![(3, 1)], p);
    mat.d[2] = Polynomial::new(vec![(2, 1)], p);
    mat.d[4] = Polynomial::new(vec![(4, p - 1)], p);
    mat.d[6] = Polynomial::new(vec![(2, 1)], p);
    mat.d[7] = Polynomial::new(vec![(3, 1)], p);
    d.insert(16, mat);
    let mut mat = Matrix::zero(p);
    mat.d[2] = Polynomial::new(vec![(4, p - 1)], p);
    mat.d[4] = Polynomial::new(vec![(4, p - 1)], p);
    mat.d[6] = Polynomial::new(vec![(2, 1)], p);
    mat.d[7] = Polynomial::new(vec![(3, 1)], p);
    d.insert(17, mat);
    let mut mat = Matrix::zero(p);
    mat.d[0] = Polynomial::new(vec![(2, p - 1)], p);
    mat.d[1] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[3] = Polynomial::new(vec![(3, 1)], p);
    mat.d[5] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[6] = Polynomial::new(vec![(4, p - 1)], p);
    d.insert(18, mat);
    let mut mat = Matrix::zero(p);
    mat.d[0] = Polynomial::new(vec![(2, p - 1)], p);
    mat.d[1] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[3] = Polynomial::new(vec![(3, 1)], p);
    mat.d[4] = Polynomial::new(vec![(2, 1)], p);
    mat.d[5] = Polynomial::new(vec![(3, 1)], p);
    mat.d[6] = Polynomial::new(vec![(4, p - 1)], p);
    d.insert(19, mat);
    let mut mat = Matrix::zero(p);
    mat.d[1] = Polynomial::new(vec![(3, 1)], p);
    mat.d[2] = Polynomial::new(vec![(2, 1)], p);
    mat.d[3] = Polynomial::new(vec![(3, 1)], p);
    mat.d[5] = Polynomial::new(vec![(1, p - 1)], p);
    mat.d[6] = Polynomial::new(vec![(4, p - 1)], p);
    d.insert(20, mat);
    let mut mat = Matrix::zero(p);
    mat.d[2] = Polynomial::new(vec![(4, p - 1)], p);
    mat.d[3] = Polynomial::new(vec![(3, 1)], p);
    mat.d[4] = Polynomial::new(vec![(2, 1)], p);
    mat.d[5] = Polynomial::new(vec![(3, 1)], p);
    mat.d[6] = Polynomial::new(vec![(4, p - 1)], p);
    d.insert(21, mat);
    let mut mat = Matrix::zero(p);
    mat.d[1] = Polynomial::new(vec![(3, 1)], p);
    mat.d[2] = Polynomial::new(vec![(2, 1)], p);
    mat.d[4] = Polynomial::new(vec![(4, p - 1)], p);
    mat.d[6] = Polynomial::new(vec![(4, p - 1)], p);
    d.insert(22, mat);
    d
}
