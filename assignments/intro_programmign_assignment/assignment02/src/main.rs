fn is_even(n: i32) -> bool {
    return n % 2 == 0;
}

fn main() {
    //initiate variables
    let mut sum = 0;
    let mut i = 0;
    let mut largest = 0;
    //initiate array
    let nums:[i32; 10] = [1, 2, 3, 4, 5, 6, 7, 9, 10, 15];
    
    //for loop fizzbuzz
    for num in nums.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        }
        else if num % 3 == 0 {
            println!("Fizz");
        }
        else if num % 5 == 0 {
            println!("Buzz");
        }
        else if is_even(*num) {
            println!("Even");
        }
        else {
            println!("Odd");
        }
    }
    
    //while loop
    while i < nums.len() {
        sum += nums[i];
        i += 1;
    }
    println!("Sum: {}", sum);
    
    //for loop
    for &num in nums.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest: {}", largest);
    
}
// Create a Rust program that analyzes a series of numbers. The program should:

// Create an array of 10 integer numbers of your choice.
// Implement a function is_even(n: i32) -> bool that returns true if a number is even, false otherwise.
// Use a for loop to iterate through the array and for each number:
//  Print whether it's even or odd using your is_even function
//  If the number is divisible by 3, print "Fizz" instead
//  If the number is divisible by 5, print "Buzz" instead
//  If it's divisible by both 3 and 5, print "FizzBuzz"
// Use a while loop to find and print the sum of all numbers in the array.
// Use a loop to find and print the largest number in the array.