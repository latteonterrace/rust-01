pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 열거형 또한 모듈과 비슷한 일종의 이름공간을 형성하고 있기 때문에
// 열거형의 variant 또한 use를 이용하여 가져올 수 있습니다
use TrafficLight::{Red, Yellow};
use a::series::of::nested_modules;


fn main() {
    // use를 사용하여 간단하게 함수 호출
    nested_modules();

    let red = Red;  // 열거형의 variant 사용 
    let yellow = Yellow;
    // Green variant에 대해서는 여전히 TrafficLight 이름공간을 명시하고 있는데, 
    // 이는 use 구문 내에 Green를 포함하지 않았기 때문입니다.
    let green = TrafficLight::Green;
}

