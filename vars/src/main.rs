// fn main() {
//     let x = 5; // 불변이다
//     println!("The value of x is: {}", x);
    
//     // 변수가 불변성인 경우, 일단 값이 이름에 bound되면 해당 값을 변경할 수 없다. 
//     // 아래 오류 발생한다.
//     // x = 6;
//     println!("The value of x is: {}", x);
// }

fn main() {
    let mut x = 5; // 가변성
    println!("The value of x is: {}", x);
    x = 6; // 오류 발생하지 않는다
    println!("The value of x is: {}", x);
}