

pub mod pi {
    use rand::Rng;
    struct CartesianCoordinate{
        x: f32,
        y: f32
    }

    pub fn estimate(n: u32) -> f32 {
        
        let mut results: Vec<bool> = Vec::new();
        for _ in 1..n {
            let mut rng = rand::thread_rng();
            let x: f32 = rng.gen();
            let y: f32 = rng.gen();
            
            let point = CartesianCoordinate{x,y};
            let radius: f32 = (point.x.powi(2) + point.y.powi(2)).sqrt();
            let result = if radius < 1.0 { true } else { false };
            results.push(result)
        }
        let points_within_radius = results.iter().filter(|x| **x).count();
        println!("Points within radius: {points_within_radius}");
        let all_points = results.len();
        println!("All points: {all_points}");

        let estimated_pi = (points_within_radius as f32/all_points as f32)*4.0;
        println!("Result {estimated_pi}");
        estimated_pi
    }
    
    pub fn estimate_optimized(n: u32) -> f32 {
        
        let mut points_within_radius = 0;
        let mut all_points = 0;
        let mut rng = rand::thread_rng();
        for _ in 1..n {
            let x: f32 = rng.gen();
            let y: f32 = rng.gen();
            
            let point = CartesianCoordinate{x,y};
            let radius: f32 = (point.x.powi(2) + point.y.powi(2)).sqrt();
            if radius < 1.0 {
                points_within_radius += 1;
            }
            all_points += 1;
        }
        let estimated_pi = (points_within_radius as f32/all_points as f32)*4.0;
        println!("Result {estimated_pi}");
        estimated_pi
    }
}