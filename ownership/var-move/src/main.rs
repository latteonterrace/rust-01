fn main() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); 오류 발생

    // clone
    let s3 = String::from("hello");
    let s4 = s3;
    println!("{}, world!", s4);

}
