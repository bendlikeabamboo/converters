mod monte_carlo;
use monte_carlo::pi;

mod gui_own;
use gui_own::*;

mod converters;
use converters::temperature::*;

fn main() {
    loop {
        println!("Welcome to Rust Experiments");
        println!("====================");
        println!("[1] Convert from Farenheit to Celsius");
        println!("[2] Convert from Celsius to Farenheit");
        println!("[3] Estimate Pi Using Monte Carlo");
        println!("[4] Estimate Pi Using Monte Carlo (optimized)");
        println!("[100] Exit");
        println!("====================");
        println!("Please choose conversion:");

        // main owns the memory of the input
        match read_int_input() {
            1 => convert_farenheit_to_celsius(),
            2 => convert_celsius_to_farenheit(),
            3 => estimate_pi(),
            4 => estimate_pi_optimized(),
            100 => {
                println!("Goodbye!");
                break;
            }
            _ => continue,
        };
    }
}

fn estimate_pi() -> f32 {
    let title: &str = "Pi Estimator";
    print_converter_title(title);

    let user_input: u32 = read_int_input();
    let result = pi::simulate(user_input);
    result
}

fn estimate_pi_optimized() -> f32 {
    let title: &str = "Pi Estimator (Optimized)";
    print_converter_title(title);

    let user_input: u32 = read_int_input();
    let result = pi::simulate_memory_optimized(user_input);
    result
}
