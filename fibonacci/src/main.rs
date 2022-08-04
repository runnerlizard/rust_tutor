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

    let mut answer = 1;
    let mut first = 1;
    let mut second = 1;

    if num < 1 {
        answer = 0;
    } else if num < 3 {
        answer = 1;
    } else {
        for mut i in 2..num {
            answer = first + second;
            first = second;
            second = answer;
            i += 1;
        }
    }

    println!("Element in position {num} is {answer}");
}
