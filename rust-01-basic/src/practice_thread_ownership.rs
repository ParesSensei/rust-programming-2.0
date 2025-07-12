// Problem 1: Fix the code to make it compile.
// Solution:

use std::thread;

pub fn solution_p1() {
    let mut v = vec!["Nouman".to_string()];
    let handle = thread::spawn(move || {
        v.push("Azam".to_string());
    });
}


// Problem 2: Fix the code to make it compile. Do not delete any line.
// Solution

pub fn solution_p2() {
    let v = vec![1, 2, 3];
    let x = 5;
    let v1 = v.clone();
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v1);
        println!("Here's a variable : {:?}", x);
    });

    println!("The variable x is still alive {}", x);
    println!("The variable v is currenlty not alive {:?}", v);
    println!("Make approperiate changes so that it remains alive on this line");

    handle.join();
}