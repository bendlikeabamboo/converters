pub mod pi {
    use rand::Rng;
    struct Coordinate {
        x: f32,
        y: f32,
    }

    pub fn simulate(iterations: u32) -> f32 {
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

    pub fn simulate_memory_optimized(iterations: u32) -> f32 {
        let mut points_within_radius = 0;
        let mut all_points = 0;
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
}
