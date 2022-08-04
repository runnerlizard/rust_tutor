use std::io;

fn main() {
    println!("Input temperature in Fahrenheit");

    let mut temp = String::new();


    io::stdin()
        .read_line(&mut temp)
        .expect("Input error");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => 8888.0,
    };

    println!("Temperature in Farenheit {temp}");

    let temp = convert_fahrenheit_to_celsius(temp);

    println!("Temperature in Celsius {temp}");
}

fn convert_fahrenheit_to_celsius(t: f64) -> f64 {
    (t - 32.0) * 5.0 / 9.0
}
