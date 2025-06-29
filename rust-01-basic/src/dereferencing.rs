//-------------------------------------------------------
//                  -Ownership in functions
//-------------------------------------------------------

#[test]
fn test1() {
    #[allow(unused_variables)]
    let mut some_data = 42;
    let ref_1 =  &mut some_data;
    let deref_copy = *ref_1;
    *ref_1 = 13;
    println!("some data is {}. deref_copy is {}", some_data, deref_copy);

    let mut heap_data = vec![1, 2, 3];
    let ref_1 = &heap_data;
    let _ref_2 = ref_1;
    let _ref3 = ref_1;
    let _deref_copy = ref_1.clone();

    let move_out = ref_1;
    // let move_out_again = ref_1;
}