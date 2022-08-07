use std::io;

fn main() {
    println!("Input number of element:");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Error input");

    let num: u32 = match num.trim().parse() {
        Ok(n) =>  n,
        Err(_) => 8888
    };

    println!("Element in position {} is {}", num, fibonacci::fibonacci(num));
}

