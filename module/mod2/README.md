# 모듈 (2)


모듈은 계층적인 구조를 형성하는데, 여러분이 익숙하게 사용하고 있는 다른 구조와 매우 닮았습니다: 바로 파일 시스템이죠


## 모듈을 다른 파일로 옮기기

스트에서는 프로젝트를 잘게 나누기 위해 여러 개의 파일 상에서 모듈 시스템을 사용할 수 있어, 모든 것들이 src/lib.rs나 src/main.rs 안에 존재하지 않게할 수 있습니다. 

세 개의 모듈 client, network, network::server가 모두 src/lib.rs에 정의되어 있습니다. 

```rust
mod client {
    fn connect() {
    }
}

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

파일 src/lib.rs는 아래와 같은 모듈 계층을 갖고 있습니다:
```shell
communicator
 ├── client
 └── network
     └── server
```


만일 이 모듈들이 여러 개의 함수들을 갖고 있고, 이 함수들이 길어지고 있다면, 우리가 작업하고자 하는 코드를 찾으려고 이 파일을 스크롤 하기가 까다로워질 것입니다. client, network, 그리고 server 모듈을 src/lib.rs로부터 떼어내어 각자를 위한 파일들에 위치시켜 봅시다. 

먼저, client 모듈의 코드를 client 모듈의 선언 부분만 남겨두는 것으로 바꾸세요. 그러니까 여러분의 src/lib.rs는 아래와 같이 될 것입니다:


```rust
mod client;

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```


client 모듈을 선언하고 있지만, 코드 블록을 세미콜론으로 대체함으로써, 우리는 러스트에게 client 모듈의 스코프 내에 정의된 코드를 다른 위치에서 찾으라고 말하는 것입니다. 달리 말하면, mod client;라는 라인의 뜻은 이렇습니다:


```rust
mod client {
    // contents of client.rs
}
```

이제 모듈의 이름과 같은 이름을 가진 외부 파일을 만들 필요가 있습니다. client.rs 파일을 여러분의 src/ 디렉토리에 생성하고 여세요.  그런 뒤 아래와 같이 앞 단계에서 제거했던 client 모듈내의 connect 함수를 입력합니다. 



 **src/lib.rs 안에다 client 모듈을 mod를 이용하여 선언을 했기 때문에, 이 파일 안에는 mod 선언이 필요없다는 점을 기억하세요.** 이 파일은 단지 client 모듈의 내용물만 제공할 뿐입니다. 만일 mod client를 여기에 또 집어넣는다면, 이는 client 모듈 내에 서브모듈 client를 만들게 됩니다!



러스트는 기본적으로 src/lib.rs만 찾아볼줄 압니다. 만약에 더 많은 파일을 프로젝트에 추가하고 싶다면, src/lib.rs 내에서 다른 파일을 찾아보라고 러스트에게 말해줄 필요가 있습니다; 이는 mod client라는 코드가 왜 src/lib.rs 내에 정의될 필요가 있는지, 그리고 src/client.rs 내에는 정의될 수 없는지에 대한 이유입니다.


이제 몇 개의 컴파일 경고가 생기지만, 프로젝트는 성공적으로 컴파일 되어야 합니다. 우리가 바이너리 크레이트 대신 라이브러리 크레이트를 만드는 중이므로 cargo run 대신 cargo build를 이용해야 한다는 점을 기억해두세요:


이 경고들은 사용된 적이 없는 함수가 있음을 우리에게 알려줍니다. 지금은 이 경고들을 너무 걱정하지 마세요:



다음으로 같은 방식을 이용하여 network 모듈을 개별 파일로 추출해봅시다. src/lib.rs 안에서, 아래와 같이 network 모듈의 몸체를 지우고 선언부의 끝부분에 세미콜론을 붙이세요:

```rust
mod client;

mod network;
```

그리고나서 새로운 src/network.rs 파일을 만들어서 아래를 입력하세요:

```rust
fn connect() {
}

mod server {
    fn connect() {
    }
}
```

이 모듈 파일 내에는 mod 선언이 여전히 있음을 주목하세요; 이는 server가 network의 서브모듈로서 여전히 필요하기 때문입니다.


cargo build를 다시 실행시키세요. 성공! 여기 또 추출할만한 모듈이 하나 더 있습니다: server 말이죠. 이것이 서브모듈(즉, 모듈 내의 모듈)이기 때문에, 모듈을 파일로 추출해서 파일 이름을 모듈 이름으로 사용하는 전략은 사용하기 힘듭니다. 

어쨌든 시도해서 에러를 확인해보겠습니다. 먼저, src/network.rs 내에서 server 모듈의 내용물 대신에 mod server을 쓰세요:


```rust
fn connect() {
}

mod server;
```


그후 src/server.rs 파일을 만들고 추출해둔 server 모듈의 내용물을 입력하세요:
```rust
fn connect() {
}
```

cargo build를 실행해보면, 에러를 얻게 됩니다. 에러는 이 위치에 새로운 모듈을 선언할수 없다고 말해주며 src/network.rs의 mod server; 라인을 지적하고 있습니다.
이렇게 하는 경우 server.rs가 network의 하위 모듈이라고 rust 가 인식하지 않기 때문입니다. 




1. 부모 모듈의 이름에 해당하는, network라는 이름의 새로운 디렉토리를 만드세요.
2. src/network.rs 파일을 이 새로운 network 디렉토리 안으로 옮기고, 파일 이름을 src/network/mod.rs로 고치세요.
3. 서브모듈 파일 src/server.rs를 network 디렉토리 안으로 옮기세요.


우리의 모듈 레이아웃은 여전히 아래와 같이 되는데

```shell
communicator
 ├── client
 └── network
     └── server
```     
이에 대응하는 파일 레이아웃는 아래와 같이 생겼습니다:
```shell
├── src
│   ├── client.rs
│   ├── lib.rs
│   └── network
│       ├── mod.rs
│       └── server.rs
```


## 모듈 파일 시스템의 규칙
파일에 관한 모듈의 규칙을 정리해봅시다:

* 만일 foo라는 이름의 모듈이 서브모듈을 가지고 있지 않다면, foo.rs라는 이름의 파일 내에 foo에 대한 선언을 집어넣어야 합니다.
* 만일 foo가 서브모듈을 가지고 있다면, foo/mod.rs라는 이름의 파일에 foo에 대한 선언을 집어넣어야 합니다.


이 규칙들은 재귀적으로 적용되므로, foo라는 이름의 모듈이 bar라는 이름의 서브모듈을 갖고 있고 `bar는 서브모듈이 없다면, 여러분의 src 디렉토리 안에는 아래와 같은 파일들이 있어야 합니다:


이 모듈들은 부모 모듈의 파일에 mod 키워드를 사용하여 선언되어 있어야 합니다.