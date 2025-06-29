// Problem 1: Fix the code below

pub fn solution_p1() {
    let mut some_vec = vec![1, 2, 3];
    let first = get_first_element(&some_vec);
    println!("The first number is: {}", first);
    some_vec.push(4);
}

fn get_first_element(num_vec: &Vec<i32>) -> &i32 {
    &num_vec[0]
}

// Problem 2: Fix the code so that it compiles.

pub fn solution_p2() {
    let vec_1 = vec![1, 2, 3];
    let vec_2 = vec![4, 5, 6];
    let mut vec_ptr: &Vec<i32>;
    vec_ptr = &vec_1;
    println!("vec ptr is pointing to vec_1");
    vec_ptr = &vec_2;
    println!("vec ptr is updated and now pointing to vec_2");
}

//Problem 3: Fix the code so that it compiles.

pub fn solution_p3() {
    let mut first_num = 42; // we are using mutable references to it so the variable must be mutable
    let mut second_num = 64; // we are using mutable references to it so the variable must be mutable
    let ref1 = &mut first_num;
    let mut ref2 = &mut second_num; // a mutable references means that the reference can be updated to point to some other variable

    *ref1 = 15;
    *ref2 = 10;
    ref2 = ref1;

    println!("Updated first number: {ref2}");
}
