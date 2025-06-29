// Problem 1: Modify the code by eliminating all the let statements

pub fn solution_p1() {
    print_fruit(increase_fruit(new_fruit()));
}

struct Fruit {
    apples: i32,
    bananas: i32,
}

fn increase_fruit(fruit: Fruit) -> Fruit {
    Fruit {
        apples: fruit.apples * 2,
        bananas: fruit.bananas * 3,
    }
}

fn new_fruit() -> Fruit {
    Fruit {
        apples: 10,
        bananas: 5,
    }
}

fn print_fruit(fruit: Fruit) {
    println!(
        "You have {} apples and {} bananas",
        fruit.apples, fruit.bananas
    );
}


// Problem 2: Fix the code by only changing the indicated line.

struct Book {
    title: String,
}

pub fn solution_p2() {
    let some_book = Book {
        title: String::from("The Rust Programming Language"),
    };
    print_book(some_book.title.clone()); // Fix this line
    println!("Book name: {}", some_book.title);
}

fn print_book(book_name: String) {
    println!("Book: {}", book_name);
}
