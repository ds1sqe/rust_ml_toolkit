use std::path::Path;

use rust_ml_toolkit::{adapter::context::Context, core::nn::dataset::DataSet};

fn main() {
    /// this is example of creating adder model

    const BITS: usize = 4;

    let max: usize = 1 << BITS;

    let mut input: Vec<Vec<f64>> = vec![];
    let mut output: Vec<Vec<f64>> = vec![];

    for input_x in 0..max {
        for input_y in 0..max {
            let sum = input_x + input_y;

            // carry flag
            let carry = sum >= max;

            let mut cur_input = vec![0_f64; BITS * 2];
            let mut cur_output = vec![0_f64; BITS + 1];

            for bit in 0..BITS {
                cur_input[bit] = ((input_x >> bit) & 1) as f64;
                cur_input[bit + BITS] = ((input_y >> bit) & 1) as f64;

                cur_output[bit] = ((sum >> bit) & 1) as f64;
            }
            cur_output[BITS] = carry as u8 as f64;

            input.push(cur_input);
            output.push(cur_output);
        }
    }

    let layers = [BITS * 2, BITS * 2, BITS * 2, BITS + 1];
    let mut ctx = Context::default();
    ctx.create_model(
        &layers,
        rust_ml_toolkit::adapter::session::TrainingMethod::BackProp,
        rust_ml_toolkit::adapter::session::PostX::Sigmoid,
        50,
    );

    let dataset = DataSet::new(input, output);
    ctx.attach_dataset(dataset);

    ctx.save_session(Path::new("save/adder"));
}
