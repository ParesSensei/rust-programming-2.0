//-------------------------------------------------------
//                  -Borrowing
//-------------------------------------------------------
/*
- Borrowing rules
    - At any time, you can have either one mutable reference or any number of immutable references.
    -  References must always be valid.

- Solve out two problems
    - Data race
    - Dangling  references

 & -> keyword reference
 */

fn test1() {
    let mut vec_1 = vec![1,2,3,4];
    let ref_1 = &vec_1;
    let ref_2 = &vec_1;
    println!("ref1: {:?}, ref2: {:?}", ref_1, ref_2);
    let ref3 = &mut vec_1;
}

//-------------------------------------------------------
//                  -Borrowing in function
//-------------------------------------------------------
/*
- Borrowing rules
    - At any time, you can have either one mutable reference or any number of immutable references.
    - References must always be valid.
 */

#[test]
fn test2() {
    let mut vec1 = vec![1,2,3,4];
    let ref1 = &vec1;
    borrows_vec(ref1);
    let ref2 = &mut vec1;
    mutably_borrows_vec(ref2);
    println!("vec1 is {:?}", vec1);
}

fn borrows_vec(vec: &Vec<i32>) {
    println!("vec is {:?}", vec);
}

fn mutably_borrows_vec(vec: &mut Vec<i32>) {
    vec.push(10);
}