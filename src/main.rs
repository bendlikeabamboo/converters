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
        println!("[5] Read a CSV in a data");
        println!("[100] Exit");
        println!("====================");
        println!("Please choose conversion:");

        // main owns the memory of the input
        match read_int_input() {
            1 => convert_farenheit_to_celsius(),
            2 => convert_celsius_to_farenheit(),
            3 => pi::estimate_pi(),
            4 => pi::estimate_pi_optimized(),
            100 => {
                println!("Goodbye!");
                break;
            }
            _ => continue,
        };
    }
}

