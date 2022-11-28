# 구조체 (2)

존재하는 인스턴스에서 기존 값의 대부분은 재사용하고, 몇몇 값만 바꿔 새로운 인스턴스를 정의하는 방법은 유용합니다

## 구조체 갱신법을 이용한 새 구조체 인스턴스 생성

user2에 email과 username은 새로 할당하고, 나머지 필드들은  user1의 값들을 그대로 사용하는 방식으로 User 인스턴스를 생성하는 것을 보여줍니다.



```rust
fn main() {
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,   // user1의 값들을 그대로 사용
        sign_in_count: user1.sign_in_count,
    };    

    println!("user2: {}", user2.email);
    
}
```


구조체 갱신법(struct update syntax)은 입력으로 주어진 인스턴스와 변화하지 않는 필드들을 명시적으로 할당하지 않기 위해 .. 구문을 사용합니다

```rust
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };    
```
새 User 구조체 생성 시 email과 username 필드에는 새 값을 할당하고, 나머지 필드는 user1에서 재사용합니다. 