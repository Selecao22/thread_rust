extern crate rand;

use std::thread;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

const MAX_TIME: u64 = 6;
const MAX_VALUE: u64 = 12;

fn main() {

    let mut handles = vec![];
    let start_time = Instant::now();
    let mx = Arc::new(Mutex::new(0));

    for i in 0..5
    {
        let mux = Arc::clone(&mx);
        let time = rand::random::<u64>() % MAX_TIME;
        let tt = i;
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(time));
            let mut num = mux.lock().unwrap();
            println!("Поток № {}", tt);
            println!("Спал {} с", time);
            *num = rand::random::<u64>() % MAX_VALUE;
            println!("Значение, которое утановил поток {}\n", *num);
            });

        handles.push(handle);

    }

//    for h in handles
//    {
//        h.join().unwrap();
//    }

    thread::sleep(Duration::from_secs(6));
    println!("Последнее значение в мьютексе {}", *mx.lock().unwrap());
    println!("Программа выполнилась за {} с", Instant::now().duration_since(start_time).as_secs());
}
