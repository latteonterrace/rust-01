fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let x = 2.0; // f64
    println!("The value of x is: {}", x);
    let y: f32 = 3.0; // f32
    println!("The value of y is: {}", y);
    let t = true;
    println!("The value of t is: {}", t);
    let f: bool = false; // with explicit type annotation
    println!("The value of f is: {}", f);
    let c = 'z';
    println!("The value of c is: {}", c);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);



}
