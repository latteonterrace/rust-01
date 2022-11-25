fn main() {
    let s1 = String::from("hello");

    // 참조를 &을 사용하여 넘김
    // 소유권이 넘어가지 않음
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// 참조를 받음 
// 함수의 파라미터로 참조자를 만드는 것을 빌림이라고 부름 
fn calculate_length(s: &String) -> usize {
    s.len()
}




