use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // let random_number = rand::rng().random_range(1..=10);

    // loop {
    //     let mut guess = String::new();
    //     println!("Enter you guess!");
    //     println!("secret number is :{}", random_number);

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = match guess.trim().parse(){
    //         Ok(num) => num,
    //         Err(_) => continue,

    //     };

    //     println!("your guess is :{}", guess);

    //     match guess.cmp(&random_number) {
    //         Ordering::Equal => {
    //             println!("Done!");
    //             break;
    //         },
    //         Ordering::Less => println!("Too low!"),
    //         Ordering::Greater => println!("Too high!")
    //     };
    // };
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");

    // let mut choice = String::new();
    // let mut temprature = String::new();

    // loop {
    //     choice.clear();
    //     println!("enter choice, either F or C");
    //     io::stdin().read_line(&mut choice).expect("Failed to read line");
    //     let choice = choice.trim(); // Trim whitespace

    //     if choice != "F" && choice != "C" {
    //         println!("Invalid choice. Please enter F or C.");
    //         continue; // Ask again instead of breaking
    //     }
    //     temprature.clear();

    //     if choice == "C" {println!("Enter temperature in Celsius")} else {println!("Enter temperature in Fahrenheit")};
    //     io::stdin().read_line(&mut temprature).expect("Failed to read line");
    //     let temprature: f32 = match temprature.trim().parse(){
    //         Ok(num) => num,
    //         Err(_) => {
    //             println!("Invalid number. Please enter a valid temperature.");
    //             continue; // Ask again instead of breaking
    //         }
    //     };
    //     if choice == "C" {
    //         println!("Temperature in Fahrenheit is :{:.2}", temprature * 1.8 + 32.0);
    //     } else {
    //         println!("Temperature in Celsius is :{:.2}", (temprature - 32.0) / 1.8);
    //     }
    // }

    // fn fibonacci(n: i64) -> Vec<i128> {
    //     let mut series = vec![0, 1];
    //     for i in 2..n as usize { // Convert n to usize
    //         series.push(series[i - 1] + series[i - 2]);
    //     }
    //     series
    // }

    // let mut n = String::new();
    // loop {
    //     n.clear();
    //     io::stdin().read_line(&mut n).expect("Failed to read line");
    //     let n: i64 = match n.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => {
    //             println!("Invalid number. Please enter a valid number.");
    //             return;
    //         }
    //     };
    //     let fibonacci_series = fibonacci(n);

    //     println!("Fibonacci series is :{:?}", fibonacci_series);
    // }
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{s2}");
    println!("{s1}");

    let mut s = String::from("hello");
    println!("{s}, world!");
    
    s = String::from("ahoy");

    println!("{s}, world!");

}
