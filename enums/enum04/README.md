# 열거형 (4)

구조체의 필드에 열거형을 정의할 수 있습니다. 

```rust
#[derive(Debug)]
enum GenderCategory {
   Male,Female
}

// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct Person {
   name:String,
   // 구조체의 필드에 열거형을 사용
   gender:GenderCategory
}

```
구조체를 인스턴스화할 때 열거형의 값을 다음과 같이 설정합니다.

```rust
fn main() {
    let p1 = Person {
        name:String::from("Mohtashim"),
        // 열거형 값 설정
        gender:GenderCategory::Male
     };
     let p2 = Person {
        name:String::from("Amy"),
        gender:GenderCategory::Female
     };
     println!("{:?}",p1);
     println!("{:?}",p2);

}
```