use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator};
use rayon::prelude::*;

pub trait SimulationEngine<T> {
    fn run(&mut self) -> Vec<T>;

    fn with_n_simulations(&mut self, n: usize);

    fn n_simulations(&self) -> usize;

    fn with_progress_bar(&mut self, pb: ProgressBar);

    fn with_default_progress_bar(&mut self) {
        let pb = ProgressBar::new(self.n_simulations() as u64);
        self.with_progress_bar(pb);
    }

    fn without_progress_bar(&mut self);
}

pub struct SerialSimulationEngine<T, F: Fn() -> T> {
    n: usize,
    simulation: F,
    pb: Option<ProgressBar>,
}

impl<T, F: Fn() -> T> SerialSimulationEngine<T, F> {
    pub fn new(n: usize, simulation: F) -> SerialSimulationEngine<T, F> {
        SerialSimulationEngine {
            n,
            simulation,
            pb: None,
        }
    }
}

impl<T, F: Fn() -> T> SimulationEngine<T> for SerialSimulationEngine<T, F> {
    fn run(&mut self) -> Vec<T> {
        let iter = 0..self.n;
        if let Some(pb) = self.pb.take() {
            iter.progress_with(pb)
                .map(|_| (self.simulation)())
                .collect()
        } else {
            iter.map(|_| (self.simulation)()).collect()
        }
    }

    fn with_n_simulations(&mut self, n: usize) {
        self.n = n
    }

    fn n_simulations(&self) -> usize {
        self.n
    }

    fn with_progress_bar(&mut self, pb: ProgressBar) {
        self.pb = Some(pb)
    }

    fn without_progress_bar(&mut self) {
        self.pb = None
    }
}

pub struct ParSimulationEngine<T: std::marker::Send, F: Fn() -> T + std::marker::Sync> {
    n: usize,
    simulation: F,
    pb: Option<ProgressBar>,
}

impl<T: std::marker::Send, F: Fn() -> T + std::marker::Sync> ParSimulationEngine<T, F> {
    pub fn new(n: usize, simulation: F) -> ParSimulationEngine<T, F> {
        ParSimulationEngine {
            n,
            simulation,
            pb: None,
        }
    }
}

impl<T: std::marker::Send, F: Fn() -> T + std::marker::Sync> SimulationEngine<T>
    for ParSimulationEngine<T, F>
{
    fn run(&mut self) -> Vec<T> {
        let iter = (0..self.n).into_par_iter();
        if let Some(pb) = self.pb.take() {
            iter.progress_with(pb)
                .map(|_| (self.simulation)())
                .collect()
        } else {
            iter.map(|_| (self.simulation)()).collect()
        }
    }

    fn with_n_simulations(&mut self, n: usize) {
        self.n = n
    }

    fn n_simulations(&self) -> usize {
        self.n
    }

    fn with_progress_bar(&mut self, pb: ProgressBar) {
        self.pb = Some(pb)
    }

    fn without_progress_bar(&mut self) {
        self.pb = None
    }
}
