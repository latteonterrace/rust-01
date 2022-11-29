# 열거형 (6)

러스트는 match라고 불리는 극도로 강력한 흐름 제어 연산자를 가지고 있는데 이는 우리에게 일련의 패턴에 대해 어떤 값을 비교한 뒤 어떤 패턴에 매치되었는지를 바탕으로 코드를 수행하도록 해줍니다


[Rust - Enums](https://www.tutorialspoint.com/rust/rust_enums.htm)

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {

    // match 키워드 뒤에 표현식 사용
    // if 사용 시 해당 식은 bool 타입을 반환할 필요가 있지만
    // match는 어떤 타입이든 가능
    match coin {
        // match 표현식이 실행될 때, 결과 값을 각 갈래의 패턴에 대해서 순차적으로 비교합니다. 
        // 만일 어떤 패턴이 그 값과 매치되면, 그 패턴과 연관된 코드가 실행됩니다. 만일 그 패턴이 
        // 값과 매치되지 않는다면, 동전 분류기와 비슷하게 다음 갈래로 실행을 계속합니다.
        Coin::Penny => 1,
        // 만일 매치 갈래 내에서 여러 줄의 코드를 실행시키고 싶다면, 중괄호를 이용할 수 있습니다
        Coin::Nickel => {
            println!("Lucky nickel!");
            5 // 5 반환
        },
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


fn main() {
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("value of coin is {}", value);
}
```


