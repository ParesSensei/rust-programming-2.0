// Problem 1: Fix the code by moving the indicated line to approperiate location
// Solution:

pub fn solution_p1() {
    let mut some_str = String::from("I am String");
    let ref1 = &some_str;
    println!("{ref1}");
    let ref2 = &mut some_str;
    ref2.push_str(" additional information");
    println!("{ref2}");
}


// Problem 2: Fix the code by moving the indicated line to approperiate location
// Solution:

fn identity(a: &i32) -> &i32 {
    a
}

pub fn solution_p2() {
    let mut x_ref: Option<&i32> = None;
    {
        let x = 7;
        x_ref = Some(identity(&x));
        assert_eq!(*x_ref.unwrap(), 7);
    }
}


//Problem 3: Fix the code by moving the indicated line to approperiate place
// Solution:

fn option(opt: Option<&i32>) -> &i32 {
    opt.unwrap()
}

pub fn solution_p3() {
    let y = 4;
    let answer = {
        option(Some(&y))
    };
    assert_eq!(answer, &4);
}


// Problem 4: Add lifetime annotations in the function signature
// Solution:

fn some_if_greater<'a>(number: &'a i32, greater_than: &i32) -> Option<&'a i32> {
    if number > greater_than {
        Some(number)
    } else {
        None
    }
}

pub fn solution_p4() {
    let num_1 = 7;
    let greater_val = 4;
    let test = some_if_greater(&num_1, &greater_val);
}
