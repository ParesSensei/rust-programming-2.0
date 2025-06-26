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



/*
-----------------------------------------------------
---------------------question no 2-------------------
-----------------------------------------------------
 */

// Problem 2:

/*
Write a program to find the sum of natural numbers below a given number N, where N is provided by the user.
The sum should only include numbers that are multiples of either 3 or 5.
For example, if the user enters N = 20, the multiples of 3 are 3, 6, 9, 12, 15, 18, and the multiples of 5 are 5, 10, and 15.

Please note that the value of 15 will be considered only once since it is a multiple of both 3 and 5.
The sum will be calculated as follows: 3 + 5 + 6 + 9 + 10 + 12 + 15 + 18.

Write a program that takes the user input N, performs the necessary calculations, and outputs the sum.
*/

// fn main() {
pub fn solution_p2() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");

    /* Add your code below this line */
    let mut sum = 0;
    for i in 1..n {
        if i % 3 ==0 {
            sum+= i;
            println!("{i}");
        } else if i % 5 == 0 {
            sum += i;
            println!("{i}");
        }
    }

    println!("final output is {}", sum);

}

// Note: You may require the % operator which is used for computing the remainder
// after dividing one number by another. It's commonly used to check for divisibility.
// Example: The statement 17 % 5 will result in a value of 2.



/*
-----------------------------------------------------
---------------------question no 3-------------------
-----------------------------------------------------
 */


// Problem 3:

/*
This question involves writing code to analyze the production of an assembly line in a car factory.
The assembly line has different speeds, ranging from 0 (off) to 10 (maximum).
At the lowest speed of 1, the assembly line produces a total of 221 cars per hour.
The production rate increases linearly with the speed,
meaning that a speed of 4 produces 4 * 221 = 884 cars per hour.

However, higher speeds increase the likelihood of producing faulty cars that need to be discarded.
The success rate depends on the speed, as shown in the table below:
· Speeds 1 to 4: 100% success rate.
· Speeds 5 to 8: 90% success rate.
· Speeds 9 and 10: 77% success rate.

You need to write two functions:
1. The first function, total_production(), calculates the total number of cars successfully produced without faults within a specified time given in hours. The function takes the number of hours and speed as input and returns the number of cars successfully produced.
2. The second function, cars_produced_per_minute(), calculates the number of cars successfully produced per minute. The function takes the number of hours and speed as input and returns the number of cars produced per minute.
Write the code for both functions based on the provided specifications.
*/


pub fn solution_p3() {
    println!("{}", total_production(6, 5) as i32); // to round the values we use i32. just ignore for mow
    println!("{}", cars_produced_per_minutes(6, 5) as i32); // to round the values we use i32. just ignore for mow
}

fn total_production(hours: u8, speed: u8) -> f32 {
    let success_rate: f32;

    /* Your code below this line*/
    if speed <= 4 {
        success_rate = 1.0;
    } else if speed >= 5 && speed <= 8 {
        success_rate = 0.9;
    } else {
        success_rate = 0.77;
    }

    hours as f32 * 221.0 * success_rate * speed as f32
}

fn cars_produced_per_minutes(hours: u8, speed: u8) -> f32 {
    let success_rate: f32;
    /* Your code below this line*/
    if speed <= 4 {
        success_rate = 1.0;
    } else if speed >= 5 && speed <= 8 {
        success_rate = 0.9;
    } else {
        success_rate = 0.77;
    }

    (hours as f32 * 221.0 * success_rate * speed as f32) / (60.0 * hours as f32)
}


/*
-----------------------------------------------------
---------------------question no 4-------------------
-----------------------------------------------------
 */

// Problem 4:

/*
A palindrome is a word, verse, or sentence that reads the same backward or forward,
such as 'Able was I ere I saw Elba,' or a number like 1881.

Write a function named is_palindrome() that checks whether a given string is a palindrome or not.
The function should take a string as input and return a boolean value indicating whether the string is a palindrome or not.
*/

pub fn solution_p4() {
    let input = String::from("apapapa");
    println!(
        "It is {:?} that the given string is palindrome",
        palindrome(input)
    );
}

fn palindrome(input: String) -> bool {
    /* Your Code here */
    let mut reverse_code: String = "".to_string();

    for i in input.chars(){
        reverse_code = format!("{}{}", i, reverse_code);
    }

    if input == reverse_code {
        true
    } else {
        false
    }
}



/*
-----------------------------------------------------
---------------------question no 6-------------------
-----------------------------------------------------
 */
// Problem 6: Write a function that implements the logic,
// 'You can see the movie if you are 17 or older, or if you are 13 or older and have a parent's permission.'
// Note: This means that if you 17 or older, you implicitly have permission.

pub fn solution_p6() {
    println!("John who is 14, can see the move: {}", can_see_movie(14, true));
}

fn can_see_movie(age: i32, permission: bool) -> bool {
    // Write your code here to implement the logic
    if age >= 17 {
        true
    } else if age >= 13{
        if permission == true{
            true
        } else {
            false
        }
    } else {
        false
    }

}