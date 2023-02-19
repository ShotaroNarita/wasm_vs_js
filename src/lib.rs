use serde::{Deserialize, Serialize};
use std::ops;
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct Matrix {
    elements: Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            elements: vec![vec![0; rows]; cols],
            rows: rows,
            cols: cols,
        }
    }
}

impl ops::Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("can't mul");
        }

        let mut rev = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    rev.elements[i][j] += self.elements[i][k] * other.elements[k][j];
                    rev.elements[i][j] %= 65536;
                }
            }
        }

        return rev;
    }
}

#[wasm_bindgen]
pub async fn compute_rs(matrix: String, count: usize) -> String {

    let m: Matrix = serde_json::from_str(&matrix).unwrap();

    let mut ans: Matrix = Matrix::new(m.rows, m.cols);
    ans.elements = m.elements.clone();

    for _ in 0..count{
        ans = &ans * &m;
    }

    serde_json::to_string(&ans).unwrap()
}
