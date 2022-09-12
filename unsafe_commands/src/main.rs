static mut COUNTER: i32 = 0;

fn add_to_count(inc: i32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe fn dangerous() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    // unsafe { dangerous(); }
    // unsafe {
    //     println!("Abs -3 = {}", abs(-3));
    // }
    add_to_count(5);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}