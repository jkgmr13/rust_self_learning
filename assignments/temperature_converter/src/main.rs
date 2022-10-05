use std::io;

fn main() {
    loop { 
        let mut input = String::new();

        println!("Enter 1 to convert from celsius to farenheit\nEnter 2 to convert from farenheit to celsius\nEnter 9 to quit");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let selection: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 99999,
        };

        match selection{
        2 => println!("{}", convert_farenheit_to_celsius()),
        1 => println!("{}", convert_celsius_to_farenheit()),
        9 => break,
        _ => println!("Enter 1 2 or 9")
        }
    }
}

fn convert_farenheit_to_celsius() -> f32 {
    let mut input = String::new();

    println!("Enter the temperature to convert");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let temp: f32 = input.trim().parse::<f32>().unwrap();
    (temp - 32.) * 5. / 9.
}

fn convert_celsius_to_farenheit() -> f32 {
    let mut input = String::new();

    println!("Enter the temperature to convert");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let temp: f32 = input.trim().parse::<f32>().unwrap();
    temp * 9. / 5. + 32.
}
