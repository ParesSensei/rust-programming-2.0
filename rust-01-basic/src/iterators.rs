// -------------------------------------------
// 			Iterator
// -------------------------------------------

// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

struct Employee {
    name: String,
    salary: u16,
}

struct Employee_Records {
    employee_db: Vec<Employee>,
}

impl Iterator for Employee_Records {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.employee_db.len() != 0 {
            let result = self.employee_db[0].name.clone();
            self.employee_db.remove(0);
            Some(result)
        } else {
            None
        }
    }
}

#[test]
fn test_1() {
    let mut emp_1 = Employee {
        name: String::from("John"),
        salary: 40_000,
    };

    let mut emp_2 = Employee {
        name: String::from("Joseph"),
        salary: 30_000,
    };

    let mut emp_db = Employee_Records {
        employee_db: vec![emp_1, emp_2],
    };

    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());

    for employee in emp_db {
        println!("{employee}");
    }
}



// -------------------------------------------
// 		IntoIterator
// -------------------------------------------
/*
trait IntoIterator {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
*/

struct Book {
    title: String,
    author: String,
    genre: String,
}

// struct BookIterator {
//     properties: Vec<String>,
// }

// impl Iterator for BookIterator {
//     type Item = String;

//     fn next(&mut self) -> Option<Self::Item> {
//         if !self.properties.is_empty() {
//             Some(self.properties.remove(0))
//         } else {
//             None
//         }
//     }
// }

impl IntoIterator for Book {
    type Item = String;
    // type IntoIter = BookIterator;

    // fn into_iter(self) -> Self::IntoIter {
    //     BookIterator {
    //         properties: vec![self.title, self.author, self.genre],
    //     }
    // }

    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        vec![self.title, self.author, self.genre].into_iter()
    }
}

#[test]
fn test_2() {
    let book = Book {
        title: "Digital Image Processing".to_string(),
        author: "Gonzales".to_string(),
        genre: "Science Book".to_string(),
    };

    let mut book_iterator = book.into_iter();

    // println!("{:?}", book_iterator.next());
    // println!("{:?}", book_iterator.next());
    // println!("{:?}", book_iterator.next());
    // println!("{:?}", book_iterator.next());

    for book_info in book_iterator {
        println!("{book_info}");
    }
}


// -------------------------------------------
//         - Iterating Through Collections
// -------------------------------------------

use std::collections::HashMap;

#[test]
fn test_3() {
    let mut vec_1 = vec![45, 30, 85, 90, 41, 39];
    // let mut vec_1_iter = vec_1.into_iter();
    // let value_1 = vec_1_iter.next();

    for values in vec_1 {
        println!("{values}");
    }

    // println!("{:?}", vec_1);

    let mut person: HashMap<String, i32> = HashMap::new();
    person.insert("Hannash".to_string(), 40);
    person.insert("Joseph".to_string(), 44);
    person.insert("Sara".to_string(), 55);

    for (name, age) in person {
        println!("The person {} has an age of {}", name, age);
    }
}
