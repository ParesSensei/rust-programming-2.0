// Problem 1: fill in the code for TODO's
// Solution:

use std::cell::RefCell;

pub fn solution_p1() {
    let data: RefCell<Option<i32>> = RefCell::new(Some(42));
    *data.borrow_mut() = None;

    if  data.borrow().is_some() {
        println!("Final value: {:?}", data.borrow());
    } else {
        println!("No value present.");
    }
}


// Problem 2: Fix the lines indicated in the code so that it compiles
// Solution:

struct Car {
    model: String,
    price: u32,
    status: RefCell<&'static str>,
}

impl Car {
    fn new(model: &str, price: u32) -> Self {
        Car {
            model: model.to_owned(),
            price,
            status: RefCell::new("Available"),
        }
    }

    fn sold(&self) {
        let new_status = match self.price {
            0..=50000 => "Sold - Economy",
            50001..=100000 => "Sold - Mid Range",
            _ => "Sold - Luxury",
        };
        *self.status.borrow_mut() = new_status;
    }
}

pub fn solution_p2() {
    let car = Car::new("Sedan", 75000);
    car.sold();
    println!("Car Status: {}", car.status.borrow());
}


// Problem 3: The code below will compile. However it will panic at execution time.
// Task 1: Your first task is to add some code so that the it does not panic at execution time.
// Task 2: The value at the last print line will not be displayed.
// Instead of value, <Borrowed> will be displayed.
// Add appropriate code so that the value of x is being displayed.


pub fn solution_p3() {
    let x = RefCell::new(5);
    let x_ref1 = x.borrow();
    let x_ref2 = x.borrow();
    println!("x_ref1: {}, x_ref2: {}", x_ref1, x_ref2);
    drop(x_ref1);
    drop(x_ref2);

    let mut x_ref3 = x.borrow_mut();
    *x_ref3 = 6;
    drop(x_ref3);

    println!("Stored value: {:?}", x);
}
