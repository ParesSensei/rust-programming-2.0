// Problem 1: Add the correct types for the associated type 'item' in the implemenation blocks
// Solution:

trait Container {
    type Item;

    fn add_item(&mut self, item: Self::Item);
    fn get_item(&self) -> Option<&Self::Item>;
}

struct VecContainer_i32 {
    items: Vec<i32>,
}

impl Container for VecContainer_i32 {
    type Item = i32;

    fn add_item(&mut self, item: Self::Item) {
        self.items.push(item);
    }

    fn get_item(&self) -> Option<&i32> {
        self.items.last()
    }
}

struct OptionContainer<T> {
    item: Option<T>,
}

impl<T> Container for OptionContainer<T> {
    type Item = T;

    fn add_item(&mut self, item: T) {
        self.item = Some(item);
    }

    fn get_item(&self) -> Option<&T> {
        self.item.as_ref()
    }
}

pub fn solution_p1() {
    let mut vec_container = VecContainer_i32 { items: Vec::new() };
    vec_container.add_item(42);
    vec_container.add_item(123);

    if let Some(last_item) = vec_container.get_item() {
        println!("Last item in VecContainer: {}", last_item);
    } else {
        println!("VecContainer is empty");
    }

    let mut option_container = OptionContainer { item: None };
    option_container.add_item("Hello, Rust!");

    if let Some(only_item) = option_container.get_item() {
        println!("Only item in OptionContainer: {}", only_item);
    } else {
        println!("OptionContainer is empty");
    }
}


// Problem 1: Add the associated types in the implementation blocks
// Solution:

#[derive(Debug)]
struct CarEngine {
    model: String,
    horsepower: u32,
}
#[derive(Debug)]
struct GasolineCar {}

#[derive(Debug)]
struct ElectricEngine {
    model: String,
    power: u32,
}

#[derive(Debug)]
struct ElectricCar {}

trait Vehicle {
    type EngineType;
    fn get_engine(&self) -> Self::EngineType;
}

impl Vehicle for GasolineCar {
    type EngineType = CarEngine;
    fn get_engine(&self) -> Self::EngineType {
        CarEngine {
            model: "V8".to_string(),
            horsepower: 400,
        }
    }
}

impl Vehicle for ElectricCar {
    type EngineType = ElectricEngine;
    fn get_engine(&self) -> Self::EngineType {
        ElectricEngine {
            model: "Electric Motor".to_string(),
            power: 300,
        }
    }
}

pub fn solution_p2() {
    let gasoline_car = GasolineCar {};
    let electric_car = ElectricCar {};

    println!("Gasoline Car engine: {:?}", gasoline_car.get_engine());
    println!("Electric Car engine: {:?}", electric_car.get_engine());
}
