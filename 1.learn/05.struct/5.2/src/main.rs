
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

// 구조체 메소드 

// Rectangle의 내용 안에 함수를 정의하기 위해서, impl (구현: implementation) 블록을 시작한다. 
impl Rectangle {
    // 구조체 메소드 
    // 첫번째 파라미터가 언제나 self인데, 이는 메소드가 호출되고 있는 구조체의 인스턴스를 나타내다
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn use_method() {
    let rect1 = Rectangle { length: 50, width: 30 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()  // 메소드 호출
    );
}



// 연관 함수 (Associated Functions)

impl Rectangle {
    // 연관 함수 (associated function)
    // 연관 함수는 구조체의 인스턴스를 생성하지 않고도 호출할 수 있다
    // 연관 함수는 구조체의 이름에 :: 연산자를 사용하여 호출한다
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}


fn use_associated_func() {
    let sq = Rectangle::square(3);
    println!("{:?}", sq);
}


fn main() {
    use_method();
    use_associated_func();
}