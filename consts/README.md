# 상수

MAX_POINTS라는 이름을 갖는 상수를 선언하는 예제에서는 값을 100,000으로 설정합니다. Rust의 상수 명명 규칙에 따라 모든 단어를 대문자로 사용합니다.

```rust
fn main() {
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}
```