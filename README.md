# Fibonnacigame
Fibonacci Sequence Quiz
Description
This is a simple interactive program that quizzes the user on the Fibonacci sequence. The Fibonacci sequence is a series of numbers in which each number is the sum of the two preceding ones, usually starting with 0 and 1. The program prompts the user to identify the next number in the sequence based on the given starting numbers.

Usage
Clone the repository or download the source code file.
Make sure you have Rust installed on your system.
Open a terminal and navigate to the directory containing the program.
Compile the program using the command: cargo run

The read_line method reads a line from the standard input and appends it to the provided string (&mut guess). The expect method is used to handle any potential errors during the input reading process. 
Code: io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

Converting the input string was particularly hard but found a simple function
Convert the input string (guess) to an unsigned 32-bit integer (u32):
Code: let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Invalid input. Using default value 0");
        0
    }
    };
trim() is used to remove leading and trailing whitespaces from the input.
parse() attempts to parse the trimmed string into an integer.
The match expression is used to handle the result of the parsing operation. If parsing is successful (Ok(num)), it returns the parsed number. If there is an error (Err(_)), it prints an error message, sets the value to 0, and continues.



Levels Game Details 
Level 1
Question: What is the next number in the sequence after 0, 1?
Code: if guess == a + b { //checks for correctness and provides an output.
        println!("Correct! Good job my guy");
    } else {
        println!("Incorrect, you already almost out of tries sadly. The next number is {}", a + b);
    }
    
    a = b;// Update sequence variables
    b = a + b;

Level 2
Question: What is the next number in the sequence after 0, 1, 1?
Code: let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input but try again. Using default 0");
            0
        }
    };
    
    if guess == a + b { //checks for correctness and provides and output
        println!("Correct!");
    } else {
        println!("Incorrect you almost out of tries. The next number is {}.", a + b);
    }


Notes
The program uses the Fibonacci sequence logic where each number is the sum of the two preceding ones.
User input is validated to ensure it is a valid unsigned 32-bit integer. If the input is invalid, a default value of 0 is used.
After completing each level, the program updates the sequence variables (a and b) to progress to the next level.