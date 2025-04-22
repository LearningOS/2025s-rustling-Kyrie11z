// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.



use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: Mutex<u32>, // 用Mutex包裹计数器
}

fn main() {
    let status = Arc::new(JobStatus {
        jobs_completed: Mutex::new(0), // 初始化Mutex
    });

    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 获取锁后修改值
            let mut completed = status_shared.jobs_completed.lock().unwrap();
            *completed += 1;
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    let completed = status.jobs_completed.lock().unwrap();
    println!("jobs completed {}", *completed);
}