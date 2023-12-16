use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

use crate::core::nn::{cost::CostInfo, nn::NN};

use super::{nodes::Nodes, session::Session};

// Learner Process <- DATA(A) -- Gui Process
//
// DATA(A) {
//     Control Signal
//     Initial State (Config)...
//     Context
// }
//
// Learner Process -- DATA(B) -> Gui Process
//
// DATA(B) {
//     Renewed Context ?
//     Costs Differences
//     Nodes View
// }
//
//
// Learner Process Logic
//
// 1. Wait for signal (a microsec)
// 1.1 if there was signal,
//     handle it.
//
// 2. Train (b times)
//
// 3.1.
//   Create Nodes View
//   Get Cost Diff
// 3.2.
//   Clone Context
// 4.
//   Send data.
//
//
// Gui Process Logic
//
// 1. Wait for signal (c microsec)
//
// 2. if get DATA(B), rerender with that data.
//
// <Refresh list>
// 2.1 Nodes view.
// 2.2 Gui Context (if provided)
// 2.3 Plot of Costs

/// control signal (gui => worker)
#[derive(Debug)]
pub enum ControlSignal {
    Pause,
    Stop,
}
/// data (gui => worker)
#[derive(Debug)]
pub struct G2w {
    pub sig: ControlSignal,
}

/// data (worker => gui)
pub struct W2g {
    pub cycle: usize,
    pub cost_info: Option<CostInfo>,
    pub nodes: Option<Nodes>,
    pub model: Option<NN>,
}

pub fn spawn_learner(session: Session, rx: Receiver<G2w>) -> Receiver<W2g> {
    let (snd, rec) = channel();

    thread::spawn(move || handle(rx, snd, session));

    return rec;
}

pub fn handle(rx: Receiver<G2w>, snd: Sender<W2g>, session: Session) {
    let mut session = session;
    let mut cycle: usize = 0;

    loop {
        let g2w = rx.recv_timeout(Duration::from_micros(1000));

        match g2w {
            Ok(g2w) => {
                println!("got signal from gui thread {:?}", g2w);
                match g2w.sig {
                    ControlSignal::Pause => loop {
                        // TODO: control start here
                    },
                    ControlSignal::Stop => {
                        // manage Stop here
                        return;
                    }
                }
            }
            Err(_) => {}
        }

        session.train();

        println!("Learner>>run cycle : {} ", cycle);
        cycle += 1;

        let nodes = Some(Nodes::from(&session.model));
        let cost_info = session.cost();

        let snd_res = snd.send(W2g {
            cycle: cycle * session.option.cycle,
            cost_info,
            nodes,
            model: Some(session.model.clone()),
        });
    }
}
