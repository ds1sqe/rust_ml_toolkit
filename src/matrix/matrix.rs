use crate::common::sigmoid;

#[derive(Debug)]
pub struct Matrix<T> {
    el: Box<Vec<Vec<T>>>,
}

pub trait __Matrix<T> {
    fn new(row: usize, col: usize) -> Self;
    fn at(&self, row: usize, col: usize) -> T;
    fn at_mut(&mut self, row: usize, col: usize) -> &mut T;
    fn len_row(&self) -> usize;
    fn len_col(&self) -> usize;
    fn row(&self, row: usize) -> &[T];
    fn row_mut(&mut self, row: usize) -> &mut [T];
    fn col(&self, col: usize) -> Vec<&T>;
    fn rand(&mut self);
    fn fill(&mut self, x: T);
    fn sum(&mut self, with: &Self);
    fn sub(&mut self, with: &Self);
    fn mul(&mut self, with: &f64);
    fn div(&mut self, with: &f64);
    fn dot(&mut self, mat_a: &Self, mat_b: &Self);
    fn sigmoid(&mut self);
}

impl Clone for Matrix<f64> {
    fn clone(&self) -> Self {
        let new_one = Matrix {
            el: self.el.clone(),
        };
        return new_one;
    }
}

impl __Matrix<f64> for Matrix<f64> {
    fn new(row: usize, col: usize) -> Self {
        let el = Box::new(vec![vec![0.0; col]; row]);

        return Matrix { el };
    }
    fn at(&self, row: usize, col: usize) -> f64 {
        assert!(self.len_row() > row);
        assert!(self.len_col() > col);
        self.el[row][col]
    }
    fn at_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        assert!(self.len_row() > row);
        assert!(self.len_col() > col);
        &mut self.el[row][col]
    }
    fn len_row(&self) -> usize {
        self.el.len()
    }
    fn len_col(&self) -> usize {
        self.el[0].len()
    }
    fn row(&self, row: usize) -> &[f64] {
        assert!(self.len_row() > row);
        return &self.el[row];
    }
    fn row_mut(&mut self, row: usize) -> &mut [f64] {
        assert!(self.len_row() > row);
        return &mut self.el[row];
    }

    fn col(&self, col: usize) -> Vec<&f64> {
        assert!(self.len_col() > col);
        let cols = self.el.iter().map(|row| row.get(col).unwrap()).collect();
        cols
    }
    fn rand(&mut self) {
        for row in self.el.iter_mut() {
            for col in row.iter_mut() {
                *col = rand::random();
            }
        }
    }
    fn fill(&mut self, x: f64) {
        for row in self.el.iter_mut() {
            for col in row.iter_mut() {
                *col = x;
            }
        }
    }
    fn sum(&mut self, with: &Self) {
        assert!(self.len_col() == with.len_col());
        assert!(self.len_row() == with.len_row());

        for (ridx, row) in self.el.iter_mut().enumerate() {
            for (cidx, col) in row.iter_mut().enumerate() {
                *col = *col + with.at(ridx, cidx);
            }
        }
    }

    fn sub(&mut self, with: &Self) {
        assert!(self.len_col() == with.len_col());
        assert!(self.len_row() == with.len_row());

        for (ridx, row) in self.el.iter_mut().enumerate() {
            for (cidx, col) in row.iter_mut().enumerate() {
                *col = *col - with.at(ridx, cidx);
            }
        }
    }

    fn mul(&mut self, with: &f64) {
        for (ridx, row) in self.el.iter_mut().enumerate() {
            for (cidx, col) in row.iter_mut().enumerate() {
                *col = *col * with;
            }
        }
    }

    fn div(&mut self, with: &f64) {
        for (ridx, row) in self.el.iter_mut().enumerate() {
            for (cidx, col) in row.iter_mut().enumerate() {
                *col = *col / with;
            }
        }
    }

    fn dot(&mut self, mat_a: &Self, mat_b: &Self) {
        assert!(mat_a.len_col() == mat_b.len_row());
        assert!(self.len_row() == mat_a.len_row());
        assert!(self.len_col() == mat_b.len_col());

        for (ridx, row) in self.el.iter_mut().enumerate() {
            for (cidx, col) in row.iter_mut().enumerate() {
                *col = 0.0;
                for tcidx in 0..mat_a.len_col() {
                    *col += mat_a.at(ridx, tcidx) * mat_b.at(tcidx, cidx);
                }
            }
        }
    }
    fn sigmoid(&mut self) {
        for row in self.el.iter_mut() {
            for col in row.iter_mut() {
                *col = sigmoid::sigmoid_f64(*col)
            }
        }
    }
}

#[test]
fn matrix_test_new_len() {
    let mat = Matrix::new(3, 5);
    println!("{:?}", mat);
    assert_eq!(mat.len_row(), 3);
    assert_eq!(mat.len_col(), 5);
}
#[test]
fn matrix_test_at() {
    let mat = Matrix::new(3, 5);
    println!("{:?}", mat);
    println!("{:?}", mat.at(2, 4));
}

#[test]
fn matrix_test_sum() {
    let mut mat_a = Matrix::new(3, 3);
    mat_a.fill(5.0);
    let mut mat_b = Matrix::new(3, 3);
    mat_b.fill(5.0);
    mat_a.sum(&mat_b);
    println!("{:?}", mat_a);
}

#[test]
fn matrix_test_dot() {
    let mut result = Matrix::new(4, 4);
    let mut mat_a = Matrix::new(4, 3);
    mat_a.fill(5.0);
    let mut mat_b = Matrix::new(3, 4);
    mat_b.fill(2.0);
    result.dot(&mat_a, &mat_b);
    println!("{:?}", result);
}

#[test]
fn matrix_test_sigmoid() {
    let mut mat = Matrix::new(3, 5);
    mat.fill(10.0);
    println!("{:?}", mat);
    mat.sigmoid();
    println!("{:?}", mat);
}
