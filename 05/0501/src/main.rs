#![allow(unused)]

// 구조체 정의 
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 구조체 반환 함수 
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// 구조체 반환 함수 
fn build_user2(email: String, username: String) -> User {
    // 매개변수인 email과 username이 User구조체의 필드명과 같기 때문에, 함수 build_user 에서 
    // email과 username를 명시하는 아래와 같이 다시 작성할 필요가 없다.
    User {
        email,  
        username,
        active: true,
        sign_in_count: 1,
    }
}


fn main() {
    // 구조체 User의 인스턴스 생성
    // 인스턴스는 반드시 변경 가능(mutable)해야한다.Rust에서는 특정 필드만 변경할 수 있도록 허용하지 않는다.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

}
