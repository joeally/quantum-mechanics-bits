extern crate rand;
use std::f64;
use rand::Rng;
use rand::distributions::Uniform;

fn opposite_direction(direction: f64) -> f64 {
    let opposite = direction + f64::consts::PI;
    let full_circle = 2.0 * f64::consts::PI;
    if opposite > full_circle {
        return opposite - full_circle;
    } else {
        return opposite;
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum QubitReading {
    Up,
    Down
}

pub struct Qubit {
    direction: f64,
}

pub struct Apparatus {
    direction: f64
}

pub fn measure_qubit<R>(apparatus: &Apparatus, 
                               qubit: &mut Qubit, 
                                rng_iter: &mut R) -> QubitReading 
where 
    R: Iterator<Item=f64>
{
    let theta = (apparatus.direction - qubit.direction).abs();
    let avg_val = theta.cos();
    let p = (avg_val + 1.0)/2.0;
    let r: f64 = rng_iter.next().expect("This should never happen");
    if r < p {
        qubit.direction = apparatus.direction;
        return QubitReading::Up;
    } else {
        qubit.direction = opposite_direction(apparatus.direction);
        return QubitReading::Down;
    }
}

pub fn perform_experiment() {
    let apparatus = Apparatus { direction: 0.0 };
    let mut qubit = Qubit { direction: f64::consts::PI / 2.0 };
    let mut rng =  rand::thread_rng();
    let distribution = Uniform::new(0.0, 1.0);
    let mut rng_iter = rng.sample_iter(&distribution);
    let measurement = measure_qubit(&apparatus, &mut qubit, &mut rng_iter);
    let measurement_str = match measurement {
        QubitReading::Up => "Up",
        QubitReading::Down => "Down"
    };
    println!("{} measured", measurement_str);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate rand;
    fn assert_experiment<RngIter>(apparatus_dir: f64, 
                          qubit_dir: f64,
                          expected_reading: QubitReading,
                          rng_iter: &mut RngIter)
    where 
        RngIter: Iterator<Item=f64>
    {

        let apparatus = Apparatus { direction: apparatus_dir };
        let mut qubit = Qubit { direction: qubit_dir };
        let reading = measure_qubit(&apparatus, &mut qubit, rng_iter);
        assert!(reading == expected_reading);
    }

    fn assert_experiments(apparatus_dir: f64, 
                          qubit_dir: f64,
                          expected_reading: QubitReading){
        let mut rng_iter =  (0..10).map(|i: i32| -> f64 { i as f64/ 10.0 });
        for _ in 1..10 {
            assert_experiment(apparatus_dir, qubit_dir, expected_reading, &mut rng_iter);
        }
    }

    #[test]
    fn measure_qubit_opposite_always_down() {
        assert_experiments(0.0, f64::consts::PI, QubitReading::Down);
        assert_experiments(f64::consts::PI, 0.0, QubitReading::Down);
    }

    #[test]
    fn measure_qubit_same_always_up() {
        assert_experiments(0.0, 0.0, QubitReading::Up);
        assert_experiments(f64::consts::PI, f64::consts::PI, QubitReading::Up);
    }
}
