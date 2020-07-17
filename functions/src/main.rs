fn main() {
    println!("Hello, world!");
    another_function(5, 6);

    // NG
    //let x = (let y = 6);

    let _x = 5;
    let y = {
        let z = 3;
        z + 1 // no semicolon to make it as a expression
    };
    println!("The value of y is: {}", y);

    let a = five();
    println!("The value of a is: {}", a);
    println!("The value of a is: {}", plus_one(a));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
