fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    match six {
        Some(i) => println!("{i}"),
        _ => (),
    }

    if let None = none {
        println!("It is None");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}