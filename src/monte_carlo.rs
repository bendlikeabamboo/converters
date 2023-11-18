pub mod pi {
    use crate::gui_own::*;
    use rand::Rng;
    struct Coordinate {
        x: f32,
        y: f32,
    }

    fn simulate(iterations: u32) -> f32 {
        let mut results: Vec<bool> = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 1..iterations {
            let point = Coordinate {
                x: rng.gen(),
                y: rng.gen(),
            };
            let radius: f32 = (point.x.powi(2) + point.y.powi(2)).sqrt();
            results.push(if radius <= 1.0 { true } else { false })
        }
        let points_within_radius = results.iter().filter(|x| **x).count();
        let all_points = results.len();
        let estimated_pi = (points_within_radius as f32 / all_points as f32) * 4.0;

        println!("Points within radius: {points_within_radius}");
        println!("All points: {all_points}");
        println!("Result {estimated_pi}");
        estimated_pi
    }

    fn simulate_memory_optimized(iterations: u32) -> f32 {
        let mut points_within_radius: i32 = 0;
        let mut all_points: i32 = 0;
        let mut rng = rand::thread_rng();

        for _ in 1..iterations {
            let point = Coordinate {
                x: rng.gen(),
                y: rng.gen(),
            };
            let radius: f32 = (point.x.powi(2) + point.y.powi(2)).sqrt();
            if radius <= 1.0 {
                points_within_radius += 1;
            }
            all_points += 1;
        }
        let estimated_pi = (points_within_radius as f32 / all_points as f32) * 4.0;
        println!("Points within radius: {points_within_radius}");
        println!("All points: {all_points}");
        println!("Result {estimated_pi}");
        estimated_pi
    }

    pub fn estimate_pi() -> f32 {
        let title: &str = "Pi Estimator";
        print_converter_title(title);

        let user_input: u32 = read_int_input();
        let result = simulate(user_input);
        result
    }

    pub fn estimate_pi_optimized() -> f32 {
        let title: &str = "Pi Estimator (Optimized)";
        print_converter_title(title);

        let user_input: u32 = read_int_input();
        let result = simulate_memory_optimized(user_input);
        result
    }

}