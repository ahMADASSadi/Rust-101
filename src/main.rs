// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

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
    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("{s2}");
    // println!("{s1}");

    // let mut s = String::from("hello");
    // println!("{s}, world!");

    // s = String::from("ahoy");

    // println!("{s}, world!");

    // fn option(s:&String) -> (&String,usize){
    //     let len:usize = s.len();
    //     (s,len)
    // }
    // let s1 = String::from("some text here");

    // let (s2,len) = option(&s1);
    // println!("{s1}");

    // println!("{s2},{len}");

    // fn dangle() ->String{
    //     let s = String::from("hello");
    //     s
    // }

    // let some_thing = dangle();

    // let pointer = &some_thing;

    // println!("{some_thing}");
    // println!("{pointer}");

    // let s = String::from("hello world");

    // let mut hello = &s[0..5];
    // let mut world = &s[6..11];

    // let hello = String::from(&s[0..5]);
    // let world = String::from(&s[6..11]);

    // let hello = (s[0..5]).to_string();
    // let world = (s[6..11]).to_string();

    // print!("{hello}");
    // print!("{world}");
    // println!();
    // fn first_word(s: &str) -> &str {
    //     let bytes = s.as_bytes();

    //     for (i, &item) in bytes.iter().enumerate() {
    //         if item == b' ' {
    //             return &s[0..i];
    //         }
    //     }
    //     &s[..]
    // }
    // let mut s = String::from("hello world");

    // let word = first_word(&s);

    // println!("the first word is: {word}");

    // let my_string = String::from("hello world");

    // // `first_word` works on slices of `String`s, whether partial or whole
    // let word = first_word(&my_string[0..6]);
    // println!("the first word is: {word}");
    // let word = first_word(&my_string[..]);
    // println!("the first word is: {word}");
    // // `first_word` also works on references to `String`s, which are equivalent
    // // to whole slices of `String`s
    // let word = first_word(&my_string);
    // println!("the first word is: {word}");
    // let my_string_literal = "hello meow";
    // // `first_word` works on slices of string literals, whether partial or whole
    // let word = first_word(&my_string_literal[6..]);
    // println!("the first word is: {word}");
    // let word = first_word(&my_string_literal[..]);
    // println!("the first word is: {word}");

    // // Because string literals *are* string slices already,
    // // this works too, without the slice syntax!
    // let word = first_word(my_string_literal);
    // println!("the first word is: {word}");

    // let a = [1, 2, 3, 4, 5];

    // let slice = &a[1..3];
    // print!("[");
    // for i in 0..a.len(){
    //     print!("{},", a[i]);
    // }
    // println!("]");

    // print!("[");
    // for i in 0..slice.len(){
    //     print!("{},", slice[i]);
    // }
    // println!("]");

    // if slice == &[2, 3] {
    //     println!("slice is equal to [2, 3]");
    // };

    // struct User {
    //     username: String,
    //     email: String,
    //     sign_in_count: u64,
    //     active: bool,
    // }

    // let mut user=User{
    //     username:String::from("madassandd"),
    //     email:String::from("madassand@gmail.com"),
    //     sign_in_count:1,
    //     active:true,
    // };

    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user
    // };

    // println!("{}",user2.sign_in_count);

    // user.email= String::from("madassandd@gmail.com");
    // println!("{}",user.email);

    // struct Game(String,i32);

    // let game = Game(String::from("Game of thrones"), 8);

    // println!("{}",game.0);

    // fn calculate_area(circle: &Circle) -> f64 {
    //     3.14159 * circle.radius * circle.radius
    // }
    // struct Circle{
    //     radius:f64,
    // }

    // let circle = Circle{radius:5.0};
    // println!("Area of circle is :{:?}",calculate_area(&circle));

    // // this line will work with the above code
    // println!("{}",circle.radius);

    // fn calculate_area(circle: Circle) -> f64 {
    //     3.14159 * circle.radius * circle.radius
    // }
    // struct Circle{
    //     radius:f64,
    // }

    // let circle = Circle{radius:5.0};
    // println!("Area of circle is :{:?}",calculate_area(circle));

    // // this line will not work with the above code
    // println!("{}",circle.radius);

    // #[derive(Debug)]
    // struct Circle {
    //     radius: f64,
    // }

    // let circle = Circle { radius: 5.0 };

    // println!("{circle:#?}");
    // dbg!(&circle);
    // println!("{circle:#?}")
    // #[derive(Debug)]
    // struct Circle { radius: f64}

    // impl Circle{
    //     fn area(&self) -> f64{
    //         3.14159 * self.radius
    //     }
    // }

    // let circle = Circle { radius: 5.0 };

    // dbg!(circle.area());
    // println!("{}",circle.area());

    #[derive(Debug)]
    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Rectangle {
        fn square(size: f64) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
        fn area(&self) -> f64 {
            self.width * self.height
        }
        fn can_hold(&self,other: &Rectangle) -> bool {
            self.width >= other.width && self.height >= other.height
        }
    }


    let square:Rectangle = Rectangle::square(10.0);

    let rect1:Rectangle = Rectangle{width:20.0, height:10.0};

    dbg!(&square);
    // println!("{square:#?}");
    println!("{}",square.area());

    println!("{}",rect1.area());

    println!("{}",square.can_hold(&rect1));
}
