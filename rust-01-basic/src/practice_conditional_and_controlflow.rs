// Problem 1:
/*
Write a program to find the difference between the square of the sum and the sum of the squares of the first N numbers.
N will be a user-defined input that your program will take.

For example, if the user enters the number 5, you should compute the square of the sum as (1 + 2 + 3 + 4 + 5)^2 = 225.
Next, compute the sum of the squares as (1^2 + 2^2 + 3^2 + 4^2 + 5^2) = (1 + 4 + 9 + 16 + 25) = 55.
Finally, calculate the difference as 225 - 55 = 170.
*/

pub fn solution_p1() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");

    let mut square_of_sum = 0;
    let mut sum_of_squares = 0;

    /* Complete the code after this line */
    for angka in 1..n+1{
        square_of_sum += angka
    };

    let square = square_of_sum.pow(2);

    for angka in 1..n+1 {
        sum_of_squares += angka.pow(2)
    };

    let sum = sum_of_squares;

    let selisih = &square - &sum;

    println!("jumlah angka sekarang + di kuatradtin {}", square);
    println!("jumlah angka sekarang langsung kuatrat {}", sum);
    println!("selesih {}", selisih);
}

// Note: The pow() method in Rust is used to raise a number to an integer power.
// Example: If we want to compute 2 raised to the power 3, then the syntax
// will be 2.pow(3).
// If we want to compute a variable raise to the power of some value,
// then the following syntax can be used.
// let number: u32 = 5;
// let square = number.pow(2); // This computes square of the variable number
