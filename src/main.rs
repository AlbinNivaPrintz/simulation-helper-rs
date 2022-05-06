use rand::Rng;
use simulator::{SerialSimulationEngine, SimulationEngine};

fn main() {
    let eng = SerialSimulationEngine::new(10, || {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() > 0.5 {
            1
        } else {
            0
        }
    });
    let res = eng.run();
    println!("{:?}", res);
}
