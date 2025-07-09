//Problem 2: We want to change the linked list implementation by making the element part of
// the node as generic rather then concrete i32. Make approperiate changes to the code below.
// For printing, T should have the trait bound of  std::fmt::Debug and
// for the peek to work, T must also have the trait bound of std::marker::Copy
// Solution:


#[derive(Debug)]
struct Linklist<T> {
    head: pointer<T>,
}

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: pointer<T>,
}
type pointer<T> = Option<Box<Node<T>>>;

impl<T: std::fmt::Debug + std::marker::Copy> Linklist<T> {
    fn new() -> Linklist<T> {
        Linklist { head: None }
    }

    fn add(&mut self, element: T) {
        let previous_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element: element,
            next: previous_head,
        }));
        self.head = new_head;
    }

    fn remove(&mut self) -> Option<T> {
        match self.head.take() {
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<T> {
        match &self.head {
            Some(H) => Some(H.element),
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

pub fn solution_p2() {
    let mut list = Linklist::new();
    list.add(5);
    list.add(7);
    list.add(10);
    list.add(15);
    list.add(20);

    println!("{:?}", list.peek());
}
