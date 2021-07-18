use std::sync::Arc;
use std::thread;

fn main(){

let five = Arc::new(5);

for _ in 0..10 {
    let five = five.clone();

    thread::spawn(move || {
        println!("{:?}", five);
    });
}
}
