use rand::Rng;
use simulator::{ParSimulationEngine, SerialSimulationEngine, SimulationEngine};
use std::time::Instant;

fn main() {
    // Demo usage
    {
        let mut eng = SerialSimulationEngine::new(1000, || {
            let mut rng = rand::thread_rng();
            if rng.gen::<f64>() > 0.5 {
                1
            } else {
                0
            }
        });
        eng.with_default_progress_bar();
        let start = Instant::now();
        eng.run();
        println!("serial with progress bar {:?}", start.elapsed());
    }
    {
        let mut eng = SerialSimulationEngine::new(1000, || {
            let mut rng = rand::thread_rng();
            if rng.gen::<f64>() > 0.5 {
                1
            } else {
                0
            }
        });
        eng.without_progress_bar();
        let start = Instant::now();
        eng.run();
        println!("serial without progress bar {:?}", start.elapsed());
    }
    {
        let mut eng = ParSimulationEngine::new(1000, || {
            let mut rng = rand::thread_rng();
            if rng.gen::<f64>() > 0.5 {
                1
            } else {
                0
            }
        });
        eng.with_default_progress_bar();
        let start = Instant::now();
        eng.run();
        println!("parallel with progress bar {:?}", start.elapsed());
    }
    {
        let mut eng = ParSimulationEngine::new(1000, || {
            let mut rng = rand::thread_rng();
            if rng.gen::<f64>() > 0.5 {
                1
            } else {
                0
            }
        });
        eng.without_progress_bar();
        let start = Instant::now();
        eng.run();
        println!("parallel without progress bar {:?}", start.elapsed());
    }
}
