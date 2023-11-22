use std::io;

fn main() {

    println!("Fibonacci sequence quiz!");
    
    let mut a = 0; 
    let mut b = 1;
    
    for _ in 0..5 { //with the power of loops I write less code to achieve the same outcome
        println!("What is the next number after {:?}, {:?}?", a, b);   

        let mut guess = String::new(); // string saved in guess
        io::stdin().read_line(&mut guess).expect("Failed to read line"); //parsing in rust is so much fun

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Cmon, put a real number its fibonnaci mate. Using 0."); // gotta sound human to be a game
                0
            }
        };

        let next = a + b;
        if guess == next { //checker program to grade the user
            println!("Correct, I think you get the hang of this!");
        } else {
            println!("Incorrect. Dont use chatgpt :[ The next number is {}.", next);
        }

        a = b;
        b = next;
    }
    
    println!("I can tell you get the sequence. You have completed 5 more levels, goodjob myguy! Now go teach the next guy and star my repository");
}