// -------------------------------------------
// 		Link List (Part 1)
// -------------------------------------------

#[derive(Debug)]
struct Linklist {
    head: pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: pointer,
}

type pointer = Option<Box<Node>>;

#[test]
fn test_1() {
    // let list = Node {
    //     element: 1,
    //     next: None,
    // };

    // let list = Node {
    //     element: 1,
    //     next: Some(Box::new(Node {
    //         element: 2,
    //         next: None,
    //     })),
    // };

    // let list = Linklist {
    //     head: Some(Node {
    //         element: 1,
    //         next: None,
    //     }),
    // };

    // let list = Linklist {
    //     head: Some(Node {
    //         element: 1,
    //         next: Some(Box::new(Node {
    //             element: 2,
    //             next: Some(Box::new(Node {
    //                 element: 3,
    //                 next: None,
    //             })),
    //         })),
    //     }),
    // };

    // let list = Linklist { head: None };

    let list = Linklist {
        head: Some(Box::new(Node {
            element: 100,
            next: Some(Box::new(Node {
                element: 200,
                next: None,
            })),
        })),
    };

    println!("{:?}", &list.head);
}


// -------------------------------------------
// 		Link List (Part 2)
// -------------------------------------------

// #[derive(Debug)]
// struct Linklist {
//     head: pointer,
// }
//
// #[derive(Debug)]
// struct Node {
//     element: i32,
//     next: pointer,
// }
// type pointer = Option<Box<Node>>;

impl Linklist {
    fn new() -> Linklist {
        Linklist { head: None }
    }

    fn add(&mut self, element: i32) {
        // match self.head {
        //     None => {
        //         let new_node = Some(Box::new(Node {
        //             element: element,
        //             next: None,
        //         }));
        //         self.head = new_node;
        //     }
        //     Some(previous_head) => {
        //         let new_node = Some(Box::new(Node {
        //             element: element,
        //             next: Some(previous_head),
        //         }));
        //         self.head = new_node;
        //     }
        // }

        // fn take<T>(dest: &mut T) -> T
        let previous_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element: element,
            next: previous_head,
        }));
        self.head = new_head;
    }

    fn remove(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }

    fn print(&self) {
        let mut list_traversal = &self.head;
        while !list_traversal.is_none() {
            println!("{:?}", list_traversal.as_ref().unwrap().element);
            list_traversal = &list_traversal.as_ref().unwrap().next;
        }
    }
}


#[test]
fn test_2() {
    let mut list = Linklist::new();
    list.add(5);
    list.add(7);
    list.add(10);
    list.add(15);
    list.add(20);

    //println!("List: {:?}", list);
    list.print();
    println!("{}", list.remove().unwrap());
}
