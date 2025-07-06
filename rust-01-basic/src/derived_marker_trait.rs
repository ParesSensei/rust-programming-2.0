// -------------------------------------------
// 			Derived Traits
// 			Marker Traits
// -------------------------------------------

trait Properties: PartialEq + Default + Clone {}
#[derive(Debug, PartialEq, Default, Clone)]
struct Student {
    name: String,
    age: u8,
    sex: char,
}
impl Properties for Student {}

#[test]
fn test_student() {
    let s_1 = Student {
        name: String::from("ABC"),
        age: 35,
        sex: 'M',
    };

    let s_2 = Student {
        name: String::from("XYZ"),
        age: 40,
        sex: 'M',
    };

    println!("Student: {:?}", s_1);
    println!("s_1 and s_2 are equal: {}", s_1 == s_2);
}


// ----------------------------------------------
// 		Associated Types in Traits
// ----------------------------------------------

#[derive(Debug)]
struct Km {
    value: u32,
}

#[derive(Debug)]
struct Kmh {
    value: u32,
}

#[derive(Debug)]
struct Miles {
    value: u32,
}

#[derive(Debug)]
struct Mph {
    value: u32,
}

// impl Kmh {
//     fn distance_in_three_hours(&self) -> Km {
//         Km {
//             value: self.value * 3,
//         }
//     }
// }

// impl Mph {
//     fn distance_in_three_hours(&self) -> Miles {
//         Miles {
//             value: self.value * 3,
//         }
//     }
// }

trait DistanceThreeHours {
    type Distance;
    fn distance_in_three_hours(&self) -> Self::Distance;
}

impl DistanceThreeHours for Kmh {
    type Distance = Km;
    fn distance_in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}

impl DistanceThreeHours for Mph {
    type Distance = Miles;
    fn distance_in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}

#[test]
fn test_2() {
    let speed_Kmh = Kmh { value: 90 };
    let distance_Km = speed_Kmh.distance_in_three_hours();

    println!(
        "At {:?}, you will travel {:?} in 3 hours",
        speed_Kmh, distance_Km
    );

    let speed_Mph = Mph { value: 90 };
    let distance_Miles = speed_Mph.distance_in_three_hours();
    println!(
        "At {:?}, you will travel {:?}, in 3 hours",
        speed_Mph, distance_Miles
    );
}
