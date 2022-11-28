
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn main() {

    let mut  user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };


    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     username: String::from("anotherusername567"),
    //     active: user1.active,   // user1의 값들을 그대로 사용
    //     sign_in_count: user1.sign_in_count,
    // };    

    // 새 User 구조체 생성 시 email과 username 필드에는 새 값을 할당하고, 나머지 필드는 user1에서 재사용
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };    

    println!("user2: {}", user2.email);
    
}
