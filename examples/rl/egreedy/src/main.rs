mod traits;

mod grid;
mod qagent;

use grid::GridWorld;
use qagent::{QAgent, State, Transition};
use traits::Environment;

//
//      0  1  2  3  4  5  6
//    |--|--|--|--|--|--|--|
//  0 |  |  |XX|  |  |  |  |
//    |--|--|--|--|--|--|--|
//  1 |  |  |XX|  |  |  |  |
//    |--|--|--|--|--|--|--|
//  2 |ST|  |XX|  |XX|  |  |
//    |--|--|--|--|--|--|--|
//  3 |  |  |  |  |XX|  |  |
//    |--|--|--|--|--|--|--|
//  4 |  |  |  |  |XX|  | G|
//    |--|--|--|--|--|--|--|

fn main() {
    let mut environ = GridWorld::new(6, 4);

    let mut agent = QAgent::build();

    for idx in 0..5000 {
        let mut done = false;
        let mut hist = Vec::new();

        environ.reset();
        while !done {
            let next_move = agent.select_action(environ.state());
            let reward = environ.step(next_move.clone());
            let (x, y) = environ.state();
            let state = State { x, y };
            hist.push(Transition {
                state,
                act: next_move,
                reward,
            });
            done = environ.is_done();
        }
        agent.update_table(hist);
        agent.anneal_eps();
        if idx % 50 == 0 {
            println!("Index: {}", idx);
            agent.show_table();
        }
    }
    agent.show_table();

    let mut done = false;
    let mut hist = Vec::new();

    environ.reset();
    while !done {
        environ.print_state();
        let next_move = agent.select_action(environ.state());
        println!("Next move: {:?}", next_move);
        let reward = environ.step(next_move.clone());
        let (x, y) = environ.state();
        let state = State { x, y };
        hist.push(Transition {
            state,
            act: next_move,
            reward,
        });
        done = environ.is_done();
    }
}
