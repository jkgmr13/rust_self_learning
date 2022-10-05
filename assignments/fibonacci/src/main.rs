use std::io;

fn main() {
    loop { 
        let mut input = String::new();

        println!("Enter the index of the fibonacci number you want or ctl + c to exit");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let index = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => 0-1,
        };
        let result = fibonacci(index);
        match result{
        0.. => println!("The fibonacci of index {index} is {result}"),
        _ => println!("Enter a valid Index"),
        }
    }
}

fn fibonacci(num: i32) -> i32 {
    if num == 0 { 
        0 
    } else if num == 1 { 
        1 
    } else if num > 1 { 
        fibonacci(num-1) + fibonacci(num-2) 
    } else {
        0-1
    }

}
