#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let a: i32 = 1;
    let b: i32 = 2;

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("integer is {:?}", integer);
    println!("float is {:?}", float);
}
