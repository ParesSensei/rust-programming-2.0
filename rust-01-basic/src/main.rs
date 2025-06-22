mod practice;
mod practice_function;
mod practice_condtitional_and_controlflow;
//-------------------------------------------------------
//                  -Control flow
//                  -if else
//                  -Pattern match
//-------------------------------------------------------

fn main() {
    // my_fn("this is my function");
    // let str: &str = "Function call with a variable";
    //
    // let marks: i32 = 95;
    // let mut grade: char = 'n';
    //
    // if marks >= 90 {
    //     grade = 'A';
    // } else if marks >= 80 {
    //     grade = 'B';
    // } else if marks >= 70 {
    //     grade = 'C';
    // } else {
    //     grade = 'F';
    // }

    let marks: i32 = 95;
    let grade: char = if marks >= 90 {
        println!("You are great student");
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 70 {
        'C'
    } else {
        'F'
    };

    println!("The grade is {}", grade);

}
#[test]
fn grade_using_pattren_matching() {
    let marks: i32 = 95;
    let mut grade: char = 'F';

    match marks{
        90..=100 => {
            println!("You are the best");
            grade = 'A'
        },
        80..=89 => grade = 'B',
        70..=79 => grade = 'C',
        _ => grade = 'F',
    }

    println!("Your grade is {}", grade);
}

#[test]
fn grade_using_pattren_matching_2() {
    let marks: i32 = 95;
    // let mut grade: char = 'F';

    let grade = match marks{
        90..=100 => {
            println!("You are the best");
            'A'
        },
        80..=89 =>'B',
        70..=79 =>'C',
        _ => 'F',
    };

    println!("Your grade is {}", grade);
}


//-------------------------------------------------------
//                  -Loops
//                  -For loops
//                  -While loops
//-------------------------------------------------------
#[test]
fn loop1() {
    'outer: loop {
        println!("simple loop");
        break 'outer;
    }

    let a: i32 = loop {
        break 5
    };
    println!("a is {}", a);
}

#[test]
fn for_loop() {
    let vec: Vec<i32> = vec![45,56,12,56,23,66];

    for i in vec {
        println!("{}", i);
    }
}

#[test]
fn while_loop() {
    let mut num: i32 = 0;
    while num < 10 {
        num += 1;
        println!("{}", num);
    }
}

// #[test]