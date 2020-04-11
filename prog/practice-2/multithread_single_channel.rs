/*
연습문제
    1. 3개의 스레드를 만들어서 각각 mpsc의 tx를 복제하여 소유하고, 그 tx를 통하여 random i32 수를 송신하고,
        메인 스레드에서 수신하여 출력하는 프로그램을 작성해 보세요.
*/

use std::thread;
use std::sync::mpsc;
use rand::Rng;

fn main() {
    let (sender, receiver) = mpsc::channel();
    let mut handles = vec![];
    for i in 1..10 {
        let sender = mpsc::Sender::clone(&sender);
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let number: i32 = rng.gen_range(i32::min_value(), i32::max_value());
            sender.send(String::from("thread-") + &i.to_string() + ": " + &number.to_string());
        });
        handles.push(handle);
    }
    drop(sender);
    for h in handles {
        h.join().unwrap();
    }
    for message in receiver {
        println!("{:?}", message);
    }
}

