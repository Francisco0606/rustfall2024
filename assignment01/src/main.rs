const FREEZING_F: f64 = 32.0; 

fn fahrenheit_to_celsius(f: f64) -> f64 {
    //Converts Fahrenheit to Celsius
    return (f - FREEZING_F) * (5.0/9.0); 
}
fn celsius_to_fahrenheit(c: f64) -> f64 {
    //Converts Celsius to Fahrenheit 
    return (c * 9.0/5.0) + FREEZING_F
}

fn main() {
    let mut temp_f: f64 = 32.0;
    let temp_c: f64 = 16.0;
    println!("Celcius: {:.1} to Fahrenheit: {:.1}", temp_c, celsius_to_fahrenheit(temp_c));
    println!("Fahrenheit: {:.1} to Celcius: {:.1}", temp_f, fahrenheit_to_celsius(temp_f));
    //loop 5 times
    for _ in 0..5 {
        //add 1 and display
        temp_f += 1.0;
        println!("Fahrenheit: {:.1} to Celcius: {:.1}", temp_f, fahrenheit_to_celsius(temp_f));
    }
}

// Declare a constant for the freezing point of water in Fahrenheit (32°F).
// Implement two functions:
//  fahrenheit_to_celsius(f: f64) -> f64: Converts Fahrenheit to Celsius
//  celsius_to_fahrenheit(c: f64) -> f64: Converts Celsius to Fahrenheit
// In the main function:
//  Declare a mutable variable with a temperature in Fahrenheit
//  Convert it to Celsius using your function and print the result
//  Use a loop to convert and print the next 5 integer temperatures (e.g., if you start with 32°F, print conversions for 33°F, 34°F, 35°F, 36°F, and 37°F)
