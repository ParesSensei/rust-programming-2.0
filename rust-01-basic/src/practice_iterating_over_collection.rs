// Problem 1: Fix the code so that it compiles
// Solution:

pub fn solution_p1() {
    let mut vec_1 = vec![4, 5, 6, 9, 8];
    for i in vec_1.iter_mut()  {
        *i = *i * 2;
    }
    println!("{:?}", vec_1);
}


// Problem 2: Convert the code based on the combinators
// Solution:

pub fn solution_p2() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut result = 0;

    let result: i32 = numbers
        .iter()
        .filter(|&&num| num % 2 != 0)
        .map(|&num| num * num)
        .sum();

    println!("Result without combinators: {}", result);
}