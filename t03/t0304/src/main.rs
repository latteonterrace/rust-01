fn main() {

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }


    // let number = 3;
    // // Rust가 bool을 기대하였으나 정수형이 왔기 때문에 에러가 난다. 
    // if number {  //  에러 
    //        println!("number was three");
    // }

    // 조건이 bool이기 때문에 에러 없다 
    let chk = true; 
    if chk {
        println!("chk is true");
    } else {
        println!("chk is false");
    }


    // else if외 다수 조건
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    // if가 표현식이기 때문에, 우리는 이를 let 구문의 우측에 사용할 수 있다. 
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);    

    // 반복문
    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");    

    // for 
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}