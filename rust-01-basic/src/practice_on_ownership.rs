// Problem 1: Fix the code below so that it compiles

pub fn solution_p1() {
    let s1: String = String::from("this is me, ");
    let s2: &str = "Nouman";
    some_function(s1.clone(), s2); // Something is wrong here
    println!("{} {}", s1, s2);
}

fn some_function(a1: String, a2: &str) {
    println!("{} {}", a1, a2);
}


// Problem 2:

/*
Fix the code below. By solving this problem you will be able to understand
the change of ownership inside a loop
*/

pub fn solution_p2() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let mut temp;

    while !my_vec.is_empty() {
        temp = my_vec.clone(); // Something wrong on this line
        println!("Elements in temporary vector are: {:?}", temp);
        println!("Popped element: {}", my_vec.pop().unwrap()); // pop() is used to remove an element from the vec. The unwrap() is used to return the value inside the Some() variant
    }
}

// Problem 3: Fix the code so that it compiles.
// Solution:
#[allow(unused_variables)] // this line will supress the warning of unused variables

pub fn solution_p3() {
    let str1 = {
        let str1 = generate_string();
        str1
    };


    // An alternate solution would be to move the statement
    // let str1 = generate_string(); out of the scope

    let str2 = str1;   // Something wrong with this line
}

fn generate_string() -> String {
    let some_string = String::from("I will generate a string");
    some_string
}