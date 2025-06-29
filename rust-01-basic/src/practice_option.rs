// Problem 1: Fix the code so that it compiles.

pub fn solution_p1() {
    let my_chars = vec!['a', 'b', 'c', 'd'];
    match first_character(&my_chars) {
        Some(character) => println!("First character: {character}"),
        None => println!("Empty array"),
    }
}

fn first_character(chars: &Vec<char>) -> Option<char> {
    if chars.len() > 0 {
        Some(chars[0])
    } else {
        None
    }
}


// Problem 2: Fix the code so that it compiles.

pub fn solution_p2() {
    let user_fruit = String::from("apple");
    if let Some(fruit) = check_fruit(user_fruit) {
        println!("User's name: {fruit}");
    }
}

fn check_fruit(input_fruit: String) -> Option<String> {
    let fruit_basket = vec![
        String::from("mango"),
        String::from("apple"),
        String::from("banana"),
    ];
    for fruit in fruit_basket {
        if input_fruit == fruit {
            return Some(fruit);
        }
    }
    None // In case the if statement is not successfull for any fruit in the basket,
    // then the function will return a unit type while it is expecting an Option enum.
    // Therefore we need to explicitly return the None variant
}


