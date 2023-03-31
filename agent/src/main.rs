use std::{thread::sleep, time::Duration};

fn main() {
    loop {
        sleep(Duration::new(1, 0));
        println!("test");
    }
}
