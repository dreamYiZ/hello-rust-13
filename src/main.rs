#![allow(dead_code, unused_variables, unused_mut)]

use std::cmp::Ordering;
use std::io;

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

    let (s2,len) = calculate_length(s1);

    println!("the length of {} is {}", s2, len);


    let s1 = String::from("Hello");

    let len = calculate_length_2(&s1);
    println!("len of {} is {}", s1, len);

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

fn calculate_length_2 (s: &String)->usize{
    s.len()
}
