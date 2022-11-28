
struct  User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


//다른 표현식과 마찬가지로, 함수 본문의 마지막에 새 인스턴스 구조체를 표현식(expressions)으로 생성하여 새 인스턴스를 바로 반환 할 수 있습니다.
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}


fn main() {
    
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };    

    user1.email = String::from("anotheremail@example.com");
    println!("User 1 email: {}", user1.email);

    let user2 = build_user(String::from("aaa@gmail.com"), String::from("aaa"));
    println!("User 2 email: {}", user2.email);

}
