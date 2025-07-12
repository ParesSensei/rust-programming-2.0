// Problem 1: Complete the code below.

use std::thread;

pub fn solution_p1() {

    let mut thread_vec = vec![];
    for i in 0..10 {
        thread_vec.push(
            // insert code here
            // Spawn a thread
            thread::spawn(|| {
                println!("Hi from thread")
            })
            // include a simple print statement with a msg of "Hi from Thread"
        );
    }

    // The code below will make sure that all the threads go to completion
    for i in thread_vec {
        i.join();
    }
}



// Problem 2: Complete the code below
// Solution:

pub fn solution_p2() {
    let handle_1 = thread::spawn(|| {
        let mut sum = 0;
        let range = 0..=1_000;
        for num in range {
            sum += num;
        }
        sum
    });

    let handle_2 = thread::spawn(|| {
        let mut sum = 0;
        let range = 1_001..=2_000;
        for num in range {
            sum += num;
        }
        sum
    });

    let handle_3 = thread::spawn(|| {
        let mut sum = 0;
        let range = 2_001..=3_000;
        for num in range {
            sum += num;
        }
        sum
    });

    let mut sum = 0;
    sum += handle_1.join().unwrap();
    sum += handle_2.join().unwrap();
    sum += handle_3.join().unwrap();

    println!("Final Summation Result {sum}");
}
