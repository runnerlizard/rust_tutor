use std::thread;
use std::time::Duration;

fn count_in_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("{} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("here is a vector! {:?}", v);
    });

    handle.join().unwrap();
}
