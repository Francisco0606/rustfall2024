fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret{
        return 0;
    }
    else if guess > secret {
        return 1;
    }
    else {
        return -1;
    }
}

fn main() {
    let secret: i32 = 66;
    let guess: i32 = 66;
    let mut guesses: i32 = 0;

    //loop
    loop {
        //increase attempt
        guesses += 1;
        //check guess
        let check = check_guess(guess, secret);

        if check == 0 {
            println!("Correct! {} was the right answer.", guess);
            break;
        }
        else if check == 1 {
            println!("{} is too high.", guess);
        }
        else if check == -1 {
            println!("{} is too low.", guess);
        }
    }

    println!("You guessed it in {} attempts", guesses);
}
// Create a simple number guessing game in Rust. The program should:

// Use a mutable variable to store a "secret" number (you can hard-code this).
// Implement a function check_guess(guess: i32, secret: i32) -> i32 that returns:
//   0 if the guess is correct
//   1 if the guess is too high
//  -1 if the guess is too low
// In the main function:
//  Use a loop to repeatedly:
//      Set a mutable guess variable to a number of your choice (simulating user input)
//      Call the check_guess function
//      Use an if-else expression to print whether the guess was correct, too high, or too low
//      If the guess was correct, break the loop
//  After the loop ends, print how many guesses it took (you'll need to track this in a variable)