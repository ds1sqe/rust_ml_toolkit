mod traits;

mod agent;
mod grid;

use agent::Agent;
use grid::GridWorld;
use traits::Environment;

fn main() {
    let mut environ = GridWorld::new(3, 3);

    let mut data: Vec<Vec<f64>> = vec![
        vec![0_f64, 0_f64, 0_f64, 0_f64],
        vec![0_f64, 0_f64, 0_f64, 0_f64],
        vec![0_f64, 0_f64, 0_f64, 0_f64],
        vec![0_f64, 0_f64, 0_f64, 0_f64],
    ];

    let gamma = 1.0_f64;
    let alpha = 0.01_f64;

    for _ in 0..50000 {
        let mut done = false;

        while !done {
            let (x, y) = environ.state();
            let next_move = Agent::select_action();
            let reword = environ.step(next_move);
            done = environ.is_done();
            let (x_prime, y_prime) = environ.state();

            data[x][y] += alpha * (reword as f64 + gamma * data[x_prime][y_prime] - data[x][y]);
        }
        environ.reset();
    }

    for row in data.iter() {
        println!("{:>.1?}", row)
    }
}
