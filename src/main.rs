use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    
    let random_number = rand::rng().random_range(1..=10);
    
    loop {
        let mut guess = String::new();
        println!("Enter you guess!");
        println!("secret number is :{}", random_number);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
            
        };

        println!("your guess is :{}", guess);


        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!("Done!");
                break;
            },
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!")
        };
    };
}
