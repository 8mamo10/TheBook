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
    let spaces = "      ";
    let spaces = spaces.len();
    // bad
    let mut spaces2 = "      ";
    spaces2 = spaces2.len();
}
