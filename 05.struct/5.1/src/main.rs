#![allow(unused_variables)]
#![warn(dead_code)]


// 구조체 정의 
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


//  구조체 정의하고 사용하기
fn struct1() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user1);
}

// 변경이 가능한 구조체 인스턴스에 들어있는 값을 바꾸고자 할 때는, 점(.) 표기법을 사용하여
// 특정 필드에 새 값을 할당할 수 있다
fn struct2() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("{:?}", user1);
}

// 변수명이 같을 경우 간단하게 필드 초기화
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
// build_user() 사용
fn build_user_use() {
    let user1 = build_user("aaa@gmail.com".to_string(), "aaa".to_string());
    println!("{:?}", user1);
}


// 구조체 갱신법 
fn build_spread() {

    // 존재하는 인스턴스에서 기존 값의 대부분은 재사용하고, 몇몇 값만 바꿔 새로운 인스턴스를 정의하는 방법은 유용하다. .. 구문을 사용한다. 
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // 구조체 갱신법
        ..user1
    };
    println!("{:?}", user2);
}


fn main() {
    struct1();
    struct2();
    build_spread();
}
