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