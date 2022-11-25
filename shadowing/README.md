# Shadowing

, 이전에 선언한 변수와 같은 이름의 새 변수를 선언할 수 있고, 새 변수는 이전 변수를 shadows하게 됩니다. Rustaceans들은 이를 첫 변수가 두 번째에 의해 shadowed 됐다고 표현하게 됩니다. 해당 변수명은 두 번째 변수의 값을 갖게 된다는 뜻이죠. let키워드를 사용해서 다음처럼 반복하여 같은 변수명으로 변수를 shadow 할 수 있습니다.

[Shadowing](https://rinthel.github.io/rust-lang-book-ko/ch03-01-variables-and-mutability.html)    


```rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```