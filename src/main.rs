use std::io;
use std::io::Write;

fn main() {
    loop {
        println!("Welcome to Converter");
        println!("====================");
        println!("[1] Convert from Farenheit to Celsius");
        println!("[2] Convert from Celsius to Farenheit");
        println!("[100] Exit");
        println!("====================");
        println!("Please choose conversion:");

        // main owns the memory of the input
        match read_integer_input() {
            1 => convert_farenheit_to_celsius(),
            2 => convert_celsius_to_farenheit(),
            100 => {
                println!("Goodbye!");
                break;
            },
            _ => continue,
        };
    }
}

fn read_integer_input() -> u32 {
    // this function should loop until we get valid input
    loop {
        // declare new string variable
        let mut user_input: String = String::new();

        print!("Enter input: ");
        io::stdout().flush().unwrap();

        // obtain user input
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error getting input");

        println!("You have selected \"{}\"", user_input.trim());

        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return user_input;
    }
}

fn read_float_input() -> f32 {
    loop {
        // declare new string variable
        let mut user_input: String = String::new();

        print!("Enter input: ");
        io::stdout().flush().unwrap();

        // obtain user input
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error getting input");

        println!("You have selected \"{}\"", user_input.trim());

        let user_input: f32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return user_input;
    }
}

fn print_converter_title(title: &str) {
    println!();
    println!("=================================");
    println!("{}", title);
    println!("=================================");
    println!();
}

fn print_converter_results(results: &String, units: &str) {
    println!("Result: {results}{units}");
    println!("=================================");
    println!();
    println!();
}

fn convert_farenheit_to_celsius() -> f32 {
    let title: &str = "Farenheit To Celsius Converter";
    print_converter_title(title);
    
    let user_input: f32 = read_float_input();
    let results: f32 = (user_input - 32.0) * (5.0 / 9.0);
    {
        let results: String = format!("{:.2}", &results);
        let units: &str = "\u{00B0}C";
        print_converter_results(&results, &units);
    }

    results
}

fn convert_celsius_to_farenheit() -> f32 {
    let title: &str = "Celsius To Farenheit Converter";
    print_converter_title(title);

    let user_input: f32 = read_float_input();
    let results: f32 = (user_input / (5.0 / 9.0)) + 32.0;
    {
        let results: String = format!("{:.2}", &results);
        let units: &str = "\u{00B0}F";
        print_converter_results(&results, &units);
    }
    results
}