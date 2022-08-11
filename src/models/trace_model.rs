pub trait Trace {
    type State;
    type States;

    fn is_empty(&self) -> bool;
    fn add_trace(&mut self, trace: Self::State);
    fn get_step(&self, n: usize) -> Option<Self::State>;
    fn next_trace(&mut self) -> Option<Self::State>;
    fn previous_trace(&mut self) -> Option<Self::State>;
    fn all_traces(&self) -> Self::States;
}
