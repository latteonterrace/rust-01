
// 간단히 열거형을 정의 
#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

fn main() {
    // 인스턴스화 
    let gender = Gender::Female; 
    // enum 출력
    println!("Gender: {:?}", gender);
    
    
    // 아래는 오류 발생 
    // if gender == Gender::Female  {
    //     println!("Female");
    // }
    // let을 사용
    if let Gender::Female = gender {
        println!("Female");
    }
}