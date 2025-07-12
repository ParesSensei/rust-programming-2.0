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

