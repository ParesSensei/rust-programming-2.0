//Problem 1: Add the method add_back() to the implementation of doubly linked list below.
// The method should add the element at the back or at the end of the linked list.

use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
struct Doubly_Linklist {
    head: pointer,
    tail: pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: pointer,
    prev: pointer,
}

type pointer = Option<Rc<RefCell<Node>>>;

impl Doubly_Linklist {
    fn new() -> Self {
        Doubly_Linklist {
            head: None,
            tail: None,
        }
    }

    fn add(&mut self, element: i32) {
        let new_head = Node::new(element);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }

            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn add_back(&mut self, element: i32) {
        let new_tail = Node::new(element);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail.clone());
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }
    fn remove(&mut self) -> Option<i32> {
        if self.head.is_none() {
            println!("List is empty so we can not remove");
            None
        } else {
            let removed_val = self.head.as_ref().unwrap().borrow().element;
            self.head
                .take()
                .map(|old_head| match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev = None;
                        self.head = Some(new_head);
                        self.head.clone()
                    }
                    None => {
                        self.tail = None;
                        println!("List is empty after removal");
                        None
                    }
                });
            Some(removed_val)
        }
    }

    fn print(&self) {
        let mut traversal = self.head.clone();
        while !traversal.is_none() {
            println!("{}", traversal.as_ref().unwrap().borrow().element);
            traversal = traversal.unwrap().borrow().next.clone();
        }
    }
}

impl Node {
    fn new(element: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            element: element,
            next: None,
            prev: None,
        }))
    }
}


pub fn solution_p1() {
    let mut list1 = Doubly_Linklist::new();

    list1.add(30);
    list1.add(32);
    list1.add(34);
    list1.add(36);
    list1.print();
    list1.add_back(1);
    list1.add_back(2);
    list1.print();
}
