use std::io;
use std::process;

fn convert_to_celcius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * (5.0/9.0)
}

fn main() {
    println!("Enter temperatature in Fahrenheit:");
    let fahrenheit: f32 = {
        let mut read_from_console = String::new();
        match io::stdin().read_line(&mut read_from_console) {
            Ok(_) => {
                read_from_console.trim().parse().unwrap()
            }
            Err(error) => {
                println!("error: {}", error);
                process::exit(1);
            }
        }
    };
    println!("{}° fahrenheit = {}° celcius", fahrenheit, convert_to_celcius(fahrenheit))
}