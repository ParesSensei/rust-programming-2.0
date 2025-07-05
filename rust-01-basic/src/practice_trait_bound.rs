// Problem: Make a few fixes to the code so that it compiles
// Solution:

trait Greeting {
    fn greet(&self) -> String {
        "Hello from Rust!".to_string()
    }
}

fn print_greeting1<T: Greeting>(input: &T) {
    println!("{}", input.greet());
}


fn print_greeting2(input: &impl Greeting) {
    println!("{}", input.greet());
}


fn print_greeting3<T>(input: &T)
where T: Greeting,
{
    println!("{}", input.greet());
}


struct Greeter;

impl Greeting for Greeter {}

pub fn solution_p1() {
    let greeter_instance = Greeter;

    print_greeting1(&greeter_instance);
    print_greeting2(&greeter_instance);
    print_greeting3(&greeter_instance);
}


// Problem 2: Fix the code by completing the function definition
// Solution:
pub trait VehicleHorn {
    fn horn_sound(&self) -> String {
        "peep peep".to_string()
    }
}

struct Car {}

struct Truck {}

impl VehicleHorn for Car {}
impl VehicleHorn for Truck {}

fn compare_horn_sound(vehicle_1: impl VehicleHorn, vehicle_2: impl VehicleHorn) -> bool {
    vehicle_1.horn_sound() == vehicle_2.horn_sound()
}

pub fn solution_p2() {
    let car = Car {};
    let truck = Truck {};
    assert_eq!(compare_horn_sound(car, truck), true);
}


// Problem 3: Complete the function signature of `get_square_root_str`.
// Solution:

trait SquareRoot {
    fn square_root(&self) -> Self;
}

trait Displayable {
    fn to_display_string(&self) -> String;
}

fn get_square_root_str(input: impl SquareRoot + Displayable) -> String {
    let squared_rooted = input.square_root();
    squared_rooted.to_display_string()
}

impl SquareRoot for f64 {
    fn square_root(&self) -> Self {
        self.sqrt()
    }
}

impl Displayable for f64 {
    fn to_display_string(&self) -> String {
        format!("{:.2}", self)
    }
}

pub fn solution_p3() {
    let num = 9.0;
    let mut msg = format!("{num} square rooted is ");
    msg.push_str(&get_square_root_str(num));
    println!("{msg}");
}