// 다양한 타입으로 열거형을 정의할 수 있습니다.
#[derive(Debug)]
enum UserInfo {
    Name(String),
    Age(u8),
    Height(u8),
    Weight(u8),
}

fn main() {
    // 열거형에 값을 넣을 수 있습니다.
    let name = UserInfo::Name(String::from("John"));
    println!("Name: {:?}", name);
    let age  = UserInfo::Age(30);
    let height = UserInfo::Height(180);
    let weight = UserInfo::Weight(80);
    println!("Weight: {:?}", weight);

}
