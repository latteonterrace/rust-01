# 구조체 (3)

구조체명을 통해 의미를 부여할 수 있으나 필드의 타입만 정의할 수 있고 명명은 할 수 없는, 튜플 구조체(tuple structs)라 불리는 튜플과 유사한 형태의 구조체도 정의할 수 있습니다.


## 튜플 구조체


구조체 내의 타입이 모두 동일하더라도 각각의 구조체는 고유의 타입입니다. 
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {}, {}, {}", black.0, black.1, black.2);

}
```







