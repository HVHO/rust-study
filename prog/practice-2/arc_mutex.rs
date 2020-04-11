/*
연습문제
    2. Arc<Mutuex<SomeStruct>>, 여기서 SomeStruct { a: i32, b: i32} 이고,
        3개의 스레드를 만들어서 SomeStruct를 공유하고, 한 스레드는 1초 간격으로 1씩 증가 시키고, 2번 스레드는 2초 간격으로 1씩 감소시키고,
        3번 스레드는 3초 간격으로 2씩 곱하여서, 결과 값을 출력해 보세요. 여기서 각각의 증가/감소를 10회 하고 각각의 스레드는 종료합니다.
*/

use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct SomeStruct {
    a: i32,
    b: i32,
}

fn main() {
    let ss = Arc::new(Mutex::new(SomeStruct { a: 1, b: 10001 }));
    let mut handles = vec![];
    {
        let ss = Arc::clone(&ss);
        let handle = thread::spawn(move || {
            let mut ss = ss.lock().unwrap();
            for _ in 1..11 {
                (*ss).a += 1;
                (*ss).b += 1;
                thread::sleep(Duration::from_secs(1));
                println!("# thread-1: \n\t{:#?}\n", ss);
            }
        });
        handles.push(handle);
    }
    {
        let ss = Arc::clone(&ss);
        let handle = thread::spawn(move || {
            let mut ss = ss.lock().unwrap();
            for _ in 1..11 {
                (*ss).a -= 1;
                (*ss).b -= 1;
                thread::sleep(Duration::from_secs(2));
                println!("# thread-2: \n\t{:#?}\n", ss);
            }
        });
        handles.push(handle);
    }
    {
        let ss = Arc::clone(&ss);
        let handle = thread::spawn(move || {
            let mut ss = ss.lock().unwrap();
            for _ in 1..11 {
                (*ss).a *= 2;
                (*ss).b *= 2;
                thread::sleep(Duration::from_secs(3));
                println!("# thread-3: \n\t{:#?}\n", ss);
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("result: {:?}", ss.lock().unwrap());
}

