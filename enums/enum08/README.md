# 열거형 (8)


match를 이용하여 Option<T>를 다룰 수 있습니다! 동전들을 비교하는 대신, Option<T>의 variant를 비교할 것이지만, match 표현식이 동작하는 방법은 동일하게 남아있습니다.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // None이 매칭되면 None을 반환
        None => None,
        // Some(5)가  Some(i)와 매칭
        // 새로운 Some 값을 생성
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    // plus_one에서 None과 매치되지 않으면 다음 갈래로 계속 간다
    let six = plus_one(five); 
    // 
    let none = plus_one(None);    
}
```


## None을 작성하지 않는 경우
plus_one에서 None에 대한 흐름이 없습니다. 

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```
여기서는 None 케이스를 다루지 않았고, 따라서 이 코드는 버그를 일으킬 것입니다. 다행히도, 이는 러스트가 어떻게 잡는지 알고 있는 버그입니다. 이 코드를 컴파일하고자 시도하면, 아래와 같은 에러를 얻게 됩니다:

```shell
error[E0004]: non-exhaustive patterns: `None` not covered
 -->
  |
6 |         match x {
  |               ^ pattern `None` not covered
```

Option<T>의 경우, 즉 러스트가 우리로 하여금 None 케이스를 명시적으로 다루는 일을 잊는 것을 방지하는 경우에는, Null 일지도 모를 값을 가지고 있음을 가정하여 에러를 발생합니다. 





