use std::io;
fn main() {
    let mut choose_temperature = false;

    let mut option_number: i8 = 1;

    while !choose_temperature {
        let mut choice = String::new();

        println!("Enter 1 or 2 depending of your choice:");
        println!("1 -> convertion from Fahrenheit to Celsius");
        println!("2 -> convertion from Celsius to Fahrenheit");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        option_number = match choice.trim().parse() {
            Ok(number) => {
                if number != 1 && number != 2 {
                    print!("Please choose either 1 or 2\n");
                    continue;
                } else {
                    choose_temperature = true;
                    number
                }
            }
            Err(_) => {
                print!("Please choose either 1 or 2\n");
                continue;
            }
        };
    }

    let temperature_selected;
    let temperature_desired;

    match option_number {
        1 => {
            temperature_selected = "Fahrenheit";
            temperature_desired = "Celsius";
        }
        _ => {
            temperature_selected = "Celsius";
            temperature_desired = "Fahrenheit";
        }
    };

    println!("You chose convertion from {temperature_selected} to {temperature_desired}");

    loop {
        // Get the temperature value
        println!("Temperature in {temperature_selected}:");
        let mut temperature_selected_value = String::new();
        io::stdin()
            .read_line(&mut temperature_selected_value)
            .expect("Failed to read line");
        let temperature_selected_value: f64 = match temperature_selected_value.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Enter a number");
                continue;
            }
        };
        // Make the conversion
        let temperature_desired_value: f32 = match option_number {
            1 => ((temperature_selected_value - 32.0) * (5.0 / 9.0)) as f32,
            _ => ((temperature_selected_value * (9.0 / 5.0)) + 32.0) as f32,
        };
        // Print the result
        if option_number == 1 {
            print!("{temperature_selected_value}℉ = {temperature_desired_value}℃\n");
        } else {
            print!("{temperature_selected_value}℉ = {temperature_desired_value}℃\n");
        }
        break;
    }
}
