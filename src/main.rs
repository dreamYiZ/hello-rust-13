#![allow(dead_code, unused_variables, unused_mut)]

use std::cmp::Ordering;
use std::io;

enum IpAddrKind {
    V4,
    V6,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {}
            Message::Move { x, y } => {}
            Message::Write(s) => {
                println!("write: {s}");
            }
            Message::ChangeColor(x, y, z) => {}
        }
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(u32, u32, u32);
struct Point(u32, u32, u32);

struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, world!");
    let mut a = 0;
    a += 1;
    println!("{}", a);
    add_1(&mut a);
    println!("{}", a);
    add_1(&mut a);
    println!("{}", a);
    add_1(&mut a);
    println!("{}", a);
    add_1(&mut a);
    println!("{}", a);
    add_1(&mut a);
    println!("{}", a);

    foo_1();
    foo_2();

    foo_i32(32);

    print_labeled_measurement(5, 'h');

    let y = 56;

    // let x  = (let z = 6);

    let mut guess = String::new();
    guess = String::from("66");

    println!("{guess}");

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("failed to read line");

    println!("You input is :{guess}");

    let secret_number = 66;

    let guess: i32 = guess.trim().parse().expect("please input a number ");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
        }
    }

    let mut x = 5;
    println!("value of x is: {x}");
    x = 7;
    println!("value of x is: {x}");

    let x = 5;
    let x = x + 1;
    {
        let x = x + 3;
        println!("the value of x in scope is: {x}");
    }

    println!("the value of x is: {x}");

    let mut space = "   ";
    let space_len = space.len();
    println!("space len {space_len}");

    let guess: u32 = "42".parse().expect("Not a number");

    let x = 2.0;
    let y = 3.0;
    let sum = 5 + 2;
    let difference = 96.6 - 3.42;
    let quotient = 55.5 / 22.2;
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'c';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("y in tup is: {y}");
    let tup_1 = tup.0;
    let tup_2 = tup.1;
    let tup_3 = tup.2;

    let a = [1, 2, 3, 4, 5, 6];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let a = [1, 2, 3, 4, 5];
    let a_1 = a[0];
    let a_2 = a[1];

    let a: [usize; 5] = core::array::from_fn(|i| i + 1);

    println!("{}", a[0]);
    println!("{}", a[1]);
    println!("{}", a[2]);

    let x = plus_one(5);
    println!("{x}");

    let lucky_number = 7;

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;

    let number = if condition { 5 } else { 6 };

    loop {
        println!("again");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result is: {}", result);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [1, 2, 3, 4, 5];

    let mut a_index = 0;
    while a_index < 5 {
        println!("value at index {} is {}", a_index, a[a_index]);
        a_index += 1;
    }

    for number in (1..4).rev() {
        println!("number {}", number);
    }

    let s = "hello";
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{s}");
    let x = 5;
    make_copy(x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("the length of {} is {}", s2, len);

    let s1 = String::from("Hello");

    let len = calculate_length_2(&s1);
    println!("len of {} is {}", s1, len);

    let mut s1 = String::from("hello");

    change(&mut s1);

    println!("s1 is {s1}");

    let mut s1 = String::from("hello");
    let r1 = &s1;
    let r2 = &s1;

    println!("r1 {r1}, r2 {r2}");
    let r3 = &mut s1;

    println!("r3 {r3}");

    let s = String::from("hello world!");
    let word = first_word(&s);

    println!("word: {word}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user@gmail.com"),
        sign_in_count: 1u64,
    };

    let user2 = build_user(String::from("user2@gmail.com"), String::from("user2"));
    let user3 = build_user(String::from("user3@gmail.com"), String::from("user3"));
    let user4 = User {
        email: String::from("user4@gmail.com"),
        ..user3
    };

    let black = Color(0, 0, 0);
    let origin = Color(0, 0, 0);

    let subject = AlwaysEqual;

    let width1 = 30;
    let height1 = 50;
    println!(
        "the area of rectangle is {} square pixels",
        area(width1, height1)
    );

    let rect1 = Rectangle {
        width: 100,
        height: 200,
    };
    let area_rect1 = area3(&rect1);

    println!("rect1 is {:?}", &rect1);

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let x = 5;
    let y = Some(5);
    let sum = x + y.unwrap();
    println!("sum: {sum}");

    let c = Coin::Penny;

    let v_of_c = value_in_cents(c);
    println!("v of c is: {v_of_c}");

    let c2 = Coin::Penny;
    value_in_cents_2(c2);

    let five = Some(5);
    let six = plus_one_2(five);
    let none = plus_one_2(None);

    let a = 9;

    match a {
        5 => {}
        7 => {}
        other => {
            println!("a is {a}");
        }
    }

    let mut arr = vec![1, 2, 3, 4];

    for i in 0..4 {
        println!("{}", arr[i]);
        arr.push(i);
    }

    // for i in 0..4{
    //     println!("{}",arr[i]);
    //     // arr.push(i);
    // }

    println!("{:?}, len: {}", arr, arr.len());

    let a = 6;
    let mut b: u32 = 100;
    let c = compute(&a, &mut b);

    println!("c is: {:?}", c);
    println!("b is: {:?}", b);
    println!("b is: {}", b);


    let config_max = Some(3u8);
    match config_max{
        Some(max)=>{
            println!("the maximum is configured to be {}", max);
        },
        _=>(),
    }

    let mut count = 0;
    // match  coin {
        
    // }


}

fn add_1(x: &mut i32) {
    *x += 1;
}

fn foo_1() {
    println!("fn foo 1");
}

fn foo_2() {
    println!("fn foo 2");
}

fn foo_i32(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(v: i32, u_label: char) {
    println!("The measurement is: {v}{u_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn takes_ownership(s: String) {
    println!("string: {}", s);
}

fn make_copy(x: i32) {
    println!("x: {x}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_2(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world.");
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn route(ip_kind: IpAddrKind) {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 15,
        Coin::Quarter => 25,
    }
}

fn value_in_cents_2(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("value in cents 2");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 15,
        Coin::Quarter => 25,
    }
}

fn plus_one_2(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn as_str(n: &u32) -> String {
    let s = format!("{}", n);
    s
}

fn compute(input: &u32, output: &mut u32) {
    if *input > 10 {
        *output = 1;
    }
    if *input > 5 {
        *output *= 2;
    }
}

fn compute_2(input: &u32, output: &mut u32) {
    let cached_input = *input;
    if cached_input > 10 {
        *output = 2;
    } else if cached_input > 5 {
        *output *= 2;
    }
}

fn compute_3(input: &u32, output: &mut u32) {
    let mut temp = *output;
    if *input > 10 {
        temp = 1;
    }

    if *input > 5 {
        temp *= 2;
    }

    *output = temp;
}
