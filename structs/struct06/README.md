# 구조체 (6)

메소드(method) 는 함수와 유사합니다: 이들은 fn 키워드와 이름을 가지고 선언되고, 파라미터와 반환값을 가지고 있으며, 다른 어딘가로부터 호출되었을때 실행될 어떤 코드를 담고 있습니다. 하지만, 메소드는 함수와는 달리 구조체의 내용 안에 정의됩니다. 

## 메소드 문법

Rectangle 구조체 위에서 정의된 area 메소드를 만들어 봅시다. Rectangle의 내용 안에 함수를 정의하기 위해서, impl (구현: implementation) 블록을 시작합니다. 첫번째 파라미터가 언제나 self인데, 이는 메소드가 호출되고 있는 구조체의 인스턴스를 나타냅니다.self의 타입이 Rectangle 라는 사실을 알 수 있기 때문입니다. 우리가 &Rectangle이라고 썼던 것 처럼, self 앞에도 여전히 &를 사용할 필요가 있음을 주목하세요
 
```rust
#[derive(Debug)]

struct Rectangle {
    length: u32,
    width: u32,
}


// Rectangle의 내용 안에 함수를 정의하기 위해서, impl (구현: implementation) 블록을 시작합니다. 
impl Rectangle {
    // 메소드는 함수와는 달리 구조체의 내용 안에 정의
    // 첫번째 파라미터가 언제나 self인데, 이는 메소드가 호출되고 있는 구조체의 인스턴스를 나타냅니다.
    // self의 타입이 Rectangle 라는 사실을 알 수 있기 때문입니다. 우리가 &Rectangle이라고 썼던 것 처럼, self 앞에도 여전히 &를 사용할 필요가 있음을 주목하세요
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

## 연관함수
impl 블록의 또다른 유용한 기능은 self 파라미터를 갖지 않는 함수도 impl 내에 정의하는 것이 허용된다는 점입니다. 이를 연관 함수 (associated functions) 라고 부르는데, 그 이유는 이 함수가 해당 구조체와 연관되어 있기 때문입니다. 이들은 메소드가 아니라 여전히 함수인데, 이는 함께 동작할 구조체의 인스턴스를 가지고 있지 않아서 그렇습니다. 여러분은 이미 String::from 연관 함수를 사용해본 적이 있습니다.


```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    // 연관 함수
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    let sq_val = Rectangle::square(3);
    println!("sq_val: {:?}", sq_val);
}
```

이 연관 함수를 호출하기 위해서는 let sq = Rectangle::square(3); 처럼, 구조체 이름과 함께 :: 문법을 이용합니다. 이 함수는 구조체의 이름공간 내에 있습니다: :: 문법은 연관 함수와 모듈에 의해 생성된 이름공간 두 곳 모두에서 사용되는데, 모듈에 대해서는 나중에 다룰 것입니다.


**{} 내에 :? 명시자를 집어넣는 것은 println!에게 Debug라 불리우는 출력 포맷을 사용하고 싶다고 말해줍니다.** Debug는 개발자에게 유용한 방식으로 우리의 구조체를 출력할 수 있도록 해줘서 우리 코드를 디버깅 하는 동안 그 값을 볼수 있게 해주는 트레잇입니다.


러스트는 디버깅 정보를 출력하는 기능을 포함하고 있는 것이 맞지만, 우리 구조체에 대하여 해당 기능을 활성화하도록 명시적인 사전동의를 해주어야 합니다. 그러기 위해서, 보는 바와 같이 구조체 정의부분 바로 전에 #[derive(Debug)] 어노테이션을 추가합니다:

```rust
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}
```
