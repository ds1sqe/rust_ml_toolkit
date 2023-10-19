use ml_in_rust::nn::nn;

fn main() {
    // using finite_diff

    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];
    let outputs = vec![vec![1.0], vec![0.0], vec![0.0], vec![1.0]];
    let layers = [2, 4, 4, 1];

    let eps = 1e-4;
    let rate = 1e-2;
    let mut nt = nn::NN::new(&layers);

    nt.rand();
    println!("{:?}", nt);

    for input in inputs.clone() {
        println!("input : {:?}", input);
        nt.set(input.as_slice());
        nt.process();
        println!("output : {:?}", nt.output());
    }

    for idx in 0..1000000 {
        let mut delta = nt.finite_diff(&inputs, &outputs, &eps);
        delta.mul(&rate);

        nt.learn(&delta);
        if idx % 1000 == 0 {
            println!("{}@cost: {}", idx, nt.cost(&inputs, &outputs));
        }
    }

    for input in inputs {
        println!("input : {:?}", input);
        nt.set(input.as_slice());
        nt.process();
        println!("output : {:?}", nt.output());
    }

    println!("{:?}", nt);
}
