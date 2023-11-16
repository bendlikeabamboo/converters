pub mod temperature {
    use crate::gui_own::*;
    pub fn convert_farenheit_to_celsius() -> f32 {
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

    pub fn convert_celsius_to_farenheit() -> f32 {
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
}
