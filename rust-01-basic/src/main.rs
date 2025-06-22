mod practice;
mod practice_function;

fn main() {
    my_fn("this is my function");
    let str: &str = "Function call with a variable";
    my_fn(str)
}

fn my_fn(s: &str) {
    println!("{}", s);
}