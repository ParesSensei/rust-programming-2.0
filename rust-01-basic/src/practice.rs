
// my solution

// problem1
#[test]
fn test1() {
    let my_age: i32  = 40;
    println!("My age is: {}", my_age);  // do not change this line
}

// problem2
#[test]
fn test2(){
    let x1 = 40;
    let mut x2 = x1;
    x2 = x1-2;     // do not change this line
    println!("x1 is: {} and x2 is: {}", x1,x2);  // do not change this line
}

// Problem 4: Fix the code below.
#[test]
fn test3() {
    let _a = "three";  // do not change this line
    let a = 10; // do not change the name of variable 'a'
    println!("a is: {}", a);
}

#[test]
// Problem 2: Make this program compile by replacing the variable type.
fn test4() {
    let x: u8; // Don't change this line!
    x = 1;
    println!("x is: {}", x);
}

#[test]
// Problem 2: Make this program compile by replacing the variable type.
fn test5() {
    let pi: f32;
    pi = 3.14159; // This value represents pi
    println!("pi is: {}", pi);
}

#[test]
// Problem 3: Replace the placeholder "DATA_TYPES_PLEASE" with the appropriate data types in the given program
fn test6() {
    let a: i16 = -15;
    let b: i16 = 170;
    let name: &str = "Michael";
    println!("name is: {}, and the multiplication result is {}", name, a * b);
}

// #[test]