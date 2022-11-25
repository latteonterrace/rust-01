# 변수와 가변성 

다음과 같이 코드를 작성합니다. 

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

빌드하면 다음과 같은 오류가 발생합니다.  왜냐하면 기본적으로 변수는 불변성이기 때문입니다. 


```shell
   Compiling vars v0.1.0 (G:\github\latteon\rust-01\vars)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src\main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
```