
//-------------------------------------------------------
//                  -Ownership basic
//-------------------------------------------------------


/*
1. each value has a variable that's its "owner."
2. A value can have only one owner at a time.
3. If the owner goes out of tge the scope, the value is cleaned up.
*/

fn main() {
    println!("Hello, world!");
}

#[test]
fn test1() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 is: {s1}")

    // not use .clone() on primitive datatype(int,bool, char, f64)
}