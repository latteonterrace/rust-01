# 모듈 (1)

바이너리 크레이트(crate)을 만드는 대신에 라이브러리 크레이트을 만들 것입니다.. 여기서 라이브러리 크레이트이란 다른 사람들이 자신들의 프로젝트에 디펜던시(dependency)로 추가할 수 있는 프로젝트를 말합니다.


이 라이브러리를 communicator라고 부르겠습니다. 라이브러리를 만들기 위해서는 --bin 대신 --lib 옵션을 넘기세요:


```shell
$ cargo new communicator --lib
$ cd communicator'
```
카고가 src/main.rs 대신 src/lib.rs을 생성했음을 주목하세요. src/lib.rs 내부를 보면 다음과 같은 코드를 찾을 수 있습니다:

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

\#[]와 mod tests 문법은 이 장의 “super를 이용하여 부모 모듈에 접근하기”절에서 더 자세히 다룰 것이지만, 당장은 src/lib.rs의 아래쪽에 이 코드를 남겨두겠습니다.


src/main.rs 파일이 없기 때문에, cargo run 커맨드로 카고가 실행할 것이 없습니다. 따라서, 여기서는 라이브러리 크레이트의 코드를 컴파일하기 위해 cargo build를 사용할 것입니다.



## 모듈 정의 

러스트 내 모듈 정의는 모두 mod로 시작됩니다. mode 키워드 뒤에 모듈의 이름 network를 정의하고 중괄호({}) 안에 코드 블록이 옵니다.  이 블록 안의 모든 것은 이름공간 network 안에 있습니다. 

connect() 함수를 network 모듈 바깥의 스크립트에서 호출하려면  이름공간 문법 :: 을 사용해야 합니다. network::connect() 와 같이 사용합니다. 

같은 파일 내에 여러개의 모듈을 정의할 수 있습니다.

```rust
// mode 키워드 뒤에 모듈의 이름 network를 정의하고
// 중괄호({}) 안에 코드 블록이 온다. 
// 이 블록 안의 모든 것은 이름공간 network 안에 있다. 
mod network { 
    // 이 함수를 network 모듈 바깥의 스크립트에서 호출하려면
    // 이름공간 문법 :: 을 사용해야 한다.
    // network::connect() 와 같이 사용한다. 
    fn connect() {

    }
}


// 같은 파일 내에 여러개의 모듈을 정의할 수 있다.
mod client { 
    fn connect() {
    }
}
```

우리가 라이브러리를 만드는 중이기 때문에, 라이브러리의 시작 지점으로서 제공되는 파일은 src/lib.rs 입니다. 하지만 모듈을 만드는 것에 관하여 src/lib.rs는 특별할 것이 없습니다. 우리는 라이브러리 크레이트의 src/lib.rs 내에 모듈을 만드는 것과 똑같은 방식으로 바이너리 크레이트의 src/main.rs 내에도 모듈을 만들 수 있습니다.

사실 모듈 안에 다른 모듈을 집어넣는 것도 가능한데, 이는 여러분의 모듈이 커짐에 따라 관련된 기능이 잘 조직화 되도록 하는 한편 각각의 기능을 잘 나누도록 하는데 유용할 수 있습니다


* 바이너리 크레이트의 src/main.rs 내에도  모듈을 만들 수 있다.
* 모듈 안에 다른 모듈을 집어넣는 것도 가능하다.


client 모듈이 network 모듈의 내부 모듈이 되도록 mod network와 mod client의 위치를 바꿔 봅시다. 

```rust
mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}
```





이제 우리는 network::connect와 network::client::connect 함수를 갖게 되었습니다: 다시 말하지만, connect라는 이름의 두 함수는 서로 다른 이름공간에 있으므로 부딪힐 일이 없습니다.

이런 식으로 모듈들은 계층을 구성하게 됩니다. src/lib.rs의 내용은 가장 위의 층을 이루고, 서브 모듈들은 그보다 낮은 층에 있습니다.


```shell
communicator
 ├── network
 └── client
```


그리고 mod network와 mod client의 위치를 바꾼 위의 예시의 계층 구조는 다음과 같습니다.
```shell
communicator
 └── network
     └── client
```

위 계층 구조는 client가 network의 형제이기 보다는 자식임을 보여줍니다. 더 복잡한 프로젝트는 많은 수의 모듈을 갖고 있을 수 있고, 이들은 지속적인 트래킹을 위해 논리적으로 잘 조직화될 필요가 있을 것입니다.


