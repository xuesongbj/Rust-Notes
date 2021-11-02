use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::thread::sleep;

struct JobStatus {
    jobs_completed: i32,
}

fn main() {
    // Arc::new : 多线程，保证数据原子性
    // Mutex::new : 互斥锁，保证多线程数据安全
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            sleep(Duration::from_millis(250));
            status_shared.lock().unwrap().jobs_completed += 1;
        }
    });

    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting...");
        sleep(Duration::from_millis(500));
    }
}