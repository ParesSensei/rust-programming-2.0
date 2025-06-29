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
    let car_year = my_car.year;
    my_car.fuel_level = 30.0;

    let another_car = Car {
        owner: "new owner".to_string(),
        ..my_car
    };

    // Tuple struct
    let point_2d = (1, 2);
    let point_3d = (2, 3, 4);

    struct Point2d(i32, i32);
    struct Point3d(i32, i32, i32);

    let point1 = Point2d(1, 2);
    let point2 = Point3d(2, 3, 4);

    // unit struct
    struct ABC;
}

