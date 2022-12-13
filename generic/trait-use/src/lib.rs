// 공통 추상화를 지원
// java의 interface와 비슷한 개념
pub trait Animal {
    // 시그니처만 정의
    fn custom_bark(&self);
    // 메서드 구현이 있음
    fn common_bark(&self) {
        println!("bark");
    }
}

// Dog 구조체
pub struct Dog {
    pub Pomeranian: String,
    pub Poodle: String,
    pub Dashund: String,
}

// impl 뒤에 구현하려고 하는 트레잇 이름을 적고, for 뒤에 구현하려는 타입을 적음
impl Animal for Dog {
    // 메소드 구현
    fn custom_bark(&self) {
        println!("custom bark");
    }
}

