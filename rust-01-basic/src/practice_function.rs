

#[test]
// Problem 1: Add the following missing functions to the code.

// 1. add_3(x): This function should add three to the input variable ‘x’ and the return the resultant value.
// 2. add_5(x): This function should add five to the input variable ‘x’ and the return the resultant value.
// 3. times(x,y): This function should compute the multiplication of the inputs ‘x’ and ‘y’ and return the resultant value.
fn solution_p1() {
    let x = 3;
    let y = 4;
    println!(
        "The result of x+3 times y+5 is {}",
        times(add_3(x), add_5(y))
    );
}

fn add_3(x: i8) -> i8 {
    x + 3
}

fn add_5(y: i8) -> i8 {
     y + 5
}

fn times(x: i8, y: i8) -> i8 {
    x * y
}

#[test]
// Problem 2: Refactor the code by taking rid of the variables x and y in main.
// The main should only consist of a print statement.

// Further Explanation: Rewrite the code in a way that produce the same outcome
// as the original code, but without using any variables.
fn solution_p2() {
    println!("Answer: {}", result());
}

fn double(x: i32) -> i32 {
    x * 2
}

fn triple(x: i32) -> i32 {
    x * 3
}

fn result() -> i32 {
    let x = triple(double(5));
    let y = triple(x);
    return y;
}

#[test]
// Problem 3: Correct the code below so that it compiles.
fn solution_p3() {
    println!(
        "The distance of the number the point from the origin is {}",
        print_distance((5.0, 4.0)) // Concentrate on the call to the function
    );
}
fn print_distance(point: (f32, f32)) -> f32 {
    let (x, y) = point;
    (x.powf(2.0) + y.powf(2.0)).sqrt() // Formula for computing distance
}


#[test]
// Problem 4: Add the definition of the quadruple function below by calling the double function twice inside the quadruple function.
// Solution:
fn main() {
    println!(
        "For 1: the expected value is 4 while the output is {}",
        quadruple(1)
    );

    println!(
        "For 2: the expected value is 8 while the output is {}",
        quadruple(2)
    );

    println!(
        "For 3: the expected value is 12 while the output is {}",
        quadruple(3)
    );

    println!(
        "For 4: the expected value is 16 while the output is {}",
        quadruple(4)
    );
}

fn doublee(x: i32) -> i32 {
    x * 2
}

fn quadruple(x: i32) -> i32 {
    // your code here //
    doublee(doublee(x))
}

// Explanation: The function quadruple takes an integer x and returns four times
// its value by calling the double function twice—once to double x,
// and again to double the result. This approach reuses the double function
// to achieve multiplication by 4 in a concise and readable way.


