#![allow(unused)]
fn main() {
    // 변수의 스코프
    {
        // s는 유효하지 않습니다. 아직 선언이 안됐거든요.
        let s = "hello"; // s는 이 지점부터 유효합니다.

        // s를 가지고 뭔가 합니다.
    } // 이 스코프는 이제 끝이므로, s는 더이상 유효하지 않습니다.

    // 메모리와 할당
    {
        let s = String::from("hello"); // s는 여기서부터 유효합니다
                                       // s를 가지고 뭔가 합니다
    } // 이 스코프는 끝났고, s는 더 이상 유효하지 않습니다
      // drop 호출 -> 메모리 해제

    // 이동
    let x = 5; // x에 5를 할당
    let y = x; // y에 복사

    // String 타입의 경우
    let s1 = String::from("hello");
    let s2 = s1; // 복사 안된다. s1의 포인터, 길이, 용량이 s2로 이동한다.
                 // s1은 더이상 유효하지 않다.
                 // 에러 발생
                 // println!("{}, world!", s1); // 유효하지 않은 참조자 사용 에러

    // clone
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 깊은 복사

    println!("s1 = {}, s2 = {}", s1, s2);

    // 소유권과 함수
    let s = String::from("hello"); // s가 스코프 안으로 들어왔습니다.

    takes_ownership(s); // s의 값이 함수 안으로 이동했습니다...
                        // ... 그리고 이제 더이상 유효하지 않습니다.
    let x = 5; // x가 스코프 안으로 들어왔습니다.

    makes_copy(x); // x가 함수 안으로 이동했습니다만,
                   // i32는 Copy가 되므로, x를 이후에 계속
                   // 사용해도 됩니다.

}

fn takes_ownership(some_string: String) {
    // some_string이 스코프 안으로 들어왔습니다.
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는
  // 해제되었습니다.

fn makes_copy(some_integer: i32) {
    // some_integer이 스코프 안으로 들어왔습니다.
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않습니다.

fn gives_ownership() -> String {
    // gives_ownership 함수가 반환 값을
    // 호출한 쪽으로 이동시킵니다.

    let some_string = String::from("hello"); // some_string이 스코프 안에 들어왔습니다.

    some_string // some_string이 반환되고, 호출한 쪽의
                // 함수로 이동됩니다.
}

// takes_and_gives_back 함수는 String을 하나 받아서 다른 하나를 반환합니다.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string이 스코프
    // 안으로 들어왔습니다.

    a_string // a_string은 반환되고, 호출한 쪽의 함수로 이동됩니다.
}
