use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator};
use rayon::prelude::*;

pub struct SimulationEngine<T: std::marker::Send, F: Fn() -> T + std::marker::Sync> {
    /// The number of simulations to run
    n: usize,

    /// The simulation closure
    simulation: F,

    /// An optional indicatif::ProgressBar
    pb: Option<ProgressBar>,

    in_parallel: bool,
}

impl<T: std::marker::Send, F: Fn() -> T + std::marker::Sync> SimulationEngine<T, F> {
    pub fn new(n: usize, simulation: F) -> SimulationEngine<T, F> {
        SimulationEngine {
            n,
            simulation,
            pb: None,
            in_parallel: true,
        }
    }

    fn _run_serial(&mut self) -> Vec<T> {
        let iter = 0..self.n;
        if let Some(pb) = self.pb.take() {
            iter.progress_with(pb)
                .map(|_| (self.simulation)())
                .collect()
        } else {
            iter.map(|_| (self.simulation)()).collect()
        }
    }

    fn _run_parallel(&mut self) -> Vec<T> {
        let iter = (0..self.n).into_par_iter();
        if let Some(pb) = self.pb.take() {
            iter.progress_with(pb)
                .map(|_| (self.simulation)())
                .collect()
        } else {
            iter.map(|_| (self.simulation)()).collect()
        }
    }

    pub fn run(&mut self) -> Vec<T> {
        if self.in_parallel {
            self._run_parallel()
        } else {
            self._run_serial()
        }
    }

    pub fn with_n_simulations(&mut self, n: usize) {
        self.n = n
    }

    pub fn n_simulations(&self) -> usize {
        self.n
    }

    pub fn with_progress_bar(&mut self, pb: ProgressBar) {
        self.pb = Some(pb)
    }

    pub fn with_default_progress_bar(&mut self) {
        let pb = ProgressBar::new(self.n as u64);
        self.with_progress_bar(pb);
    }

    pub fn without_progress_bar(&mut self) {
        self.pb = None
    }

    pub fn with_no_parallel(&mut self) {
        self.in_parallel = false
    }

    pub fn with_parallel(&mut self) {
        self.in_parallel = true
    }
}
