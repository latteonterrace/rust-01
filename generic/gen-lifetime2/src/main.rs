
// 제네릭 라이프타임 파라미터의 이름을 구조체의 이름 뒤편에 꺾쇠괄호 안에다 선언
struct ImportantExcerpt<'a> {
    // 참조자 
    part: &'a str,
}

/// 
/// 메인 함수입니다. 
/// ```shell
/// cargo run
/// ```
fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}