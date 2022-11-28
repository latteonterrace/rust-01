
// 간단히 열거형을 정의 
#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

// 다양한 타입으로 열거형을 정의할 수 있습니다.
#[derive(Debug)]
enum UserInfo {
    Name(String),
    Age(u8),
    // 열거형에 열거형을 정의합니다.
    Gender(Gender),
}


fn main() {
    // 열거형에 값을 넣을 수 있습니다.
    let gender = UserInfo::Gender(Gender::Male);
    println!("Gender: {:?}" , gender);    
}
