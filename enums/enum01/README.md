# 열거형 (1)

Gender 열거형을 정의하면서 포함할 수 있는 Male과 Female를 나열함으로써 이 개념을 코드에 표현할 수 있습니다. 이것들은 열거형의 variants 라고 합니다:

```rust

// 간단히 열거형을 정의 
enum Gender {
    Male,
    Female,
}
```

## 열거형 값 
아래처럼 Gender 두 개의 variants 에 대한 인스턴스를 만들 수 있습니다:
```rust
    // 인스턴스화 
    let gender1 = Gender::Female; 
    let gender2 = Gender::Male; 
```

if let 구문을 이용해 단순 상태 비교는 가능합니다. 
```rust
    // let을 사용
    if let Gender::Female = gender {
        println!("Female");
    }
```    



