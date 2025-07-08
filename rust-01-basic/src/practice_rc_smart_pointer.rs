// Problem 1: Put correct values in the assert_eq! macro so that the code does not panic
// Only use numbers as second parameter to assert_eq's.

use std::rc::Rc;
#[derive(Debug)]
struct ListNode<T> {
    value: T,
    next: Option<Rc<ListNode<T>>>,
}

pub fn solution_p1() {
    let node_3 = Rc::new(ListNode {
        value: 3,
        next: None,
    });

    let node_2 = Rc::new(ListNode {
        value: 2,
        next: Some(Rc::clone(&node_3)),
    });

    let node_1 = Rc::new(ListNode {
        value: 1,
        next: Some(Rc::clone(&node_2)),
    });

    assert_eq!(Rc::strong_count(&node_1), 1); // put a value inplace of ?
    assert_eq!(Rc::strong_count(&node_2), 2); // put a value inplace of ?
    assert_eq!(Rc::strong_count(&node_3), 2); // put a value inplace of ?
}


// Problem 2: In the code below, we would like to code the idea that multiple
// users can own a file. Complete the code by adding relevant code

struct File {}

struct User {
    file: Rc<File>,
}

pub fn solution_p2() {
    let txt_file = Rc::new(File {});

    let user_1 = User {
        file: Rc::clone(&txt_file),
    };

    let user_2 = User {
        file: Rc::clone(&txt_file),
    };
}
