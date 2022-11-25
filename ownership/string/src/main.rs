fn main() {
    
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str()은 해당 스트링 리터럴을 스트링에 붙여줍니다.
    
    println!("{}", s); // 이 부분이 `hello, world!`를 출력할 겁니다.
}
