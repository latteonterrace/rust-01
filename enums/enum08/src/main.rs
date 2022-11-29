fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // None이 매칭되면 None을 반환
        None => None,
        // Some(5)가  Some(i)와 매칭
        // 새로운 Some 값을 생성
        Some(i) => Some(i + 1),
    }
}



fn main() {
    let five = Some(5);
    // plus_one에서 None과 매치되지 않으면 다음 갈래로 계속 간다
    let six = plus_one(five); 
    // 
    let none = plus_one(None);    
}
