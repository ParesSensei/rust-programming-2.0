// -------------------------------------------
//         - Combinators
// -------------------------------------------

#[test]
fn test() {
    let words = vec!["apple", "banana", "grape", "orange", "pear"];
    // let mut result: Vec<String> = vec![];

    // for word in words {
    //     if word.starts_with("a") || word.starts_with("b") {
    //         let uppercase_word = word.to_uppercase();
    //         result.push(uppercase_word);
    //     }
    // }
    // println!("Result: {:?}", result);

    let result: Vec<String> = words
        .into_iter()
        .filter(|&word| word.starts_with("a") || word.starts_with("b"))
        .map(|word| word.to_uppercase())
        .collect::<Vec<String>>();

    println!("Result: {:?}", result);
}


// -------------------------------------------
//          - Iterating Through Options
// -------------------------------------------

#[test]
fn test2() {
    // ------ Use case 1 -----

    let some_product = Some("laptop");
    let mut products = vec!["cellphone", "battery", "charger"];

    // Solution 1:
    // match some_product {
    //     Some(product) => products.push(product),
    //     _ => {}
    // };

    // Solution 2:
    // if let Some(product) = some_product {
    //     products.push(product);
    // }

    // Solution 3:
    products.extend(some_product);
    println!("{:?}", products);

    // ------- Use case 2 -----
    let mut products = vec!["cellphone", "battery", "charger"];
    let products_iter = products.iter().chain(some_product.iter());

    for prod in products_iter {
        println!("{:?} ", prod);
    }

    // ------ Use Case 3 -----
    let products = vec![Some("charger"), Some("battery"), None, Some("cellphone")];

    // Solution 1;
    // let mut prod_without_none = Vec::new();
    // for p in products {
    //     if p.is_some() {
    //         prod_without_none.push(p.unwrap());
    //     }
    // }

    // Solution 2:
    // let prod_without_none = products
    //     .into_iter()
    //     .filter(|x| x.is_some())
    //     .map(|x| x.unwrap())
    //     .collect::<Vec<&str>>();

    // Solution 3:
    let prod_wihtout_none: Vec<&str> = products.into_iter().flatten().collect();
    println!("{:?}", prod_wihtout_none);
}


