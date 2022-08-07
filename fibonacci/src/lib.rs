pub fn fibonacci(num: u32) -> u32 {
    let mut answer = 1;
    let mut first = 1;
    let mut second = 1;

    if num < 1 {
        answer = 0;
    } else if num < 3 {
        answer = 1;
    } else {
        for mut _i in 2..num {
            answer = first + second;
            first = second;
            second = answer;
            _i += 1;
        }
    }

    answer
}