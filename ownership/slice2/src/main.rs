fn main() {
    let s = String::from("hello world");
    
    let word = first_word(&s);
    println!("{}", word);
    s.clear();  // 오류 발생함 ,
    // let mut s = String::from("hello world");와 같이 가변으로 만들면 오류 발생하지 않음
    // 빌림 규칙에서 우리가 만일 무언가에 대한 불변 참조자를 만들었을 경우, 가변 참조자를 만들 수 없다는 점을 상기해보세요.
}


// “스트링 슬라이스”를 나타내는 타입은 &str로 씁니다:
// 공백 문자가 첫번째로 나타난 지점을 찾아서 단어의 끝 인덱스를 얻어냅니다. 
// 공백 문자를 찾으면, 스트링의 시작과 공백 문자의 인덱스를 각각 시작과 끝 인덱스로 사용하여 스트링 
// 슬라이스를 반환합니다.

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}