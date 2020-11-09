struct Point {
    x: i32,
    y: i32,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Text {
    Hello { id: i32 },
}

fn main() {
    // if let
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color!");
        } else {
            println!("Using orange as the background color!");
        }
    } else {
        println!("Using blue as the background color!");
    }

    // while let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let
    let _x = 5;
    let (_x, _y, _z) = (1, 2, 3);
    //let (x, y) = (1, 2, 3);

    // fn
    let point = (3, 5);
    print_coordinates(&point);

    let some_option_value: Option<&str> = None;
    //let Some(x) = some_option_value;
    if let Some(x) = some_option_value {
        println!("{}", x);
    };
    // warning: irrefutable if-let pattern
    if let x = 5 {
        println!("{}", x);
    };

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // enum
    let _msg = Message::ChangeColor(0, 160, 255);
    let _msg = Message::Move { x: 1, y: 2 };
    let _msg = Message::Write("hoge".to_string());
    let msg = Message::Quit;
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
    }

    // ref
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];
    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
    println!("sum_of_sauares: {}", sum_of_squares);

    let ((_feet, _inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    foo(3, 4);

    let mut setting_value = Some(5);
    //let mut setting_value = None;
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }
    match numbers {
        (first, _, .., fifth) => {
            println!("Some numbers: {}, {}", first, fifth);
        }
    }

    let s = Some(String::from("Hello!"));
    //if let Some(_s) = s {
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    let origin = Point3d { x: 0, y: 0, z: 0 };
    match origin {
        Point3d { x, .. } => println!("x is {}", x),
    }

    // ref
    let robot_name = Some(String::from("Bros"));
    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }
    println!("robot_name is {:?}", robot_name);

    let mut robot_name = Some(String::from("Bros"));
    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }
    println!("robot_name is {:?}", robot_name);

    // match guard
    let num = Some(4);
    //let num = Some(10);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    //let x = Some(5);
    //let x = Some(50);
    let x = Some(10);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 4;
    //let y = false;
    let y = true;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @
    //let text = Text::Hello { id: 5 };
    //let text = Text::Hello { id: 12 };
    let text = Text::Hello { id: 100 };
    match text {
        Text::Hello {
            id: id_variable @ 3...7,
        } => println!("Found an id in range: {}", id_variable),
        Text::Hello { id: 10...12 } => println!("Found an id in another range"),
        Text::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
