use std::io;
use std::io::Write;

pub fn read_float_input() -> f32 {
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

pub fn read_int_input() -> u32 {
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

pub fn print_converter_title(title: &str) {
    println!();
    println!("=================================");
    println!("{}", title);
    println!("=================================");
    println!();
}

pub fn print_converter_results(results: &String, units: &str) {
    println!("Result: {results}{units}");
    println!("=================================");
    println!();
    println!();
}
