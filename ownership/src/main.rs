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

    let _t1 = gives_ownership();
    let _t2 = String::from("hello");
    let _t3 = takes_and_gives_back(_t2);

    let a1 = String::from("hello");
    let (a2, len) = calculate_length(a1);
    println!("The length of '{}' is {}.", a2, len);

    let a2 = String::from("hello");
    let len = calculate_length_ref(&a2);
    println!("The length of '{}' is {}.", a2, len);

    let mut b = String::from("hello");
    change(&mut b);

    let mut s = String::from("hello");
    let _r1 = &mut s;
    //let r2 = &mut s; // bad
    //println!("{}, {}", r1, r2);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r3);

    //let reference_to_nothing = dangle(); // bad
    let _reference_to_something = no_dangle();
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

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

// bad
// fn change(some_string: &String) {
//     some_string.push_str(", world!");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
