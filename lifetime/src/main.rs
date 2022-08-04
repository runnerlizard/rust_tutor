fn main() {
    // let mut s = String::from("hello");
    //
    // s.push_str(", world!");
    //
    // println!("{}", s);
    //
    // let len = calculate_length(&s);
    //
    // println!("{len}");
    let s = String::from("one two free");
    println!("first {}", first_word(&s));
    println!("second {}", second_word(&s));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '  {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut start = 0;
    let mut end = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if start == 0 {
                start = i;
            } else {
                if start != 0 {
                    end = i;
                    return &s[start..end];
                }
            }
        }
    }

    if end == 0 {
        return "";
    }

    &s[..]
}

fn calculate_length(s: &str) -> usize {
    s.len()
}