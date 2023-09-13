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
