// -------------------------------------------
//         Box Smart Pointer (Part 1)
// -------------------------------------------

//       Simple Pointer          ||         Smart Pointers
// ----------------------------------------------------------------------
// Just stores memory address    ||   Special capabilities
// Indicated by &                ||   Not just simple references
// Also called references        ||
// No special capabilities       ||

/*
enum Conveyance {
    Car(i32),
    Train(i32),
    Air(i32),
    Walk
}
*/

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[test]
fn test_1() {
    let x = 0.625;
    let y = Box::new(x);
    let z = &x;

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    println!("{:?}", list);
}


// -------------------------------------------
//         Box Smart Pointer (Part 2)
// -------------------------------------------

// Example 1
/*
#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
}

//#[test]
fn test_5() {
    let list = List::Cons(
        1,
        Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3, None)))))),
    );

    println!("{:?}", list);
}
*/

// Example 2
struct Huge_Data;
struct Small_Data;

trait Storage {}

impl Storage for Huge_Data {}
impl Storage for Small_Data {}

#[test]
fn test_2() {
    let data_1 = Huge_Data;
    let data_2 = Box::new(Huge_Data);

    let data_3 = data_1;
    let data_4 = data_2;

    let data_5 = Box::new(Small_Data);

    let data: Vec<Box<dyn Storage>> = vec![Box::new(data_3), data_4, data_5];
}
