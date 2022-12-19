
// mut가 없기 때문에 기본적으로 불변 참조자
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    // 슬라이스 반환 
    &s[..]
}

fn main() {

    let mut s = String::from("hello world");

    // 가변 s를 참조자로 만들어 전달 
    // first_word는 불변으로 전달
    let word = first_word(&s);

    // s가 first_word에서 불변이 되었기 때문에 에러 
    s.clear(); // Error!

    println!("the first word is: {}", word);
}