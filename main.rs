use std::io;

fn main() {
    println!("Fibonacci sequence quiz!");
    
    let mut a = 0;
    let mut b = 1;
    
    println!("What is the next number in the sequence after 0, 1?");
    
    let mut guess = String::new();//creates a string to hold input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using default value 0");
            0
        }
    };
    
    if guess == a + b { //checks for correctness and provides an output.
        println!("Correct! Good job my guy");
    } else {
        println!("Incorrect, you already almost out of tries sadly. The next number is {}", a + b);
    }
    
    a = b;// Update sequence variables
    b = a + b;
    
    println!("What is the next number in the sequence after 0, 1, 1? "); //I made this question confusing fr
    
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess: u32 = match guess.trim().parse() {
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
}