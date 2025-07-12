// Problem 1: Fix the code below
// Solution:

use std::sync::mpsc;
use std::thread;

pub fn solution_p1() {
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();

    thread::spawn(move || {
        let my_vec = vec![1, 2, 3, 4, 5];
        for i in my_vec {
            tx.send(i).unwrap();
        }
    });

    thread::spawn(move || {
        let my_vec = vec![6, 7, 8, 9, 10];
        for i in my_vec {
            tx_clone.send(i).unwrap();
        }
    });

    for recieved_vals in rx {
        println!("I recieved the value of {}", recieved_vals);
    }
}



// Problem 2: Complete the code below
// Solution:

fn thread_fn(d: i32, tx: mpsc::Sender<i32>) {
    thread::spawn(move || {
        println!("{} send!", d);
        tx.send(d).unwrap();
    });
}

pub fn solution_p2() {
    let (tx, rx) = mpsc::channel();
    for i in 0..5 {
        thread_fn(i, tx.clone());
    }

    drop(tx);

    for recieving_val in rx {
        println!("{} recieved!", recieving_val);
    }
}



// Problem 3: The code is unable to do some other stuff.
// Seems like a message is always available, when you make a call to try_recv().
// Fix the code so that it is able to do other work.
// Solution:


use std::time::Duration;
pub fn solution_p3() {
    let (tx, rx) = mpsc::channel();

    let t = thread::spawn(move || {
        let x = "some_value".to_string();
        println!("Sending value {x}");
        tx.send(x).unwrap();
    });


    let mut received_status = false;
    while received_status != true {
        match rx.try_recv() {
            Ok(received_value) => {
                println!("Received value is: {received_value}");
                received_status = true;
            }
            Err(_) => println!("I am doing some other stuff"),
        }
    }

    t.join();  // do not be surprised if the code does alot of other stuff and the value is received late
    // seems like the playground is not good at scheduling the threads
}
