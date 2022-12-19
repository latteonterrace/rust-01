
// generic type parameter를 이용하여 구조체를 정의할 수 있다. 
// 구조체 이름 바로 뒤에 꺾쇠괄호를 쓰고 그 안에 타입 파라미터의 이름을 선언해야 한다. 
// 그러면 구조체 정의부 내에서 구체적인 데이터 타입을 명시하는 곳에 제네릭 타입을 이용할 수 있다. 
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}


fn use_struct_point() {
    let a: i32 = 1;
    let b: i32 = 2;

    // 구체적인 타입으로 구조체 생성
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("integer is {:?}", integer);
    println!("float is {:?}", float);
}



// 서로 다른 타입을 갖는 genetric type 구조체 
#[derive(Debug)]
struct User<T, U> {
    name: T,
    age: U,
}

// 서로 다른 타입을 갖는 generic type 구조체를 사용하는 함수
fn use_struct_user() {
    let user = User { name: "John", age: 20 };
    println!("user is {:?}", user);
}



fn main() {
    // use_struct_point();
    use_struct_user();
}
