# 열거형 (7) - 값들을 바인딩하는 패턴들

매치 갈래의 또 다른 유용한 기능은 패턴과 매치된 값들의 부분을 바인딩할 수 있다는 것입니다. 이것이 열거형 variant로부터 어떤 값들을 추출할 수 있는 방법입니다.

Quarter variant 내에 UsState 값을 포함하도록 우리의 enum을 변경함으로써 추가할 수 있습니다. 

```rust
#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

// 우리는 이 정보를 Quarter variant 내에 UsState 값을 포함하도록 
// enum을 변경함으로써 추가할 수 있음
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

 Coin::Quarter이 매치될 때, state 변수는 그 쿼터 동전의 주에 대한 값에 바인드 될 것입니다. 다음과 같이 해당 갈래에서의 코드 내에서 state를 사용할 수 있습니다:

```rust 
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Coin::Quarter이 매치될 때, state 변수는 그 쿼터 동전의 주에 대한 값에 바인드 될 것입니다
        // 다음과 같이 해당 갈래에서의 코드 내에서 state를 사용할 수 있습니다:
        Coin::Quarter(state) => {
            //state에 대한 바인딩은 값 UsState::Alaska가 될 것입니다. 그러면 이 바인딩을 println! 표현식 내에서 사용할 수 있습니다.
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(coin);
    println!("value of coin is {}", value);
    
}
```
