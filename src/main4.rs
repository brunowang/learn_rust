use std::fs::File;
use std::io::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    file_open();
    multi_threads_by_mutex();
    multi_threads_by_unsafe();
    multi_threads_by_atomic();
    multi_threads_by_channel();
}

fn file_open() {
    let mut f: File = File::open("src/test.txt").expect("file error");
    let mut buf: String = String::new();
    f.read_to_string(&mut buf).expect("read error");
    println!("{}", buf);
}

fn multi_threads_by_mutex() {
    let shared_num = Arc::new(Mutex::new(0));
    let mut thread_pool = Vec::new();
    for _ in 0..15 {
        let num_clone = shared_num.clone();
        thread_pool.push(thread::spawn(move || {
            let mut num = num_clone.lock().unwrap();
            *num += 1;
        }));
    }
    for t in thread_pool {
        t.join().unwrap();
    }
    println!(
        "finally shared_num value is: {:?}",
        shared_num.lock().unwrap()
    );
}

static mut N: i32 = 0;

fn multi_threads_by_unsafe() {
    let mut thread_pool = Vec::new();
    for _ in 0..15 {
        thread_pool.push(thread::spawn(|| unsafe {
            N += 1;
        }));
    }
    for t in thread_pool {
        t.join().unwrap();
    }
    unsafe {
        println!("finally N value is: {}", N);
    }
}

static NUM_ATOM: AtomicI32 = AtomicI32::new(0);

fn multi_threads_by_atomic() {
    let mut thread_pool = Vec::new();
    for _ in 0..15 {
        thread_pool.push(thread::spawn(|| {
            NUM_ATOM.fetch_add(1, Ordering::Relaxed);
        }));
    }
    for t in thread_pool {
        t.join().unwrap();
    }
    println!("finally NUM_ATOM value is: {:?}", NUM_ATOM);
}

fn curl(idx: i32, tx: mpsc::Sender<String>) {
    thread::sleep(Duration::from_millis(200));
    tx.send(format!("第{}个网页抓取完成", idx)).unwrap();
}

fn multi_threads_by_channel() {
    let (tx, rx) = mpsc::channel();
    for i in 0..5 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            curl(i, tx_clone);
        });
    }
    thread::spawn(move || tx.send(String::from("开始抓取")).unwrap());
    for ret in &rx {
        println!("{}", ret);
    }
}
