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

    // #[derive(Debug)]
    // struct Rectangle {
    //     width: f64,
    //     height: f64,
    // }

    // impl Rectangle {
    //     fn square(size: f64) -> Self {
    //         Self {
    //             width: size,
    //             height: size,
    //         }
    //     }
    //     fn area(&self) -> f64 {
    //         self.width * self.height
    //     }
    //     fn can_hold(&self,other: &Rectangle) -> bool {
    //         self.width >= other.width && self.height >= other.height
    //     }
    // }

    // let square:Rectangle = Rectangle::square(10.0);

    // let rect1:Rectangle = Rectangle{width:20.0, height:10.0};

    // dbg!(&square);
    // // println!("{square:#?}");
    // println!("{}",square.area());

    // println!("{}",rect1.area());

    // println!("{}",square.can_hold(&rect1));

    // #[derive(Debug)]
    // struct Entity {
    //     kind: String,
    //     health: i32,
    // }

    // impl Entity {
    //     fn get_kind(&self) -> &str {
    //         &self.kind
    //     }

    //     fn in_place(k:String,h:i32) -> Self {
    //         Self {
    //             kind:String::from(k),
    //             health: h,

    //         }
    //     }
    // }

    // let entity = Entity {
    //     kind: String::from("player"),
    //     health: 100,
    // };

    // println!("{entity:#?}");
    // dbg!(entity.get_kind());
    // dbg!(entity);

    // let entity1:Entity = Entity::in_place(String::from("enemy"), 50);
    // dbg!(entity1);

    // #[derive(Debug)]
    // enum IpAddrKind {
    //     V4(u8,u8,u8,u8),
    //     V6(String),
    // }

    // let home = IpAddrKind::V4(127,0,0,1);

    // dbg!(&home);
    // println!("{home:?}");

    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(u8, u8, u8),
    // }

    // impl Message {
    //     fn call(&self) {
    //         match self {
    //             Message::Quit => println!("Quit"),
    //             Message::Move { x, y } => println!("Move to x:{x}, y:{y}"),
    //             Message::Write(text) => println!("Write {text}"),
    //             Message::ChangeColor(r, g, b) => println!("Change color to r:{r}, g:{g}, b:{b}"),
    //         }
    //     }
    // }

    // let message = Message::ChangeColor(5,3,1);
    // message.call();

    // let x: i8 = 5;
    // // let y: Option<i8> = Some(5);
    // let y :Option<i8> = Some(9);
    // let sum:Option<i8> = match y {
    //     Some(i)=> Some(i+x),
    //     None => None,
    // };

    // let z: Option<i16> = None;
    // let sum1:Option<i16>= match z{
    //     Some(i) => Some(i as i16 + x as i16),
    //     other => None,
    // };

    // dbg!(sum);
    // println!("{sum:#?}");


    // dbg!(sum1);
    // println!("{sum1:#?}");

    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _ => (),
    // }

    
    // let mut v :Vec<String> = Vec::new();

    // v.push(String::from("hello"));
    // v.push(String::from(" "));
    // v.push(String::from("world!\n"));

    // for s in &v {
    //     print!("{s}");
    // }

    // let element:Option<&String> = v.get(2);
    // match element{
    //     Some(element) => println!("{}", element),
    //     None => println!("No element found"),
    // };

    // dbg!(element);
    // let sum = math::add(3, 7);
    // println!("Sum: {}", sum);
    // let anothersum = add(3,5);
    // println!("Another sum: {}", anothersum);
    // // let squared = math::advanced::square(4);
    // // println!("Square: {}", squared);
    // let anothersquared = square(4);
    // println!("Another Square: {}", anothersquared);

    // let mut result= math::Point{vector:vec![]};
    // result.vector.push(4);
    // dbg!(&result);
    // let poper =result.vector.pop();
    // match poper{
    //     Some(p) => println!("Popped something else{}",p),
    //     Some(_) => println!("another thing"),
    //     None => println!("Nothing")
    // }
    // dbg!(poper);
    // for i in &result.vector {
    //     dbg!(i);
    // }
    // let poper =result.vector.pop();
    // match poper{
    //     Some(p) => println!("Popped something else{}",p),
    //     Some(_) => println!("another thing"),
    //     None => println!("Nothing")
    // }
    // dbg!(&result);

    // let v = vec![1, 2, 3, 4, 5];
    // let mut exist:&[i32]=&[];
    // if v.len() > 100 {
    //     println!("The 100th element is: {:?}",v[100]);
    //     let does_not_exist = &v[100];
    //     let does_not_exist = v.get(100);
    // }else{
    //     exist = &v[..];
    // }
    // dbg!(exist);
    // let exist: &[i32] = if v.len() > 100 {
    //     &[]  // Assign an empty slice if `v.len() > 100`
    // } else {
    //     dbg!(&v);
    //     let (_, last_two) = v.split_at(v.len() - 2);
    //     println!("Last two: {:?}", last_two);
    //     last_two
    //     // &v[2..]
    // };
    // dbg!(exist);
    // let mut v = vec![1,2,3,4,5,6,7];
    // let &first = &v[0];
    // let &second = &v[1];
    // dbg!(first);
    // v.push(8);
    // dbg!(second);
    // dbg!(v);

    // let mut s = "some shit";
    // let mut s1 = String::from("foo");
    // let mut s2 = String::from("bar");
    // s1.push_str(&(dbg!(" ".to_string())+&s2));
    // println!("s1 is {:?}",s1);
    // s2.push_str(&s2.clone());
    // println!("Updated s2: {:?}", s2);

    // use std::any::Any;

    // fn print_type_of<T: Any>(val: T) {
    //     println!("The type of the value is: {}", type_name::<T>());
    // }

    // print_type_of(s);


    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1.clone() + &s2;
    // println!("{}",&s1[..]);
    // print_type_of(s1.clone());
    // println!("{}",&s2[..]);
    // print_type_of(s2.clone());
    // println!("{}",&s3[..]);
    // print_type_of(s3.clone());

    // let s = format!("{s1}{s2}{s3}\n");
    // print!("{s}");

    // let s = "Здравствуйте";
    // let first = s.chars().nth(0);
    // println!("{:?}",first);

    // let s = String::from("Здравствуйте");
    // let first = & s[..2];
    // println!("{:?}",first);
    // for c in s.chars() {
    //     println!("\"{}\"", c);  // This prints each character surrounded by double quotes
    // }
    use std::collections::HashMap;

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name.clone(), field_value.clone());
    // map.insert(&field_name, &field_value);
    
    // println!("{map:#?}");
    // println!("{field_value:#?}");
    // println!("{field_name:#?}");
    // println!("{:#?}",&field_value);
    // println!("{:#?}",&field_name);

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // println!("{scores:#?}");

    // // debug_assert_ne!(scores.get("Blue"), Some(&10),"ksksks");
    
    // // assert_eq!(scores.get("Blue"), Some(&10));
    
    // assert!(matches!(scores.entry(String::from("Yellow")) , std::collections::hash_map::Entry::Vacant(_)));
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);
    
    // println!("{scores:#?}");
    // let text = "hello world wonderful world";

    // let mut map = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }

    // println!("{map:?}");
}
// use std::any::type_name;
// mod math;
// use math::add;
// use math::advanced::square;