use std::io;


fn main() {
    let mut number: u32 = 3;
    let mut input = String::new();

    println!("Enter a number");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 3,
    };



    if number < 5 {
        println!("condition was true");
    } else {
        println!("Condition was false");
    }
}

