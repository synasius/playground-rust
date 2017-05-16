use std::io;


fn main() {
    println!("Are you converting from fahrenheit or celsius? [f/c]");

    let mut from = String::new();

    io::stdin().read_line(&mut from)
        .expect("Failed to read");

    let from = from.trim();

    println!("Please enter the value to convert: ");

    let mut value = String::new();

    // ask the user to enter the value to be converted
    loop {
        io::stdin().read_line(&mut value)
            .expect("Failed to read");

        let value: f64 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid value!");
                continue;
            }
        };


        // and now call the proper function that converts our value
        let result = if from == "c" {
            println!("celsius to fahrenheit");
            celsius_to_fahrenheit(value)
        } else {
            println!("fahrenheit to celsius");
            fahrenheit_to_celsius(value)
        };

        println!("Result is: {}", result);
        break;
    }
}

fn celsius_to_fahrenheit(value: f64) -> f64 {
    value * 1.8 + 32.0
}

fn fahrenheit_to_celsius(value: f64) -> f64 {
    (value - 32.0) / 1.8
}
