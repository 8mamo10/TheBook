struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn largest_any<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {
//         // `T` might need a bound for `std::cmp::PartialOrd`
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut l = number_list[0];
    for number in number_list {
        if number > l {
            l = number;
        }
    }
    println!("The largest number is {}", l);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut l = number_list[0];
    for number in number_list {
        if number > l {
            l = number;
        }
    }
    println!("The largest number is {}", l);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest_any(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest_any(&char_list);
    // println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 3.0, y: 4.0 };
    //let wont_work = Point { x: 5, y: 4.0 };
    let will_work = Point2 { x: 5, y: 4.0 };

    let distance = float.distance_from_origin();
    println!("Distance from origin is {}", distance);

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
