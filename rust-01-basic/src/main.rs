mod practice;
mod practice_function;
mod practice_conditional_and_controlflow;
mod ownership;
mod practice_on_ownership;
mod borrowing;
mod practice_borrowing;
mod dereferencing;
mod custom_library;
mod practice_struct;
mod practice_enum;
mod practice_option;
mod practice_result;
mod practice_hashmap;
mod flexibility_and_abstraction_with_generic_and_trait;
mod practice_generic;
mod practice_trait;
mod trait_bound;
mod practice_trait_bound;
mod super_trait;
mod practice_super_trait;
mod trait_object;
mod practice_trait_object;
mod practice_trait_object2;
mod derived_marker_trait;
mod practice_associated_types;
mod closure;
mod practice_closure;
mod functional_pointer;
mod practice_functional_closure;
mod iterators;
mod practice_iterator_intoiter;
mod practice_iterating_over_collection;
mod combinators;
mod lifetimes;
mod practice_lifetime;
mod practice_lifetime_elison;
mod box_smart_pointer;
mod practice_box_smart_pointer;
mod rc_smart_pointer;
mod practice_rc_smart_pointer;
mod practice_refcell_smart_pointer;
mod singly_llinked_list;
mod practice_singly_linked_list;
mod practice_singly_linked_list_2;
mod doubly_linked_list;
mod practice_double_linked_list;
mod practice_double_linked_list_2;
mod practice_double_linked_list_3;
mod builder_pattern;
mod size;
mod concurrency;
mod practice_thread;
mod practice_thread_ownership;
mod practice_thread_message_passing;
mod asynchronous;
mod practice_async_await;
mod macro_basic;
mod question_mark_operator;
mod web_programming;
mod file_handling;
mod directory_and_path;
mod regular_expression;
mod other;
//-------------------------------------------------------
//                  -Control flow
//                  -if else
//                  -Pattern match
//-------------------------------------------------------

fn main() {
    // practice_conditional_and_controlflow::solution_p1();
    // practice_conditional_and_controlflow::solution_p2();
    // practice_conditional_and_controlflow::solution_p3();
    // practice_conditional_and_controlflow::solution_p4();
    // practice_conditional_and_controlflow::solution_p6();
    // practice_on_ownership::solution_p1();
    // practice_on_ownership::solution_p2();
    // practice_on_ownership::solution_p3();
    // practice_borrowing::solution_p1();
    // practice_borrowing::solution_p2();
    // practice_borrowing::solution_p3();
    // practice_struct::solution_p1();
    // practice_struct::solution_p2();
    // practice_enum::solution_p1();
    // practice_enum::solution_p2();
    // practice_option::solution_p1();
    // practice_option::solution_p2();
    // directive()
    // practice_result::solution_p1();
    // practice_result::solution_p2();
    // practice_hashmap::solution_p1();
    // practice_hashmap::solution_p2();
    // practice_generic::solution_p1();
    // practice_generic::solution_p2();
    // practice_generic::solution_p3()
    // practice_trait::solution_p1();
    // practice_trait::solution_p2();
    // practice_trait_bound::solution_p1();
    // practice_trait_bound::solution_p2();
    // practice_trait_bound::solution_p3();
    // practice_super_trait::solution_p1();
    // practice_super_trait::solution_p2();
    // practice_trait_object::solution_p1();
    // practice_trait_object::solution_p2();
    // practice_trait_object2::solution_p3();
    // practice_associated_types::solution_p1();
    // practice_associated_types::solution_p2();
    // practice_closure::solution_p1();
    // practice_closure::solution_p2();
    // practice_closure::solution_p3();
    // practice_closure::solution_p4();
    // practice_functional_closure::solution_p1();
    // practice_functional_closure::solution_p2();
    // practice_iterator_intoiter::solution_p1();
    // practice_iterator_intoiter::solution_p2();
    // practice_iterator_intoiter::solution_p3();
    // practice_iterating_over_collection::solution_p1();
    // practice_iterating_over_collection::solution_p2();
    // practice_lifetime::solution_p1();
    // practice_lifetime::solution_p2();
    // practice_lifetime::solution_p3();
    // practice_lifetime::solution_p4();
    // practice_lifetime_elison::solution_p1();
    // practice_box_smart_pointer::solution_p1();
    // practice_box_smart_pointer::solution_p2();
    // practice_box_smart_pointer::solution_p3();
    // practice_box_smart_pointer::solution_p4();
    // practice_rc_smart_pointer::solution_p1();
    // practice_rc_smart_pointer::solution_p2();
    // practice_refcell_smart_pointer::solution_p1();
    // practice_refcell_smart_pointer::solution_p2();
    // practice_refcell_smart_pointer::solution_p3();
    // practice_singly_linked_list::solution_p1();
    // practice_singly_linked_list_2::solution_p2();
    // practice_double_linked_list::solution_p1();
    // practice_double_linked_list_2::solution_p2();
    // practice_double_linked_list_3::solution_p3();
    // practice_thread::solution_p1();
    // practice_thread::solution_p2();
    // practice_thread_ownership::solution_p1();
    // practice_thread_ownership::solution_p2();
    // practice_thread_message_passing::solution_p1();
    // practice_thread_message_passing::solution_p2();
    // practice_thread_message_passing::solution_p3();
    // practice_async_await::solution_p1();
    practice_async_await::solution_p2();
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

//-------------------------------------------------------
//                  -Print
//                  -Comment
//                  -Input/output
//-------------------------------------------------------

#[test]
fn test1() {
    // single comment

    /*
    multi
    line comment
     */

    print!("This is a print commnad");
    print!("This be going print on the same line");

    /* Escape sequences
     \n : New line chracter
     \t : tab space
     \r : Carriage return
     \" : Double quote
     \\ : Backward slash
     */

    println!("\nWill be printed after one empty line");
    println!("\t A tab space at the start");
    println!("This will be overwrtitten \r This text will only appear on the screen");
    println!("Peint double quotes \", Print Backslash \\");

    println!(
        "I am doing {2} from {1} year and i {0} it",
        "like", 20, "Programming"
    );

    println!(
        "{language} is a system programming language which is cool to {activity} in.\n",
        activity = "code",
        language = "Rust"
    );

    static WELCOME: &str= "Welcome to rust";
    const PI: f32 = 3.14;

    let _a = PI;
    let _b = PI;

    let _c = WELCOME;
    let _d = WELCOME;
}

fn test_input() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");

    let n: f64 = n.trim().parse().expect("invalid input");
    println!("{}", n);
}


//-------------------------------------------------------
//                  -Compiler Directive
//-------------------------------------------------------

#[allow(unused_variables)]
fn directive() {
    let i = 10;
    let s: String = String::from("Hello");
}

