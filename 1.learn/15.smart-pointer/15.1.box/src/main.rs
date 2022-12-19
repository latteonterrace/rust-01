


fn box1() {
    // 힙에 i32 값을 저장하기 위해 박스를 사용
    let b = Box::new(5);
    println!("b = {}", b);

    //b가 main의 끝에 도달하는 것처럼 어떤 박스가 스코프를 벗어날 때, 할당은 해제될 것이다. 
    //할당 해제는 (스택에 저장된) 박스와 이것이 가리키고 있는 (힙에 저장된) 데이터 모두에게 일어난다. 
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

fn box2() {
    let list = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3, 
                Box::new(Nil))))));
}


fn main() {
    box1();
    box2();
}
