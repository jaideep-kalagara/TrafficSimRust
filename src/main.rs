// Modules that contain logic for road, car, collision checks, config, and logging
mod road;
mod car;
mod sim_checks;
mod config;
mod logger;

// Standard library imports
use std::collections::HashSet;
use std::io::{stdin, stdout, Write};
use std::process::exit;

// Third-party crates for logging and randomization
use colored::Colorize;
use config::Config;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::{rng, Rng};

// Specific imports from local modules
use crate::road::{Road, RoadType};
use crate::sim_checks::collision_check;
use crate::car::Car;
use crate::config::load_config;
use crate::logger::*;

// Helper to print a line break
fn line_break() {
    println!("------------------------");
}

fn main() {
    // Initialize the RNG
    let mut rng: ThreadRng = rng();

    // Define the layout of the road (Input -> Signal Intersection -> Output)
    let test_road: Vec<Road> = vec![
        Road::new(RoadType::Input),
        Road::new(RoadType::SchoolZone),
        Road::new(RoadType::PedestrianCrossing),
        Road::new(RoadType::SignalIntersection),
        Road::new(RoadType::Roundabout),
        Road::new(RoadType::MergeLane),
        Road::new(RoadType::HighwaySegment),
        Road::new(RoadType::HighwaySegment),
        Road::new(RoadType::ConstructionZone),
        Road::new(RoadType::TollBooth),
        Road::new(RoadType::ExitRamp),
        Road::new(RoadType::NoSignalIntersection),
        Road::new(RoadType::Output),
    ];

    // Load configuration from config.toml or elsewhere
    let conf: Config = load_config();

    // Validate rule breaker percentage
    if conf.simulation.rule_breakers > 100 {
        panic!(
            "Invalid rule_breaker value: {} — must be between 0 and 100",
            conf.simulation.rule_breakers
        );
    }

    // get debug flag
    let debug: bool = conf.simulation.debug;

    // Calculate the number of rule breaking and lawful cars
    let rule_breaking_cars: u32 = ((conf.simulation.cars as f32)
        * (conf.simulation.rule_breakers as f32)
        / 100.0)
        .round() as u32;

    let lawful_cars: u32 = conf.simulation.cars - rule_breaking_cars;

    // Convert top speed from mph to road segments (1 segment = 10 mph)
    let top_speed: u32 = (conf.simulation.top_speed as f32 / 10.0).round() as u32;

    // Ensure the road is long enough to support the speed
    if top_speed as usize >= test_road.len() {
        log_error("Not enough road!");
        exit(-1);
    }

    // Display simulation info
    println!("{}", "Car Simulator - Made By Jaideep Kalagara".red());
    line_break();
    println!("Total # of cars: {}", conf.simulation.cars);
    println!("Percentage of rule breakers: {}%", conf.simulation.rule_breakers);
    println!("Top speed (in segments): {}", top_speed);
    println!();
    println!("# of rule breaking cars: {}", rule_breaking_cars);
    println!("# of lawful cars: {}", lawful_cars);
    line_break();

    // Wait for user confirmation
    print!("If this information looks correct, press Enter to continue: ");
    let _ = stdout().flush();
    let _ = stdin()
        .read_line(&mut String::new())
        .expect("An error occurred while waiting for input!");

    println!();
    println!("Running simulation...");

    // Initialize car vector and car ID counter
    let mut cars: Vec<Car> = vec![];
    let mut next_id: usize = 0;

    // Generate all cars with random speed and assign unique IDs
    if conf.simulation.base_point {
        // Base point doesn't affect generation logic here — both branches are same
        cars.extend((0..lawful_cars).map(|_| {
            let car = Car::new(false, 0, rng.random_range(1..=top_speed) as usize, next_id);
            next_id += 1;
            car
        }));
        cars.extend((0..rule_breaking_cars).map(|_| {
            let car = Car::new(false, 0, rng.random_range(1..=top_speed) as usize, next_id);
            next_id += 1;
            car
        }));
    } else {
        cars.extend((0..lawful_cars).map(|_| {
            let car = Car::new(false, 0, rng.random_range(1..=top_speed) as usize, next_id);
            next_id += 1;
            car
        }));
        cars.extend((0..rule_breaking_cars).map(|_| {
            let car = Car::new(false, 0, rng.random_range(1..=top_speed) as usize, next_id);
            next_id += 1;
            car
        }));
    }

    // Shuffle cars to randomize their positions at start
    cars.shuffle(&mut rng);
    log_info("Created cars vector!");

    // Set to track already-compared car pairs (by ID)
    let mut checked_pairs: HashSet<(usize, usize)> = HashSet::new();

    // Simulation counters
    let mut tick = 0;
    let mut collisions = 0;
    let mut safe_passes = 0;

    // Start simulation loop
    loop {
        tick += 1;
        log_info(&format!("[Tick {}]", tick));

        // Loop over each road segment
        for r in 0..test_road.len() {
            // Get cars currently on that road segment
            let mut cars_on_road: Vec<&Car> = cars.iter()
                .filter(|c| c.current_road_index == r)
                .collect();

            // Sort cars by ID for consistent pairing
            cars_on_road.sort_by_key(|c| c.id);

            // Check adjacent cars for collisions
            for i in 0..cars_on_road.len().saturating_sub(1) {
                let car1 = cars_on_road[i];
                let car2 = cars_on_road[i + 1];

                // Normalize pair order (smallest ID first)
                let pair = if car1.id < car2.id {
                    (car1.id, car2.id)
                } else {
                    (car2.id, car1.id)
                };

                // Skip pair if already checked
                if checked_pairs.contains(&pair) {
                    continue;
                }

                let road = &test_road[r];

                // Run collision check
                if collision_check(car1, car2, road) {
                    if debug {
                        log_warning(&format!(
                            "Collision detected between car {} and car {} on road index {}",
                            car1.id, car2.id, r
                        ));
                    }
                    collisions += 1;
                } else {
                    safe_passes += 1;
                }

                // Mark this pair as checked
                checked_pairs.insert(pair);
            }
        }

        // Move each car forward by its speed
        for car in &mut cars {
            if car.current_road_index < test_road.len() {
                car.current_road_index += car.speed;
            }
        }

        // End simulation when all cars have exited the road
        if cars.iter().all(|c| c.current_road_index >= test_road.len()) {
            break;
        }
    }

    // Final stats output
    log_info("Simulation complete!");
    println!();
    line_break();
    log_info(format!("Safe Passes: {}", (safe_passes as f32 / 2.0).round() as u32).as_str());
    log_info(format!("Collisions: {}", collisions).as_str());
    line_break();
}
