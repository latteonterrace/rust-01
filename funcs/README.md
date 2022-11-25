# Functions


[함수 동작 원리](https://rinthel.github.io/rust-lang-book-ko/ch03-03-how-functions-work.html)



함수는 Rust에 녹아들어 있습니다. 여러분은 이미 언어에서 가장 중요하게 생각하는 main함수를 보셨습니다. 이는 다수의 프로그램에서 실행 지점입니다. 여러분은 또한 fn 키워드도 보셨을텐데, 이는 새로운 함수의 선언을 가능하게 합니다.

Rust 코드는 뱀 형태를 변수나 함수 이름의 형식 규칙으로 사용합니다. 뱀 형태에서, 모든 문자는 소문자를 사용하며 밑줄 표시로 단어를 구분합니다. 다음은 예제로 함수를 선언하는 프로그램입니다:


```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

```


## 반환 값을 갖는 함수
우리는 반환되는 값을 명명해야 할 필요는 없지만, 그들의 타입은 화살표(->) 뒤에 선언해야 합니다. 

대부분의 함수들은 암묵적으로 마지막 표현식을 반환합니다. 값을 반환하는 함수의 예를 보겠습니다:


```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```




