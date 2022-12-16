
// https://doc.rust-lang.org/std/string/struct.String.html


// 불변 스트링 생성
fn create_immutable_string() {
    let hello = String::from("Hello, world!");
    println!("{}", hello);
}

// 가변 스트링 생성 
fn create_mutable_string() {
    let mut hello = String::from("Hello, world!");
    hello.push_str("!!!");
    println!("{}", hello);
}


fn main() {
    create_immutable_string();
    create_mutable_string();
}
