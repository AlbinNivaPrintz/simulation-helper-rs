/*
pub trait SimulationEngine<T> {
    fn run(&mut self) -> Vec<T>;
}
*/

pub struct SerialSimulationEngine<T, F: Fn() -> T> {
    n: u64,
    simulation: F,
}

impl<T, F: Fn() -> T> SerialSimulationEngine<T, F> {
    pub fn new(n: u64, simulation: F) -> SerialSimulationEngine<T, F> {
        SerialSimulationEngine { n, simulation }
    }

    pub fn run(&self) -> Vec<T> {
        (0..self.n).map(|_| (self.simulation)()).collect()
    }
}

/*
impl<T, F: FnMut() -> T> SimulationEngine<T> for SerialSimulationEngine<T, F> {
    fn run(&mut self) -> Vec<T> {
        (0..self.n).map(|_| (self.simulation)()).collect()
    }
}
*/