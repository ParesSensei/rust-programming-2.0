//-------------------------------------------------------
//                  -Structs and its types
//-------------------------------------------------------

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