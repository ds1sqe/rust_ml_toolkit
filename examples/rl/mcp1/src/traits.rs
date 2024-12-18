pub trait Environment {
    type Action;
    type Reward;
    type State;

    fn step(&mut self, action: Self::Action) -> Self::Reward;

    fn state(&self) -> Self::State;

    fn is_done(&self) -> bool;

    fn reset(&mut self);
}
