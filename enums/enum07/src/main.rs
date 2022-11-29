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

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Coin::Quarter이 매치될 때, state 변수는 그 쿼터 동전의 주에 대한 값에 바인드 될 것입니다
        // 다음과 같이 해당 갈래에서의 코드 내에서 state를 사용할 수 있습니다:
        Coin::Quarter(state) => {
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
