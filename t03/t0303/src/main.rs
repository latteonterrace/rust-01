fn main() {
    // * 구문은 어떤 명령들의 나열로 값을 반환하지 않는 어떤 동작을 수행
    // * 표현식은 결과 값을 산출해낸다.

    // let 키워드를 통해 변수를 만들고 값을 할당하는 구문을 만듭
    let y = 6;

    // let y = 6 구문은 반환 값이 없으므로, x에 bind 시킬 것이 없다.
    // let x = (let y = 6); // 할 수 없음

    // 함수 호출은 표현식
    another_function(5);
    another_function2(5, 5);

    // 블록은 표현식이다
    let y = {
        let x = 3;
        x + 1  // 4를 반환
    };

    // 매크로 호출은 표현식
    println!("The value of y is: {}", y);


    // 반환값 
    let x = five();
    println!("The value of x is: {}", x);
}

// Rust에서의 함수 선언은 fn으로 시작하며 함수 이름 뒤에 괄호의 형식
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
fn another_function2(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// 반환값의 타입은 화살표(->) 뒤에 선언
fn five() -> i32 {
    5 // 반환값
}