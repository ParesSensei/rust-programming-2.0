// Problem 1: Complete the code by adding the closure definition
// Solution:

pub fn solution_p1() {
    let x = 10;
    let add_to_x = |y| x+y;

    let result = add_to_x(5);
    println!("Result: {}", result);
}


// Problem 2: Complete the 'process_employee' function signature by adding the suitable trait bounds
// Solution:

struct Employee {
    name: String,
    salary: u32,
    department: String,
}

fn process_employees<V1, V2>(
    employees: Vec<Employee>,
    name_transformer: V1,
    salary_filter: V2,
) -> Vec<String>
where
    V1: Fn(&str) -> String,
    V2: Fn(u32) -> bool,
{
    let mut processed_names = Vec::new();

    for employee in employees {
        if salary_filter(employee.salary) {
            processed_names.push(name_transformer(&employee.name));
        }
    }

    processed_names
}

pub fn solution_p2() {
    let employees = vec![
        Employee {
            name: String::from("Alice"),
            salary: 60000,
            department: String::from("Engineering"),
        },
        Employee {
            name: String::from("Bob"),
            salary: 75000,
            department: String::from("Sales"),
        },
        Employee {
            name: String::from("Charlie"),
            salary: 50000,
            department: String::from("Marketing"),
        },
    ];

    let transform_name_to_uppercase = |name: &str| name.to_uppercase();

    let filter_salary_above_threshold = |salary: u32| salary > 60000;

    let processed_names = process_employees(
        employees,
        transform_name_to_uppercase,
        filter_salary_above_threshold,
    );

    println!("Processed names: {:?}", processed_names);
}


// Problem 3: Complete the code by adding the closure definition

pub fn solution_p3() {
    let mut counter = 0;

    let mut increment_counter = || counter +=1;
    increment_counter();
    increment_counter();

    println!("Final Counter: {}", counter);
}


// Problem 4: Fix the struct definition to allow closures with event handling logic.
// Solution:

struct EventHandler<T>
where
    T: FnMut(),
{
    on_event: T,
}

impl<T> EventHandler<T>
where
    T: FnMut(),
{
    fn handle_event(&mut self) {
        (self.on_event)()
    }
}

pub(crate) fn solution_p4() {
    let mut lights_on = false;
    let mut temperature = 25;

    let mut lights_handler = EventHandler {
        on_event: || {
            lights_on = !lights_on;
            println!("Lights are now {}", if lights_on { "on" } else { "off" });
        },
    };

    let mut temperature_handler = EventHandler {
        on_event: || {
            temperature += 5;
            println!("Temperature increased to {}°C", temperature);
        },
    };

    lights_handler.handle_event();
    temperature_handler.handle_event();
    temperature_handler.handle_event();
    lights_handler.handle_event();

    assert_eq!(temperature, 35);
    assert_eq!(lights_on, false);
}
