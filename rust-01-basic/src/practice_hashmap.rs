// Problem 1:
/* In this exercise, you will be working on creating a student management system
using Rust. The system should allow you to store and retrieve student information
based on their unique ID. For ease of work, the student structure is already
created in the code below

Next, create a StudentManager structure containing a field of student, which
will essentially be a hashmap where the key part will be an integer representing
unique ID of student and the value part will be the complete details of the
students contained in the student structure.

The StudentManager should implement the following methods:
1. new() -> Self: A constructor that initializes an empty student manager.

2. add_student(&mut self, student: Student) -> Result<(), String>:
Adds a student to the manager.
If the student's ID already exists, return an error message.
Otherwise, add the student to the manager and return Ok.

3. get_student(&self, id: i32) -> Option<&Student>: Retrieves a student
from the manager based on their ID.
If the student is found, return Some(student). Otherwise, return None.

Your task is to implement the StudentManager structure, and the mentioned methods.
Additionally, provide a sample usage of the student management system by adding
a few students and retrieving their information using the get_student() method.
*/

use std::collections::HashMap;
struct Student {
    id: i32,
    name: String,
    grade: String,
}

struct StudentManager {
    students: HashMap<i32, Student>,
}

impl StudentManager {
    fn new() -> Self {
        StudentManager {
            students: HashMap::new(),
        }
    }

    fn add_student(&mut self, student: Student) -> Result<(), String> {
        if self.students.contains_key(&student.id) {
            Err(format!("Student with ID {} already exists", student.id))
        } else {
            self.students.insert(student.id, student);
            Ok(())
        }
    }

    fn get_student(&self, id: i32) -> Option<&Student> {
        self.students.get(&id)
    }
}

pub fn solution_p1() {
    let mut manager = StudentManager::new();

    let student1 = Student {
        id: 1,
        name: String::from("Alice"),
        grade: String::from("A"),
    };
    let student2 = Student {
        id: 2,
        name: String::from("Bob"),
        grade: String::from("B"),
    };

    manager.add_student(student1).unwrap();
    manager.add_student(student2).unwrap();

    // Retrieve and print student information
    if let Some(student) = manager.get_student(1) {
        println!("Student ID: {}", student.id);
        println!("Student Name: {}", student.name);
        println!("Student Grade: {}", student.grade);
    }
    if let Some(student) = manager.get_student(2) {
        println!("Student ID: {}", student.id);
        println!("Student Name: {}", student.name);
        println!("Student Grade: {}", student.grade);
    }
}


// Problem 2:
// Solution:

/* The code below contains a student struct with three fields.
Your taks is to create a hashmap called student_database in the main function,
which will store instances of the Student structure. The keys of the hashmap
should be unique student IDs, represented as integers while the values will be
instances of the student struct.

Implement a function called add_student() that takes the student's ID, name, age, and grade as parameters.
The function should add a new entry to the student_database hashmap if the student ID doesn't already exist.
If the student ID already exists in the hashmap, the function should not add a new entry.

Use the skeleton of the function given below.
*/

struct Student2 {
    name: String,
    age: i32,
    grade: String,
}

fn add_student(
    student_database: &mut HashMap<i32, Student2>,
    id: i32,
    name: String,
    age: i32,
    grade: String,
) {
    let student = Student2 { name, age, grade };
    if student_database.contains_key(&id) {
        println!("The id already exist");
    } else {
        student_database.insert(id, student);
    }
}

pub fn solution_p2() {
    let mut student_database: HashMap<i32, Student2> = HashMap::new();
    add_student(
        &mut student_database,
        1,
        String::from("John"),
        17,
        String::from("Grade 11"),
    );
    add_student(
        &mut student_database,
        2,
        String::from("Sarah"),
        16,
        String::from("Grade 10"),
    );

    // Printing the student database
    for (id, student) in &student_database {
        println!("Student ID: {}", id);
        println!("Name: {}", student.name);
        println!("Age: {}", student.age);
        println!("Grade: {}", student.grade);
        println!("------------------");
    }
}

