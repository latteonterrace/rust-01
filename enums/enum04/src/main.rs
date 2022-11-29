


#[derive(Debug)]
enum GenderCategory {
   Male,Female
}

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct Person {
   name:String,
   // 구조체의 필드에 열거형을 사용
   gender:GenderCategory
}


fn main() {


    let p1 = Person {
        name:String::from("Mohtashim"),
        // 열거형 값 설정
        gender:GenderCategory::Male
     };
     let p2 = Person {
        name:String::from("Amy"),
        gender:GenderCategory::Female
     };
     println!("{:?}",p1);
     println!("{:?}",p2);

}
