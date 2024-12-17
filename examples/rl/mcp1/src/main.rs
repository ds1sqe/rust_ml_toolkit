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
    let alpha = 0.001_f64;

    for _ in 0..50000 {
        let mut done = false;
        let mut hist = Vec::new();

        while !done {
            let next_move = Agent::select_action();
            let reword = environ.step(next_move);
            let (x, y) = environ.state();
            hist.push((x, y, reword));
            done = environ.is_done();
        }
        environ.reset();

        let mut acc = 0_f64;
        hist.reverse();
        for (x, y, reword) in hist {
            data[x][y] += alpha * (acc - data[x][y]);
            acc += gamma * reword as f64;
        }
    }

    for row in data.iter() {
        println!("{:>.1?}", row)
    }
}
