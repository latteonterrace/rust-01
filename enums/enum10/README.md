# 열거형 (10)

if let 문법은 if와 let을 조합하여 하나의 패턴만 매칭 시키고 나머지 경우는 무시하는 값을 다루는 덜 수다스러운 방법을 제공합니다. 

```rust
let some_u8_value = Some(0u8);

if let Some(3) = some_u8_value {
    println!("three");
}
```


if let은 =로 구분된 패턴과 표현식을 입력받습니다. 이는 match와 동일한 방식으로 작동하는데, 여기서 표현식은 match에 주어지는 것이고 패턴은 이 match의 첫 번째 갈래와 같습니다.


if let과 함께 else를 포함시킬 수 있습니다. else 뒤에 나오는 코드 블록은 match 표현식에서 _ 케이스 뒤에 나오는 코드 블록과 동일합니다. 

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```


