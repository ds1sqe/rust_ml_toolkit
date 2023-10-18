use ml_in_rust::nn::nn;


fn main() {
    let inputs = vec![vec![0.0,0.0],vec![0.0,1.0],vec![1.0,0.0],vec![1.0,1.0]];
    let outputs = vec![vec![1.0],vec![0.0],vec![0.0],vec![1.0]];
    let layers = [2,4,4,1];

    let eps = 1e-4;
    let rate = 1e-3;
    let mut nt = nn::NN::new(&layers);


    println!("{:?}",nt);
    nt.rand();
    println!("{:?}",nt);

    nt.learn(&inputs, &outputs, &eps, &rate);

    println!("{:?}",nt);

    for idx in 0..1000 {
    nt.learn(&inputs, &outputs, &eps, &rate);
    }

    println!("{:?}",nt);
}
