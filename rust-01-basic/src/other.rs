// -------------------------------------------
//           	- Efficient Programming Skills
// -------------------------------------------

#[test]
fn main_2() {
    let cancer: bool = true;
    let smooking = false;

    // nested conditions
    /*
    match cancer {
        true => match smooking {
            true => println!("Your cancer is likely due to smooking"),
            false => {
                println!("your cancner is not be due to smooking, further investigation needed")
            }
        },
        false => match smooking {
            true => println!("Smooking is dangerour and may cause canncer"),
            false => println!("You do not have any disease"),
        },
    }

     */

    // de-nested version
    match (cancer, smooking) {
        (true, true) => println!("Your cancer is likely due to smooking"),
        (true, false) => println!("your cancner may not be due to smooking, further investigation needed"),
        (false, true) => println!("Smooking is dangerour and may cause canncer"),
        (false, false) => println!("You do not have any disease"),

    }
}


#[test]
fn main_3() {
    let responces = vec![Ok(100), Err("Client error"), Ok(300), Err("Server Error")];
    //let responces = vec![Ok(100),  Ok(300)];

    let result: Result<Vec<_>, &str> = responces.into_iter().collect();
    println!("{:?}", result);
}



use std::collections::HashMap;
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn persons_by_name(persons: Vec<Person>) -> HashMap<String, Person> {
    persons.into_iter().map(|p| (p.name.clone(), p)).collect()
}

#[test]
fn main_4() {
    let person_1 = Person {
        name: "Joseph".to_string(),
        age: 40,
    };

    let person_2 = Person {
        name: "Micheal".to_string(),
        age: 30,
    };

    let person_3 = Person {
        name: "Alexander".to_string(),
        age: 45,
    };

    let persons = vec![person_1, person_2, person_3];

    let person_hash = persons_by_name(persons);

    for (name, details) in &person_hash {
        println!("Person {:?} has the details of {:?}", name, details);
    }
}



#[test]
fn main_5() {
	// for i in 0..10 {
    //for i in 10..0 {   // does not generate anything.
    // for i in (0..10).rev()  {  //the correct syntax
    // for i in (0..=10).rev()  {  // for including 10 also
    for i in (10..0).rev() { //again it will not generate anything
        print!("{} ", i);
    }
}



// -------------------------------------------
//           	- ToDo Macro
// -------------------------------------------
#[derive(Default)]
struct Student {
    name_std: String,
    age: u8,
    sex: char,
    country: String,
    salary: u32,
    nationality: String,
}

impl Student {
    fn some_fn_1(&self) -> String {
        // todo!()
        "".to_string()
    }

    fn some_fn_2(&self) -> u8 {
        todo!()
    }
}

trait GeneralInfo {
    fn info(&self) ->  (&str, u8,char);
    fn country_info(&self) -> &str;
}

#[test]
fn main(){
    let student_1 = Student::default();
    student_1.some_fn_1();
}