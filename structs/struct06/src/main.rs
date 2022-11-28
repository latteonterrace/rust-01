// debug 정보 
// 러스트는 디버깅 정보를 출력하는 기능을 포함하고 있는 것이 맞지만, 우리 구조체에 대하여 해당 기능을 
// 활성화하도록 명시적인 사전동의를 해주어야 합니다. 그러기 위해서, 보는 바와 같이 구조체 정의부분 바로 전에 
//#[derive(Debug)] 어노테이션을 추가합니다:
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
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };
    // 구조체 출력을 위해서 {:?}를 사용 
    println!("rect1 is {:?}", rect1); // rect1 is Rectangle { length: 50, width: 30 }

    // 읽기 좀 더 수월한 출력을 쓰는 것이 유용합니다; 그러한 경우, println! 스트링 내에 {:?} 
    // 대신 {:#?}을 사용할 수 있습니다. 
    println!("rect1 is {:#?}", rect1); // rect1 is Rectangle { length: 50, width: 30 }
    // 예제 내에서 {:#?} 스타일을 이용하게 되면, 출력이 아래와  같이 생기게 될 것입니다:
    // rect1 is Rectangle {
    //     length: 50,
    //     width: 30,
    // }
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let sq_val = Rectangle::square(3);
    println!("sq_val: {:?}", sq_val);
}