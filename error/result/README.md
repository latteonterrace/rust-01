# Result와 함께하는 복구 가능한 에러

예를 들어, 만일 우리가 어떤 파일을 여는데 해당 파일이 존재하지 않아서 연산에 실패했다면, 프로세스를 멈추는 대신 파일을 새로 만드는 것을 원할지도 모릅니다.

2장의 “Result 타입으로 잠재된 실패 다루기” 절에서 Result 열거형은 다음과 같이 Ok와 Err라는 두 개의 variant를 갖도록 정의되어 있음을 상기하세요:


```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```


T와 E는 제네릭 타입 파라미터입니다; 10장에서 제네릭에 대해 더 자세히 다룰 것입니다. 지금으로서 여러분이 알아둘 필요가 있는 것은, T는 성공한 경우에 Ok variant 내에 반환될 값의 타입을 나타내고 E는 실패한 경우에 Err variant 내에 반환될 에러의 타입을 나타내는 것이라는 점입니다. Result가 이러한 제네릭 타입 파라미터를 갖기 때문에, 우리가 반환하고자 하는 성공적인 값과 에러 값이 다를 수 있는 다양한 상황 내에서 표준 라이브러리에 정의된 Result 타입과 함수들을 사용할 수 있습니다.


실패할 수도 있기 때문에 Result 값을 반환하는 함수를 호출해 봅시다: Listing 9-3에서는 파일 열기를 시도합니다:


```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

File::open이 Result를 반환하는지 어떻게 알까요? 표준 라이브러리 API 문서를 찾아보거나, 컴파일러에게 물어볼 수 있습니다! 만일 f에게 우리가 알고 있고 그 함수의 반환 타입은 아닐 어떤 타입에 대한 타입 명시를 주고 그 코드의 컴파일을 시도한다면, 컴파일러는 우리에게 타입이 맞지 않는다고 알려줄 것입니다. 그 후 에러 메세지는 f의 타입이 무엇인지 알려줄 것입니다. 한번 해봅시다: 우리는 File::open의 반환 타입이 u32는 아니라는 것을 알고 있으니, let f 구문을 이렇게 바꿔봅시다:

```rust
let f: u32 = File::open("hello.txt");
```

이제 컴파일을 시도하면 다음 메세지가 나타납니다:

```shell
   Compiling result v0.1.0 (G:\github\latteon\rust-01\error\result)
error[E0308]: mismatched types
 --> src\main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |            ---   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found enum `Result`
  |            |
  |            expected due to this
  |
  = note: expected type `u32`
             found enum `Result<File, std::io::Error>`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `result` due to previous error
```
이 메세지는 File::open 함수의 반환 타입이 Result<T, E>라는 것을 알려줍니다. 여기서 제네릭 파라미터 T는 성공값의 타입인 std::fs::File로 채워져 있는데, 이것은 파일 핸들입니다. 에러에 사용되는 E의 타입은 std::io::Error입니다.








