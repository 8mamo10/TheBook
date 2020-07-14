fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_VALUE: u32 = 100_000;
    println!("The max value is: {}", MAX_VALUE);

    let y = 5;
    println!("The value of y is: {}", y);
    let y = y + 1;
    println!("The value of y is: {}", y);

    // good
    let _spaces = "      ";
    let _spaces = _spaces.len();
    // bad
    //let mut spaces2 = "      ";
    //spaces2 = spaces2.len();

    //let _guess = "42".parse().expect("Not a number!");
    let _guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0;
    let y: f32 = 3.0;
    let remainder = 43 % 5;
    println!("remainder is: {}", remainder);
    let f: bool = false;
    println!("f is: {}", f);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // bad
    //println!("tup is: {}", tup);
    let (a, b, c) = tup;
    println!("The value of b: {}", b);
    println!("The value of second: {}", tup.1);
}
