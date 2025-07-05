// Problem 1: Fix the code so that it compiles.
// Solution:
trait Sound {
    fn animal_sound(&self) -> String {
        "I dont have sound!".to_string()
    }
}
struct Dog;
struct Cat;
struct Fish;

impl Sound for Dog {
    fn animal_sound(&self) -> String {
        "woof".to_string()
    }
}
impl Sound for Cat {
    fn animal_sound(&self) -> String {
        "meow".to_string()
    }
}
impl Sound for Fish {}

pub fn solution_p1() {
    let dog = Dog;
    let cat = Cat;
    let fish = Fish;
    println!("Dog Sound: {}", dog.animal_sound());
    println!("Cat Sound: {}", cat.animal_sound());
    println!("Fish Sound: {}", fish.animal_sound());
}


// Problem 2: Fix the code by adding the Vehicle trait for the types
// Solution:
trait Vehicle {
    fn speed(&self) -> f64 {
        0.0
    }
}

struct Car {
    model: String,
    speed: f64,
}

impl Vehicle for Car {
    fn speed(&self) -> f64 {
        self.speed
    }
}

struct Bicycle {
    brand: String,
}

impl Vehicle for Bicycle {}
struct Bus {
    model: String,
    speed: f64,
}

impl Vehicle for Bus {
    fn speed(&self) -> f64 {
        self.speed
    }
}
pub fn solution_p2() {
    let car = Car {
        model: "Camry".to_string(),
        speed: 120.0,
    };
    let bicycle = Bicycle {
        brand: "MountainBike".to_string(),
    };
    let bus = Bus {
        model: "BMC".to_string(),
        speed: 100.0,
    };

    car.speed();
    bicycle.speed();
    bus.speed();
}
