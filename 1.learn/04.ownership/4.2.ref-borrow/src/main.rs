fn main() {
    let s1 = String::from("hello");

    // &s1 문법은 우리가 s1의 값을 참조하지만 소유하지는 않는 참조자를 생성
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// 함수 시그니처도 &를 사용하여 인자 s의 타입이 참조자라는 것을 나타낸다
// 함수의 파라미터로 참조자를 만드는 것을 빌림이라고 부른다.  
fn calculate_length(s: &String) -> usize {  // s는 String의 참조자입니다
    s.len()
}// 여기서 s는 스코프 밖으로 벗어났습니다. 하지만 가리키고 있는 값에 대한 소유권이 없기
  // 때문에, 아무런 일도 발생하지 않습니다.