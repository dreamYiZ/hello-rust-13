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

    
}

fn add_1(x: &mut i32) {
    *x += 1;
}
