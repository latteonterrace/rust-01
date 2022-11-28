struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

// 우리는 구조체의 소유권을 얻기 보다는 빌리기를 원합니다. 이 방법으로, main은 그 소유권을 유지하고 rect1을 계속 이용할 수 있는데, 이는 우리가 함수 시그니처 내에서와 함수 호출시에 &를 사용하게 된 이유입니다.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}