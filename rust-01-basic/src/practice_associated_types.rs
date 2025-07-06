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


// -----------------------------------------------------
// 		Choosing Associated vs Generic Types
// -----------------------------------------------------

trait Addition<Rhs, Output> {
    // type Rhs;
    // type Output;
    fn add(self, rhs: Rhs) -> Output;
}

struct Point {
    x: i32,
    y: i32,
}

impl Addition<Point, Point> for Point {
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Addition<i32, Point> for Point {
    fn add(self, rhs: i32) -> Point {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Addition<Point, Line> for Point {
    fn add(self, rhs: Point) -> Line {
        Line {
            start: self,
            end: rhs,
        }
    }
}

#[test]
fn test_3() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let p3: Point = p1.add(p2);

    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);

    let p1 = Point { x: 1, y: 1 };
    let p3 = p1.add(2);

    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);

    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let line: Line = p1.add(p2);

    assert!(line.start.x == 1 && line.start.y == 1 && line.end.x == 2 && line.end.y == 2);
}
