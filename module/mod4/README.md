# 모듈 (4)

모듈 이름을 호출 구문의 일부분으로 사용하여  해당 모듈 내에 정의된 함수를 호출할 수 있습니다. 

```rust
a::series::of::nested_modules();
```
## use를 이용한 간결한 가져오기
use를 이용하여 긴 함수 호출을 줄일 수 있습니다. 

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}


use a::series::of;

fn main() {
    of::nested_modules();
}
```
use a::series::of; 줄은 of 모듈을 참조하고 싶은 곳마다 a::series::of 전부를 사용하기 보다는 of를 사용할 수 있다는 뜻입니다.


use 키워드는 우리가 명시한 것만 스코프 내로 가져옵니다. 즉 모듈의 자식들을 스코프 내로 가져오지는 않습니다. 다음과 같이 use 구문 안에서 모듈 대신 함수를 명시하여 스코프 내에서 함수를 가져올 수도 있습니다:

```rust
use a::series::of::nested_modules;

fn main() {
    nested_modules();
}
```

열거형 또한 모듈과 비슷한 일종의 이름공간을 형성하고 있기 때문에, 열거형의 variant 또한 use를 이용하여 가져올 수 있습니다.
```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
```

Green variant에 대해서는 여전히 TrafficLight 이름공간을 명시하고 있는데, 이는 use 구문 내에 Green를 포함하지 않았기 때문입니다.


## *를 이용한 모두(glob) 가져오기


이름공간 내의 모든 아이템을 가져오기 위해서는 * 문법을 이용할 수 있습니다. 예를 들면:
```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
```
*는 글롭(glob) 이라고 부르며, 이는 이름공간 내에 공개된 모든 아이템을 가져올 것입니다



## super를 사용하여 부모 모듈에 접근하기
src/lib.rs가 다음과 같이 되어 있다고 가정합니다. tests라는 이름의 모듈이 우리의 다른 모듈들 옆에 있고, it_works라는 이름의 함수 하나를 담고 있지요. 좀 특별한 주석(annotation)이 있지만, tests 모듈은 그냥 또다른 모듈일 뿐입니다! 
```rust
pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

따라서 우리의 모듈 계층 구조는 아래와 같이 생겼습니다:

```shell
communicator
 ├── client
 ├── network
 |   └── client
 └── tests
```

it_works 함수 안에서 우리의 client::connect 함수를 호출해 봅시다:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        client::connect();
    }
}
```

cargo test 명령을 써서 테스트를 실행하면 컴파일이 실패합니다. 

어떻게 모듈 계층 구조 내에서 한 모듈 위로 거슬러 올라가 tests 모듈 안에서 client::connect 함수를 호출할 수 있을까요? 아래와 같이 앞에 콜론 두개를 사용하여 러스트에게 우리가 루트부터 시작하여 전체 경로를 나열하겠다고 알려주는 방법이 있습니다:
```rust
::client::connect();
```

혹은, 아래와 같이 super를 사용하여 계층 구조 상에서 현재 모듈로부터 한 모듈 거슬러 올라갈 수도 있습니다:
```rust
super::client::connect();
```


각각의 테스트에 super::를 타이핑해야 하는 것이 짜증날수 있겠지만, 여러분은 이미 여기에 대한 해답이 될 도구를 보셨습니다: use 말이죠! super::의 기능은 use에 제공한 경로를 변경시켜서 이제 루트 모듈 대신 부모 모듈에 상대적인 경로가 되게 해줍니다.


```rust
#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
```




