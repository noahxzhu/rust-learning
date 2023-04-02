use std::io;

fn main() {
    println!("Temperature Converter");
    println!("1: Celsius");
    println!("2: Fahrenheit");

    let selected: u32;

    loop {
        println!("Please select the temperature standard:");
        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };

        if option != 1 && option != 2 {
            println!("Please select the provided number");
            continue;
        } else {
            selected = option;
            break;
        }
    }

    let given_temperature: i32;

    loop {
        println!("Please input the temperature:");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: i32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };

        given_temperature = temperature;
        break;
    }

    if selected == 1 {
        let temperature = (given_temperature * 9 / 5) + 32;
        println!("Fahrenheit: {temperature}");
    } else {
        let temperature = (given_temperature - 32) * 5 / 9;
        println!("Celius: {temperature}");
    }
}
