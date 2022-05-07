use rand::Rng;
use simulator::{ParSimulationEngine, SerialSimulationEngine, SimulationEngine};
use std::time::Instant;

fn main() {
    // Demo usage
    {
        // SerialSimulationEngine runs simulations serially
        let start = Instant::now();
        let mut eng = SerialSimulationEngine::new(1000, || {
            let mut rng = rand::thread_rng();
            if rng.gen::<f64>() > 0.5 {
                1
            } else {
                0
            }
        });
        eng.with_default_progress_bar();
        let res = eng.run();
        println!("{:?} {:?}", start.elapsed(), res);
    }
    {
        // ParSimulationEngine runs simulations in parallel using rayon
        let start = Instant::now();
        let mut eng = ParSimulationEngine::new(1000, || {
            let mut rng = rand::thread_rng();
            if rng.gen::<f64>() > 0.5 {
                1
            } else {
                0
            }
        });
        eng.with_default_progress_bar();
        let res = eng.run();
        println!("{:?} {:?}", start.elapsed(), res);
    }
}
