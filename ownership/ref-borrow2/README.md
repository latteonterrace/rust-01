# 참조자(References)와 빌림(Borrowing)

빌린 무언가를 고치려고 시도한다면 무슨 일이 생길까요?

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```
오류가 발생합니다. 변수가 기본적으로 불변인 것처럼, 참조자도 마찬가지입니다. 우리가 참조하는 어떤 것을 변경하는 것은 허용되지 않습니다.




## 가변 참조자(Mutable References)

코드를 살짝만 바꾸면 오류를 고칠 수 있습니다:

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```


먼저 s를 mut로 바꿔야 합니다. 그리고 &mut s로 가변 참조자를 생성하고 some_string: &mut String으로 이 가변 참조자를 받아야 합니다.

하지만 가변 참조자는 딱 한가지 큰 제한이 있습니다: 특정한 스코프 내에 특정한 데이터 조각에 대한 가변 참조자를 딱 하나만 만들 수 있다는 겁니다. 아래 코드는 실패할 겁니다:


```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

