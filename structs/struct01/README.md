# 구조체

구조체를 정의할 때는 struct 키워드를 먼저 입력하고 명명할 구조체명을 입력하면 됩니다. 이후 중괄호 안에서는, 필드(field)라 불리는 각 구성요소들의 타입과 접근할 수 있는 이름을 정의합니다.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

정의한 구조체를 사용하려면, 각 필드의 값을 명세한 인스턴스(instance)를 생성해야 합니다.

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

구조체에서 특정한 값을 읽어오려면, 점(.) 표기법을 사용하시면 됩니다. 

```rust
println!("User 1 email: {}", user1.email);
```

경이 가능한 구조체 인스턴스에 들어있는 값을 바꾸고자 할 때는, 점(.) 표기법을 사용하여 특정 필드에 새 값을 할당할 수 있습니다.
```rust
user1.email = String::from("anotheremail@example.com");
```

인스턴스는 반드시 변경 가능(mutable)해야합니다. Rust에서는 특정 필드만 변경할 수 있도록 허용하지 않습니다.  mut를 사용해야 합니다.

```rust
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };    
```    

다른 표현식과 마찬가지로, 함수 본문의 마지막에 새 인스턴스 구조체를 표현식(expressions)으로 생성하여 새 인스턴스를 바로 반환 할 수 있습니다.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```
다음과 같이 사용할 수 있습니다. 

```rust
fn main() {
    let user2 = build_user(String::from("aaa@gmail.com"), String::from("aaa"));
    println!("User 2 email: {}", user2.email);
}
```



## 변수명이 필드명과 같을 때 초기화 

변수명과 구조체의 필드명이 같다면, 필드 초기화 축약법(field init shorthand) 을 이용할 수 있습니다. 이를 활용하면 구조체를 생성하는 함수를 더 간단히 작성할 수 있게 됩니다. 


매개변수인 email과 username이 User구조체의 필드명과 같기 때문에, 함수 build_user 에서 email과 username를 명시하는 부분을 다시 작성할 필요가 없습니다. email 필드와 email 매개 변수의 이름이 같기 때문에 email:email 대신 email 만 작성하면됩니다!

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```








