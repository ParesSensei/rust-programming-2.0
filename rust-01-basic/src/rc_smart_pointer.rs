// -------------------------------------------
// 		Reference Counting Smart Pointer
// -------------------------------------------

use std::rc::Rc;
enum List {
    Cons(i32, Option<Rc<List>>),
}

#[test]
fn test() {
    let a = Rc::new(List::Cons(1, Some(Rc::new(List::Cons(2, None)))));
    println!("Reference count after a: {}", Rc::strong_count(&a));
    {
        let b = List::Cons(3, Some(Rc::clone(&a)));
        println!("Reference count after b: {}", Rc::strong_count(&a));

        let c = List::Cons(4, Some(Rc::clone(&a)));
        println!("Reference count after c: {}", Rc::strong_count(&a));
    }
    println!("Reference count after scope: {}", Rc::strong_count(&a));
}


// -------------------------------------------
// 		RefCell Smart Pointer
// -------------------------------------------

use std::{cell::RefCell};

#[test]
fn test3() {
    // Example 1: Borrowing rules at run time
    // let mut x = 50;
    // let x1 = &x;
    // let x2 = &x;
    // let x3 = &mut x;

    // println!("{} {} ", x1, x2);

    let a = RefCell::new(10);

    //{ // add the scope later on
    let b = a.borrow();
    let c = a.borrow();
    //}

    drop(b); // add later on, remove after adding the scope above
    drop(c); // add later on
    let d = a.borrow_mut();
    // drop(d) // add later on
    //println!("{} {}", b, c); // later on delete this
    //println!("Value of a is : {:?}", a); // add later on


    // Example 2: Interior mutability
    // let x = 32;
    // let x1 = &mut x;

    let a = RefCell::new(10);
    //let c = *a; // add later on
    let mut b = a.borrow_mut();
    *b = 15;

    drop(b); // add later on
    println!("{:?}", a);

    // Example 3
    let a = Rc::new(RefCell::new(String::from("c++")));
    let b = Rc::clone(&a);

    *b.borrow_mut() = String::from("rust");
    println!("{:?}", a);
}
