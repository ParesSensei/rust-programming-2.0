// -------------------------------------------
// 			Generics
// -------------------------------------------

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}

impl Point<i32, i32> {
    fn printing(&self) {
        println!("The values of the coordinates are {}, {}", self.x, self.y);
    }

    fn new_1(x: i32, y: i32) -> Point<i32, i32> {
        Point { x, y }
    }
}

impl Point<f64, f64> {
    fn printing(&self) {
        println!("The values of the coordinates are {}, {}", self.x, self.y);
    }
}

fn add_points<T, U>(p1: &Point<T, U>, p2: &Point<T, U>) -> Point<T, U> {
    unimplemented!();
}

fn add_points_i32(p1: &Point<i32, i32>, p2: &Point<i32, i32>) -> Point<i32, i32> {
    unimplemented!();
}

fn add_points_f64(p1: &Point<f64, f64>, p2: &Point<f64, f64>) -> Point<f64, f64> {
    unimplemented!();
}

#[test]
fn test1() {
    let origin = Point::new(0, 0);
    let p1 = Point::new(1.0, 4.0);

    let p2 = Point::new(5, 5.0);

    origin.printing();
    // p1.printing();

    add_points(&origin, &origin); // add_points_i32(&origin, &origin);
    add_points(&p1, &p1); // add_points_f64(&p1, &p1);
}



// -------------------------------------------
// 			Traits
// -------------------------------------------

// struct drawing_info {
//     line_width: u8,
//     color: String,
// }
struct Square {
    side: f32,
    line_width: u8,
    color: String,
    //info: drawing_info,
}

struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
    // info: drawing_info,
}

// impl Square {
//     fn calculate_area(&self) {
//         println!("The area is: {}", self.side * self.side);
//     }
// }

// impl Rectangle {
//     fn area(&self) -> f32 {
//         self.length * self.width
//     }
// }

trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented, returning dummy value");
        0.0
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rect = self.length * self.width;
        println!("Rectangle area: {}", area_of_rect);
        area_of_rect
    }

    fn perimeter(&self) -> f32 {
        let perimeter_of_rect = 2.0 * (self.length + self.width);
        println!("Rectangle Perimeter: {}", perimeter_of_rect);
        perimeter_of_rect
    }
}

impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_square = self.side * self.side;
        println!("Square area: {}", area_of_square);
        area_of_square
    }
}

#[test]
fn test_trait() {
    let r1 = Rectangle {
        width: 5.0,
        length: 4.0,
        line_width: 1,
        color: String::from("Red"),
    };

    let s1 = Square {
        side: 3.2,
        line_width: 1,
        color: String::from("Red"),
    };

    r1.area();
    s1.area();

    r1.perimeter();
    s1.perimeter();
}


