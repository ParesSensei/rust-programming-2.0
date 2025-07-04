// Problem 1: Fix the code in the main.


enum Measurement {
    CircleArea(f64),
    RectangleArea(f64, f64),
    TriangleArea(f64, f64),
    Perimeter(Vec<f64>),
}

impl Measurement {
    fn calculate(self) -> Result<f64, String> {
        match self {
            Self::CircleArea(radius) => {
                if radius < 0.0 {
                    Err(String::from("Radius cannot be negative"))
                } else {
                    Ok(std::f64::consts::PI * radius * radius)
                }
            }
            Self::RectangleArea(length, width) => {
                if length < 0.0 || width < 0.0 {
                    Err(String::from("Length and width cannot be negative"))
                } else {
                    Ok(length * width)
                }
            }
            Self::TriangleArea(base, height) => {
                if base < 0.0 || height < 0.0 {
                    Err(String::from("Base and height cannot be negative"))
                } else {
                    Ok(0.5 * base * height)
                }
            }
            Self::Perimeter(sides) => {
                if sides.len() < 3 {
                    Err(String::from("A polygon must have at least 3 sides"))
                } else {
                    Ok(sides.iter().sum())
                }
            }
        }
    }
}

pub fn solution_p1() {
    let user_input = Measurement::TriangleArea(5.0, 8.0);
    match user_input.calculate() {
        Ok(res)=> println!("Result: {res}"),
        Err(e)=> println!("Error: {e}"),
    }
}


// Problem 2: Complete the function signature.

pub fn solution_p2() {
    let number = 7;
    if let Err(e) = calculate_square(number) {
        println!("Error: {e}");
    }
}


fn calculate_square(num: i32) -> Result<i32, String> {
    if num >= 0 {
        let result = num * num;
        println!("The square of {} is: {}", num, result);
        Ok(result)
    } else {
        Err("Negative number provided".to_string())
    }
}
