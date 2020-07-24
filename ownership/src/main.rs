fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{}, world!", s1); // bad
    println!("{}, world!", s2); // good

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    //println!("{}", s); // bad

    let t1 = gives_ownership();
    let t2 = String::from("hello");
    let t3 = takes_and_gives_back(t2);

    let a1 = String::from("hello");
    let (a2, len) = calculate_length(a1);
    println!("The length of '{}' is {}.", a2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
