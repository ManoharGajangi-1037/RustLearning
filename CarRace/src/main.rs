use std::fmt::format;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use rand::Rng;

const TOTAL_CARS: usize = 5;
const FINISH_LINE: u32 = 100;

#[derive(Debug, Clone)]
struct Car {
    name: String,
    position: u32,
    finished_at: Option<Instant>,
}
fn main() {
    let mut handles = vec![];
    let cars = Arc::new(Mutex::new(vec![]));
    for i in 1..=TOTAL_CARS {
        let car = Car {
            name: format!("Car-{}", i),
            position: 0,
            finished_at: None,
        };
        cars.lock().unwrap().push(car);
    }

    for i in 0..TOTAL_CARS {
        let cars_clone = Arc::clone(&cars);
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            loop {
                thread::sleep(Duration::from_millis(rng.gen_range(100..300)));

                let mut cars = cars_clone.lock().unwrap();

                let car = &mut cars[i];

                if car.position >= FINISH_LINE {
                    if car.finished_at.is_none() {
                        car.finished_at = Some(Instant::now());
                    }
                    break;
                };

                let jump = rng.gen_range(1..15);
                car.position += jump;
                println!("{} moved to {}", car.name, car.position)
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let cars = cars.lock().unwrap();
    let mut sorted_cars = cars.clone();
    sorted_cars.sort_by_key(|c| c.finished_at);

    println!("Race finished!!!!");
    for (i, car) in sorted_cars.iter().enumerate() {
        println!("{}.{}-> finished at {:?}",i+1,car.name,car.finished_at.unwrap());
    }
}
