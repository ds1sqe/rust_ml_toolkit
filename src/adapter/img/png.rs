use std::fs::File;

use log::debug;
use png::ColorType;

use crate::core::matrix::matrix::{Matrix, __Matrix};

#[derive(Debug)]
pub enum ImgError {
    DecodeError,
}

/// decode provided img into matrix form.
/// can only decode 8bit grayscale
/// @arg file : png img file
///
/// # Errors ImgError::DecodeError
///
/// This function will return an error if failed to decode provided img
///
pub fn png2mat(file: File) -> Result<Matrix<f64>, ImgError> {
    let decoder = png::Decoder::new(file);

    let mut reader = decoder.read_info().unwrap();

    if reader.info().color_type != ColorType::Grayscale {
        debug!("img decode failed: ColorType != Grayscale");
        return Err(ImgError::DecodeError);
    }

    let mut buf = vec![0; reader.output_buffer_size()];

    let info = reader.next_frame(&mut buf).unwrap();

    let bytes = &buf[..info.buffer_size()];

    let mut mat = Matrix::new((info.width * info.height) as usize, 3);

    for yidx in 0..info.height {
        for xidx in 0..info.width {
            let idx = (yidx * info.width + xidx) as usize;
            *mat.at_mut(idx, 0) = xidx as f64 / (info.width - 1) as f64;
            *mat.at_mut(idx, 1) = yidx as f64 / (info.height - 1) as f64;
            *mat.at_mut(idx, 2) = bytes[idx] as f64 / 255_f64;
        }
    }

    return Ok(mat);
}

#[test]
fn test_png2mat() {
    println!("Converting file to matrix...");

    let file = File::open("data/img/n9.png").unwrap();
    const WIDTH: usize = 28;
    const HEIGHT: usize = 28;
    let mat = png2mat(file).unwrap();

    println!("printing matrix...");

    for yidx in 0..HEIGHT {
        for xidx in 0..WIDTH {
            let idx = (yidx * WIDTH + xidx) as usize;
            print!("{:>4}", (mat.at(idx, 2) * 255 as f64) as u8)
        }
        println!()
    }
}
