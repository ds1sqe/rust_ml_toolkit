use std::ops::Div;
use std::ops::Mul;

use super::common::sigmoid;

use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Matrix<T> {
    pub el: Box<Vec<Vec<T>>>,
}

pub trait MatrixOps<T> {
    fn new(row: usize, col: usize) -> Self;
    fn at(&self, row: usize, col: usize) -> &T;
    fn at_mut(&mut self, row: usize, col: usize) -> &mut T;
    fn len_row(&self) -> usize;
    fn len_col(&self) -> usize;
    fn row(&self, row: usize) -> &[T];
    fn row_mut(&mut self, row: usize) -> &mut [T];
    fn col(&self, col: usize) -> Vec<&T>;
    fn fill(&mut self, x: &T);
    fn sum(&mut self, with: &Self);
    fn sub(&mut self, with: &Self);
}

pub trait MatrixDiv<T, D>
where
    T: Div,
{
    fn div(&mut self, with: &D);
}

pub trait MatrixMul<T, M>
where
    M: Mul,
{
    fn mul(&mut self, with: &M);
}

impl<T> Matrix<T>
where
    Standard: Distribution<T>,
{
    pub fn rand(&mut self) {
        for row in self.el.iter_mut() {
            for col in row.iter_mut() {
                *col = rand::random::<T>();
            }
        }
    }
}

impl<T> Matrix<T>
where
    T: Default + std::ops::AddAssign + Clone + std::ops::SubAssign,
    T: std::ops::Mul<T, Output = T> + Default,
{
    pub fn dot(&mut self, mat_a: &Self, mat_b: &Self) {
        assert!(mat_a.len_col() == mat_b.len_row());
        assert!(self.len_row() == mat_a.len_row());
        assert!(self.len_col() == mat_b.len_col());

        for (ridx, row) in self.el.iter_mut().enumerate() {
            for (cidx, col) in row.iter_mut().enumerate() {
                *col = T::default();
                for tcidx in 0..mat_a.len_col() {
                    *col += mat_a.at(ridx, tcidx).clone() * mat_b.at(tcidx, cidx).clone();
                }
            }
        }
    }
}

impl<T> Clone for Matrix<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Matrix {
            el: self.el.clone(),
        }
    }
}

impl<T> MatrixOps<T> for Matrix<T>
where
    T: Default + std::ops::AddAssign + Clone + std::ops::SubAssign,
{
    fn new(row: usize, col: usize) -> Self {
        let el = Box::new(vec![vec![T::default(); col]; row]);

        Matrix { el }
    }
    fn at(&self, row: usize, col: usize) -> &T {
        assert!(self.len_row() > row);
        assert!(self.len_col() > col);
        &self.el[row][col]
    }
    fn at_mut(&mut self, row: usize, col: usize) -> &mut T {
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
    fn row(&self, row: usize) -> &[T] {
        assert!(self.len_row() > row);
        &self.el[row]
    }
    fn row_mut(&mut self, row: usize) -> &mut [T] {
        assert!(self.len_row() > row);
        &mut self.el[row]
    }

    fn col(&self, col: usize) -> Vec<&T> {
        assert!(self.len_col() > col);
        let cols = self.el.iter().map(|row| row.get(col).unwrap()).collect();
        cols
    }

    fn fill(&mut self, x: &T) {
        for row in self.el.iter_mut() {
            for col in row.iter_mut() {
                *col = x.clone();
            }
        }
    }
    fn sum(&mut self, with: &Self) {
        assert!(self.len_col() == with.len_col());
        assert!(self.len_row() == with.len_row());

        for (ridx, row) in self.el.iter_mut().enumerate() {
            for (cidx, col) in row.iter_mut().enumerate() {
                *col += with.at(ridx, cidx).clone();
            }
        }
    }

    fn sub(&mut self, with: &Self) {
        assert!(self.len_col() == with.len_col());
        assert!(self.len_row() == with.len_row());

        for (ridx, row) in self.el.iter_mut().enumerate() {
            for (cidx, col) in row.iter_mut().enumerate() {
                *col -= with.at(ridx, cidx).clone();
            }
        }
    }
}

impl MatrixDiv<f64, f64> for Matrix<f64> {
    fn div(&mut self, with: &f64) {
        for row in self.el.iter_mut() {
            for col in row.iter_mut() {
                *col /= with;
            }
        }
    }
}

impl MatrixMul<f64, f64> for Matrix<f64> {
    fn mul(&mut self, with: &f64) {
        for row in self.el.iter_mut() {
            for col in row.iter_mut() {
                *col *= with;
            }
        }
    }
}

impl Matrix<f64> {
    pub fn sigmoid(&mut self) {
        for row in self.el.iter_mut() {
            for col in row.iter_mut() {
                *col = sigmoid::sigmoid_f64(*col)
            }
        }
    }
}

#[test]
fn matrix_test_new_len() {
    let mat: Matrix<f64> = Matrix::new(3, 5);
    println!("{:?}", mat);
    assert_eq!(mat.len_row(), 3);
    assert_eq!(mat.len_col(), 5);
}
#[test]
fn matrix_test_at() {
    let mat: Matrix<f64> = Matrix::new(3, 5);
    println!("{:?}", mat);
    println!("{:?}", mat.at(2, 4));
}

#[test]
fn matrix_test_sum() {
    let mut mat_a = Matrix::new(3, 3);
    mat_a.fill(&5.0);
    let mut mat_b = Matrix::new(3, 3);
    mat_b.fill(&5.0);
    mat_a.sum(&mat_b);
    println!("{:?}", mat_a);
}

#[test]
fn matrix_test_dot() {
    let mut result = Matrix::new(4, 4);
    let mut mat_a = Matrix::new(4, 3);
    mat_a.fill(&5.0);
    let mut mat_b = Matrix::new(3, 4);
    mat_b.fill(&2.0);
    result.dot(&mat_a, &mat_b);
    println!("{:?}", result);
}

#[test]
fn matrix_test_sigmoid() {
    let mut mat = Matrix::new(3, 5);
    mat.fill(&10.0);
    println!("{:?}", mat);
    mat.sigmoid();
    println!("{:?}", mat);
}
