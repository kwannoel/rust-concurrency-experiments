use std::{thread, time};


pub fn task() {
    let five = time::Duration::from_millis(5000);
    thread::sleep(five);
    println!("hi");
}
