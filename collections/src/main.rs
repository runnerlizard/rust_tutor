use std::collections::HashMap;

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];

    v1.push(8);
    v2.push(5);

    let third: &i32 = &v2[2];
    println!("Third element is {}", third);

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("Third is {}", third),
        None => println!("There is nothing"),
    }

    for i in &v1 {
        println!("{i}");
    }


    let hello = String::from("السلام عليكم");
    println!("{hello}");
    for c in hello.chars() {
        println!("{c}");
    }
    for b in hello.bytes() {
        println!("{b}");
    }
    println!("\n\n");

    let hello = String::from("Dobrý den");
    println!("{hello}");
    for c in hello.chars() {
        println!("{c}");
    }
    for b in hello.bytes() {
        println!("{b}");
    }
    println!("\n\n");

    let hello = String::from("Hello");
    println!("{hello}");
    for c in hello.chars() {
        println!("{c}");
    }
    for b in hello.bytes() {
        println!("{b}");
    }
    println!("\n\n");

    let hello = String::from("שָׁלוֹם");
    println!("{hello}");
    for c in hello.chars() {
        println!("{c}");
    }
    for b in hello.bytes() {
        println!("{b}");
    }
    println!("\n\n");

    let hello = String::from("नमस्ते");
    println!("{hello}");
    for c in hello.chars() {
        println!("{c}");
    }
    for b in hello.bytes() {
        println!("{b}");
    }
    println!("\n\n");

    let hello = String::from("こんにちは");
    println!("{hello}");
    for c in hello.chars() {
        println!("{c}");
    }
    for b in hello.bytes() {
        println!("{b}");
    }
    println!("\n\n");

    let hello = String::from("안녕하세요");
    println!("{hello}");
    for c in hello.chars() {
        println!("{c}");
    }
    for b in hello.bytes() {
        println!("{b}");
    }
    println!("\n\n");

    let hello = String::from("你好");
    println!("{hello}");
    for c in hello.chars() {
        println!("{c}");
    }
    for b in hello.bytes() {
        println!("{b}");
    }
    println!("\n\n");

    let hello = String::from("Olá");
    println!("{hello}");
    for c in hello.chars() {
        println!("{c}");
    }
    for b in hello.bytes() {
        println!("{b}");
    }
    println!("\n\n");

    let hello = String::from("Здравствуйте");
    println!("{hello}");
    for c in hello.chars() {
        println!("{c}");
    }
    for b in hello.bytes() {
        println!("{b}");
    }
    println!("\n\n");

    let hello = String::from("Hola");
    println!("{hello}");
    for c in hello.chars() {
        println!("{c}");
    }
    for b in hello.bytes() {
        println!("{b}");
    }
    println!("\n\n");



    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 20);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let score = scores.get("blue").copied().unwrap_or(0);

    println!("{score}");

    task1();
    task2("pig latin implementation");
}

fn task1() {
    let e = vec![1, 8, 9, 0, 6, 7, 8, 9, 9, 5];

    println!("{}", get_medium(&e));
    println!("{}", get_median(&e));
    println!("{}", get_mode_of_list(&e));
}

fn get_medium(e: &Vec<i32>) -> i32 {
    let mut medium = 0;
    let mut count = 0;

    for el in e {
        medium += el;
        count += 1;
    }

    medium / count
}

fn get_median(e: &Vec<i32>) -> i32 {
    let mut more;
    let mut less;
    let mut eq;
    let mut ab :i32;

    for el in e {
        eq = 0;
        more = 0;
        less = 0;
        for elem in e {
            if el > elem {
                more += 1;
            } else if el < elem {
                less += 1;
            } else {
                eq += 1;
            }
        }
        ab = more - less;
        if more + less % 2 == 0 {
            if more == less {
                return *el;
            }
        } else {
            if ab.abs() == 1 {
                return *el;
            }
        }
        if eq >= ab.abs() {
            return *el;
        }
    }
    0
}

fn get_mode_of_list(e: &Vec<i32>) -> i32 {
    let mut h = HashMap::new();

    for el in e {
        let count = h.entry(el).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    let mut mode :i32 = 0;
    for (key, value) in h {
        if max < value {
            max = value;
            mode = *key;
        }
    }

    mode
}

fn task2(s: &str) {
    let l = s.split_whitespace();
    let mut ret :String = "".to_string();
    let vowels = "bcdfghkjlmnpqrstwxz";
    let consonants = "aeiouy";
    for e in l {
        if vowels.contains(e.to_lowercase().chars().next().unwrap()) {
            ret = ret + &e[1..] + "-" + &e[..1] + "ay ";
        } else if consonants.contains(e.to_lowercase().chars().next().unwrap()) {
            ret = ret + e + "-hay ";
        } else {
            ret = ret + e;
        }
    }

    println!("{ret}");
}
