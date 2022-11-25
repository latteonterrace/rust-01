fn main() {
    println!("Hello, world!");

    another_function();
    another_function3(5);
    let x = 5;
    println!("The value of x is: {}", x);
    let x = five();
    println!("The value of x is: {}", x);

    // y=4가 된다 
    let y = {
        let x = 3; 
        x + 1  // 세미콜론 없음 
    }; // 여기는 세미콜론
    println!("The value of y is: {}", y);
}


fn another_function() {
    println!("Another function.");
}

fn another_function3(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}



