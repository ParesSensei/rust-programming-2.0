//-------------------------------------------------------
//                  -Structs and its types
//-------------------------------------------------------

use std::collections::HashMap;

struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

#[test]
fn test1() {
    let mut my_car: Car = Car {
        owner: String::from("ABC"),
        year: 32,
        fuel_level: 0.0,
        price: 5_000,
    };
    let _car_year = my_car.year;
    my_car.fuel_level = 30.0;

    let _another_car = Car {
        owner: "new owner".to_string(),
        ..my_car
    };

    // Tuple struct
    let _point_2d = (1, 2);
    let _point_3d = (2, 3, 4);

    struct Point2d(i32, i32);
    struct Point3d(i32, i32, i32);

    let _point1 = Point2d(1, 2);
    let _point2 = Point3d(2, 3, 4);

    // unit struct
    struct ABC;
}


//-------------------------------------------------------
//                  -Adding functionality on struct
//-------------------------------------------------------

struct Car1 {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

impl Car1 {
    fn monthly_insurance() -> u32 {
        123
    }

    fn selling_price(&self) -> u32 {
        self.price + Car1::monthly_insurance()
    }

    fn new(name: String, year: u32) -> Self {
        Self{
            owner: name,
            year,
            fuel_level: 0.0,
            price: 0,
        }
    }

    fn display_car_info(&self) {
        println!(
            "Owner: {}, Year {}, Price {}",
            self.owner, self.year, self.price
        )
    }

    fn refuel(& mut self, gallons: f32) {
        self.fuel_level += gallons;
    }

    fn sell(self) -> Self {
        self
    }
}


#[test]
fn test2() {
    let mut my_car1: Car1 = Car1 {
        owner: String::from("ABC"),
        year: 32,
        fuel_level: 0.0,
        price: 5_000,
    };

    my_car1.display_car_info();

    my_car1.refuel(10.5);
    let _new_owner = my_car1.sell();

    let _new_car = Car1::new("XYZ".to_string(), 2020);
}

//-------------------------------------------------------
//                  -Enums
//-------------------------------------------------------

// enum WeekDay {
//     Monday,
//     Tuesday,
//     Wednesday,
//     Thursday,
//     Friday,
//     Saturday,
//     Sunday,
// }
//
// #[test]
// fn test3() {
//     let mut day = "Saturday".to_string();
//
//     let week_day = vec![
//         "Monday".to_string(),
//         "Tuesday".to_string(),
//         "Wednesday".to_string(),
//         "Thursday".to_string(),
//         "Friday".to_string(),
//         "Saturday".to_string(),
//         "Sunday".to_string(),
//     ];
//     day = week_day[1].clone();
//
//     let day WeekDay::Saturday;
// }

enum TravelType {
    Car,
    Train,
    Aeroplane,
}

impl TravelType {
    fn travel_allowance(&self, miles: f32) -> f32 {
        let allowance = match self {
            TravelType::Car => miles * 2.0,
            TravelType::Train => miles * 3.0,
            TravelType::Aeroplane => miles * 5.0,
        };
        allowance
    }
}

#[test]
fn test3() {
    let participant = TravelType::Car;
    println!(
        "Allowance of participant is: {}",
        participant.travel_allowance(60.0)
    );
}

//-------------------------------------------------------
//                  -Options
//-------------------------------------------------------

struct Student {
    name: String,
    grade: Option<u32>
}

// fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
//     for student in student_db {
//         if student.name == *student_name{
//             return student.grade;
//         }
//     }
//     None
// }
//
// fn check_student(student_name: &String, student_db: &Vec<Student>) -> Result<(), String> {
//     for student in student_db {
//         if student.name == *student_name {
//             return Ok(());
//         }
//     }
//     Err(String::from("Student not found"))
// }

fn check_student_get_grade(student_name: &String, student_db: &Vec<Student>) -> Result<Option<u32>, String> {
    for student in student_db {
        if student.name == *student_name {
            return Ok(student.grade);
        }
    }
    Err(String::from("Student not found"))
}

#[test]
fn test4() {
    let student_db = vec![
        Student {
            name: String::from("Alice"),
            grade: Some(90),
        },
        Student {
            name: String::from("Bob"),
            grade: Some(87),
        },
        Student {
            name: String::from("Charlie"),
            grade: None
        },
    ];

    let student_name = String::from("Bob");
    let student_status = check_student_get_grade(&student_name, &student_db);

    match student_status {
        Ok(option_grade) => {
            if let Some(grade) = option_grade {
                println!("Grade is {}", grade);
            }
        }
        Err(error_msg) => println!("{error_msg}")
    }

    // let student_grade = get_grade(&student_name, &student_db);
    //
    // match student_grade {
    //     Some(grade) => println!("grade is: {}", grade),
    //     None => {}
    // }
}

//-------------------------------------------------------
//                  -Hashmap
//-------------------------------------------------------

#[test]
fn test5() {
    let mut person: HashMap<&str, u32> = HashMap::new();
    person.insert("Api",100);
    person.insert("Bob", 140);
    person.insert("Charlie", 12);

    println!("the age is : {:?}", person.get("Bob").unwrap());

    if person.contains_key("Bob"){
        println!("the value exists");
    } else {
        println!("value not exist");
    }

    match person.get("Bob") {
        Some(value) => println!("The value exists {}", value),
        None => println!("Value not exist"),
    }

    for (name, age) in &person {
        println!("The person {} has an age is {}", name, age);
    }
}

#[test]
fn test6() {
    let mut likes: HashMap<&str, &str> = HashMap::new();

    likes.entry("nouman").or_insert("apple");
    likes.entry("nouman").or_insert("manggo");
    println!("the likes are: {:?}", likes);
}

#[test]
fn test7() {
    let some_vec = vec![2,3,4,5,3,2,4,3,5,2,3,4,3,4,2,4,2,4,3,3,2];
    let mut freq_vec: HashMap<i32, i32> = HashMap::new();

    for i in &some_vec {
        let freq: &mut i32 = freq_vec.entry(*i).or_insert(0);
        *freq += 1;
    }

    println!("{:?}", freq_vec);
}

//-------------------------------------------------------
//                  - Associativity
//                  - Explicit Boolean in conditional
//                  - Operator overloading
//-------------------------------------------------------

#[test]
fn test_aasociatifity() {
    // Assocatifity

    let x = 8 / 4 / 2; // (8 /4 ) / 2 = 1
    let mut y = 42;
    // x = y = 0; // x = (y = 0) // x = ()

    // right to left
    // =, +=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=

    let mut a = 1;
    let mut b = 2;
    let mut c = 3;
    // b += c += 42

    // Explicit boolean in value
    let x = 0;
    if x != 0 {}

    // Operator overloading
    let a = 10 + 10;
    let b = String::from("1") + ("2");
    println!("{}", b);
}

//-------------------------------------------------------
//                  -Pattern matching contexts
//-------------------------------------------------------

// value: the thing you are trying to match against
// Pattern: The shape or structure you are matching

#[test]
fn test8() {
    // 1. Match expression
    let x = 3;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("Something else"),
    }

    // Value: x
    // Pattern: 1,2,3,_

    // 2. if let
    let x = 3;
    if let 5 = x { // if x == 5
        println!("Matched five");
    }

    // Value: x
    // pattern: 5

    if let x = 5{ // let x = 5
        println!("This is always run");
        println!("X: inner {}", x);
    }

    println!("x: outer {}", x);

    // binding pattern
    // Value: Contrencate value
    // Pattern: variable

    // 3. while let
    let numbers = vec![1, 2, 3, 4, 5];
    let mut i = 0;

    while let 2 = numbers[i] {
        println!("Found a value 2 at index : {}", i)
    }

    // Value: number[i]
    // pattern : 2

    // 4. let binding
    let (a, b) = (10, 20);

    // value: (10, 20)
    // pattern : (a, b)

    // 5. Function parameter
    let point = (5, 8);
    print_cords(point);
    // value: (5, 8)
    // pattern : (x, y)
    // type: (132, i32)

}

fn print_cords((x, y) : (i32, i32)) {
    println!("x: {}, y: {}", x, y);
}

//-------------------------------------------------------
//                  -Casting of reference
//-------------------------------------------------------

#[test]
fn test_casting() {
    let x = 5;
    let y = x as f32;

    // casting immutable reference -> mutable reference (not allowed)
    // let data = 40;
    // let immutable_ref = &data;
    // let mutable_ref = immutable_ref as &mut i32;

    // Casting mutable reference -> immutable (allowed)
    let mut data = 42;
    let mutable_ref = &mut data;
    let immutable_ref = mutable_ref as &i32;

    /*
    Immutable To Mutable (not allowed)                  ||  mutable to immutable (Allowed)
    - since many immutable can coexist                  ||   - There is a single mutable reference at any time
    therefore,                                          ||     therefore,
    changing one of them to mutable may lead to         ||     changing it to immutable does not lead to violation
    coexistence of mutable and immutable (violation)    ||
     */

    // assignment of reference
    let mut str = String::from("hello");
    let ref_str_1 = &str;
    let ref_str_2 = ref_str_1;
    println!("{ref_str_1}");
}

//-------------------------------------------------------
//                  -destructured struct parameter
//-------------------------------------------------------

struct Point {
    x: i32,
    y: i32,
}

fn print_coooord(Point { x, y }: Point) {
    // Value: p or point {x: 5, y: 7}
    // pattern: Ppoint {x,y}
    println!("x: {}, y: {}", x, y);
}
#[test]
fn test_struckkk() {
    let p = Point { x: 1, y: 2 };
    match p {
        Point { x: 0, y} => println!("on the y axist-at: {}", y ),
        Point { x, y: 0} => println!("on the x axist-at: {}", x ),
        Point { x, y} => println!("at point ({}, {})", x, y ),
    }

    // first arm
    // Value: p or point { x: 0, y: 7 }
    // Pattern: Point {x: 0, y}
    let p = Point { x: 5, y: 6 };
    print_coooord(p);
}