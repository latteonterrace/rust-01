# 열거형 (2) 

열거형 variant 에 어떤 종류의 데이터라도 넣을 수 있습니다: 예를 들면 문자열, 숫자 타입, 혹은 구조체. 다른 열거형 조차도 포함할 수 있습니다! 또한 표준 라이브러리 타입들은 어떤 경우에는 해결책으로 생각한 것보다 훨씬 더 복잡하지 않습니다.

```rust
// 다양한 타입으로 열거형을 정의할 수 있습니다.
#[derive(Debug)]
enum UserInfo {
    Name(String),
    Age(u8),
    Height(u8),
    Weight(u8),
}
```
열거형에 값을 넣을 수 있습니다.
```rust
let name = UserInfo::Name(String::from("John"));
```
다음 예제는 열거혀에 값을 넣고 출력하는 방법을 보여줍니다.

```rust
fn main() {
    // 열거형에 값을 넣을 수 있습니다.
    let name = UserInfo::Name(String::from("John"));
    println!("Name: {:?}", name);
    let age  = UserInfo::Age(30);
    let height = UserInfo::Height(180);
    let weight = UserInfo::Weight(80);
    println!("Weight: {:?}", weight);

}
```