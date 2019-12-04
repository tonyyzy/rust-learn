fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 10;

    let y = y + 1;

    let y = y * 2;
    println!("y is: {}. Shadowing is different from mutating", y);

    let space = "    ";
    let space = space.len();
    println!("space has {} spaces", space);

    let guess: u32 = "42".parse().expect("Not a number!");

    let tup: (i32, f64, u8) = (500, 2.3, 3);

    let (a, b, c) = tup;
    println!("The value of a, b, c is {} {} {}", tup.0, b, tup.2);
}
