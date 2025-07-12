// -------------------------------------------
// 			Threads Basics
// -------------------------------------------
use std::thread;
use std::time::Duration;

#[test]
fn main() {
    println!("This will be printed");
    println!("This will also be printed");
    println!("The concurrency will start after this line");

    let t  = thread::spawn(|| {
        println!("Hello 1 from the thread");
        println!("Hello 2 from the thread");
        println!("Hello 3 from the thread");
        println!("Hello 4 from the thread");
        println!("Hello 5 from the thread");
        println!("Hello 6 from the thread");
        println!("Hello 7 from the thread");
    });


    thread::sleep(Duration::from_millis(1));
    println!("Hello 1 from the main");
    println!("Hello 2 from the main");
    t.join();
}


// -------------------------------------------
// 	Ownership and Threads
//              - Prerequiste: Closures
// -------------------------------------------

#[test]
fn main_2() {
    let x = "some string".to_string();

    thread::spawn(move || {
        // let y = x;
        println!("{x}");
    });
    //println!("{x}");
}


// -------------------------------------------
// 	Message Passing Through Channels (Part 1)
// -------------------------------------------

// Example 1

use std::sync::mpsc;

#[test]
fn main_3() {
    let (tx, rx) = mpsc::channel();
    // let rx_clone = rx.clone();
    for i in 0..10 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            println!("Sending value {i}");
            tx_clone.send(i).unwrap();
        });
    }

    drop(tx);
    // let recv_val = rx.recv().unwrap();
    // println!("Recieved {recv_val}");
    // let recv_val = rx.recv().unwrap();
    // println!("Received {recv_val}");

    for message in rx {
        println!("Received {message}");
    }
}



// ------------------------------------------------
// 	Message Passing through Channels (Part 2)
// ------------------------------------------------

#[test]
fn main_4() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let x = "some_value".to_string();
        println!("Sending value {x}");
        // thread::sleep(Duration::from_secs(3));
        tx.send(x).unwrap();
    });

    // rx.recv().unwrap();
    // println!("I am blocked");

    let mut received_status = false;
    while received_status != true {
        match rx.try_recv() {
            Ok(received_value) => {
                println!("Received value is {:?}", received_value);
                received_status = true;
            }
            Err(_) => println!("I am doing some other stuff"),
        }
    }
}


// -------------------------------------------
// 			Sharing States (Part 1)
// -------------------------------------------

use std::{sync::Mutex};

#[test]
fn main_6() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 10;
    }

    let lock_m = m.lock().unwrap();
    println!("m is: {:?}", *lock_m);
    drop(lock_m);   // add later on

    let lock_m1 = m.lock().unwrap();
    println!("This code is blocked");
}


// -------------------------------------------
// 			Sharing States (Part 2)
// -------------------------------------------
use std::rc::Rc;
use std::sync::Arc;

struct File {
    text: Vec<String>,
}

#[test]
fn main_7() {
    let file = Arc::new(Mutex::new(File { text: vec![] }));
    let mut thread_vec = vec![];

    for i in 0..10 {
        let file = Arc::clone(&file);
        let handle = thread::spawn(move || {
            let mut file_lock = file.lock().unwrap();
            file_lock.text.push(format!("Hello from Thread {i}"));
        });
        thread_vec.push(handle);
    }
    for handle in thread_vec {
        handle.join().unwrap();
    }

    let file_lock = file.lock().unwrap();
    for t in &file_lock.text {
        println!("{t}");
    }
}


// -------------------------------------------
// 			Barriers
// -------------------------------------------

use std::sync::{Barrier};

#[test]
fn main_8() {
    let mut threads_vec = Vec::new();
    let tasks = Arc::new(Mutex::new(vec![]));
    let barrier = Arc::new(Barrier::new(5));

    for i in 0..5 {
        let tasks = tasks.clone();
        let barrier = barrier.clone();
        let handle = thread::spawn(move || {
            // Tasks 1
            tasks
                .lock()
                .unwrap()
                .push(format!("Thread {i}, Completed its part on Task 1"));

            barrier.wait();
            // Task 2
            tasks
                .lock()
                .unwrap()
                .push(format!("Thread {i}, Completed its part on Task 2"));

            barrier.wait();
            // Task 3
            tasks
                .lock()
                .unwrap()
                .push(format!("Thread {i}, Completed its part on Task 3"));
        });
        threads_vec.push(handle);
    }

    for handle in threads_vec {
        handle.join().unwrap();
    }

    let task_lock = &*tasks.lock().unwrap();
    for contents in task_lock {
        println!("{contents}");
    }
}


// -------------------------------------------
// 	 Scoped Threads
// -------------------------------------------

#[test]
fn main_9() {
    let mut vec = vec![1, 2, 3];

    thread::scope(|some_scope| {
        some_scope.spawn(|| {
            println!("Thread inside scope");
            println!("vec: {:?}", vec);
        });

        some_scope.spawn(|| {
            println!("Another Thread inside scope");
            //vec.push(4);
            println!("vec: {:?}", vec);
        });
    });

    println!("The scope finished");
    vec.push(5);
    println!("vec: {:?}", vec);
}


// -------------------------------------------
// 			 Thread Park
// -------------------------------------------

#[test]
fn main_10() {
    let data = Arc::new(Mutex::new(5));
    let data_clone = data.clone();
    let thread_1 = thread::spawn(move || {
        println!("Thread 1: I am doing some work");
        println!("Thread 1: I am doing some more work");
        // thread::sleep(Duration::from_secs(2));

        println!("Thread 1: Parked");
        thread::park();
        //thread::park_timeout(Duration::from_secs(4));

        println!("Thread 1: Printing the updated data");
        println!("Thread 1: Data: {:?}", *data.lock().unwrap());
    });

    let thread_2 = thread::spawn(move || {
        println!("Thread 2: I am working on updating the data");
        thread::sleep(Duration::from_secs(1));
        *data_clone.lock().unwrap() = 10;
        println!("Thread 2: Data updated completed");
    });
    thread_2.join();
    thread_1.thread().unpark();
    thread_1.join();
}
