
//-------------------------------------------------------
//                  -Ownership basic
//-------------------------------------------------------


/*
1. each value has a variable that's its "owner."
2. A value can have only one owner at a time.
3. If the owner goes out of tge the scope, the value is cleaned up.
*/


#[test]
fn test1() {
    let s1 = String::from("hello");
    let _s2 = s1.clone();
    println!("s1 is: {s1}")

    // not use .clone() on primitive datatype(int,bool, char, f64)
}


//-------------------------------------------------------
//                  -Ownership in functions
//-------------------------------------------------------
#[test]
fn test2() {
    let vec_1 = vec![1, 2, 3];
    takes_ownership(vec_1.clone());
    println!("vec_1 is {:?}", vec_1);

    let vec2 = gives_ownership();
    println!("vec2 is {:?}", vec2);

    let vec3 = takes_and_gives_ownership(vec2);
    // println!("vec2 is {:?}", vec2);
    println!("vec3 is {:?}", vec3);

    let x = 10;
    stack_function(x);
    println!("in main, x is: {}", x);
}

fn takes_ownership(vec: Vec<i32>) {
    println!("{:?}", vec);
}

fn gives_ownership() -> Vec<i32> {
    vec![4,5,6]
}

fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(10);
    vec
}

fn stack_function (mut var: i32) {
    var = 56;
    println!("in func, var is {}", var);
}