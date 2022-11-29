# 열거형 (5)

표준 라이브러리에서 열거형으로 정의된 또 다른 타입인 Option 에 대한 사용 예를 살펴볼 것입니다. Option 타입은 많이 사용되는데, 값이 있거나 없을 수도 있는 아주 흔한 상황을 나타내기 때문입니다. 


러스트는 다른 언어들에서 흔하게 볼 수 있는 null 특성이 없습니다. Null 은 값이 없다는 것을 표현하는 하나의 값입니다. null 을 허용하는 언어에서는, 변수는 항상 두 상태중 하나가 될 수 있습니다: null 혹은 null 이 아님.



스트에는 null 이 없지만, 값의 존재 혹은 부재의 개념을 표현할 수 있는 열거형이 있습니다. 이 열거형은 Option<T> 이며, 다음과 같이 표준 라이브러리에 정의되어 있습니다:


```rust
enum Option<T> {
    Some(T),
    None,
}
```

Option<T> 열거형은 매우 유용하며 기본적으로 포함되어 있기 때문에, 명시적으로 가져오지 않아도 사용할 수 있습니다. 또한 variants 도 마찬가지입니다: Option:: 를 앞에 붙이지 않고, Some 과 None 을 바로 사용할 수 있습니다. Option<T> 는 여전히 일반적인 열거형이고, Some(T) 과 None 도 여전히 Option<T> 의 variants 입니다.


\<T\> 는 러스트의 문법이며 아직 다루지 않았습니다. 



여기 숫자 타입과 문자열 타입을 갖는 Option 값에 대한 예들이 있습니다:

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```
Some 이 아닌 None 을 사용한다면, Option<T> 이 어떤 타입을 가질지 러스트에게 알려줄 필요가 있습니다. 컴파일러는 None 만 보고는 Some variant 가 어떤 타입인지 추론할 수 없습니다.



Option<T> 와 T (T 는 어떤 타입이던 될 수 있음)는 다른 타입이며, 컴파일러는 Option<T> 값을 명확하게 유효한 값처럼 사용하지 못하도록 합니다. 예를 들면, 아래 코드는 Option<i8> 에 i8 을 더하려고 하기 때문에 컴파일되지 않습니다:

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y; // 오류 발생,  Option<T>와 T는 다른 타입임
```

다르게 얘기하자면, T 에 대한 연산을 수행하기 전에 Option<T> 를 T 로 변환해야 합니다.


다음은 Option 타입을 반환합니다.

```rust
fn main() {
    let result = is_even(3);
    println!("{:?}",result);
    println!("{:?}",is_even(30));
 }

 
 fn is_even(no:i32)->Option<bool> {
    if no%2 == 0 {
       Some(true)
    } else {
       None
    }
 }
 ```

기본적인 사용법에 대한 예는 다음을 참고합니다. 
```rust
  // Option::None 지정
    let no_index: Option<i32> = Option::None;
    // Option::Some 지정 
    let index: Option<i32> = Option::Some(1);
     
    // Option:: 생략 표현
    let no_index: Option<i32> = None;
    let index = Some(1);
```    



