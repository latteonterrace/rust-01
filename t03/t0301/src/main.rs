// 변수와 가변성
// fn main() {
//     let x = 5; // 불변이다
//     println!("The value of x is: {}", x);
    
//     // 변수가 불변성인 경우, 일단 값이 이름에 bound되면 해당 값을 변경할 수 없다. 
//     // 아래 오류 발생한다.
//     // x = 6;
//     println!("The value of x is: {}", x);
// }
// fn main() {
//     let mut x = 5; // 가변성
//     println!("The value of x is: {}", x);
//     x = 6; // 오류 발생하지 않는다
//     println!("The value of x is: {}", x);
// }

// shadowing
fn main() {
    let x = 5;
    // 이전에 선언한 변수와 같은 이름의 새 변수를 선언할 수 있고, 새 변수는 이전 변수를 shadows하게 된다
    let x = x + 1;  // shadowed 되었다고 표현
    let x = x * 2;  // let키워드를 사용해서 반복하여 같은 변수명으로 변수를 shadow 할 수 있다 
    println!("The value of x is: {}", x);
}