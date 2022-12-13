# 모듈 
라이브러리 크레이트 만들기 
```shell
$ cargo new comm --lib 
```
main.rs가 없기 때문에 cargo build 사용
```shell
cargo build
```

### 모듈 정의

하나의 파일에 여러개의 모듈을 정의할 수 있다.

**src/lib.rs**
```rust
mod network {
    fn connect() {
    }
}

mod client {
    fn connect() {
    }
}
```


호출할 때는 다음과 같이 :: 사용한다. 
```rust
fn main() {
    network::connect();
    client::connect();
}
```

**중첩 모듈**    

다음과 같이 중첩되게 모듈을 정의할 수 있다. 

**src/lib.rs**   
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
이제 network::connect와 network::client::connect 함수를 갖게 되었다. 


## 모듈 파일 분리

**src/lib.rs**

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

mode client 사용하여 다음과 같이 변경한다. 
**iib/client.rs**
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


client.rs 파일을 다음과 같이 작성한다.  위의 코드에서 mod client; 의 client는 이제 파일명이 모듈명이다. 

**lib/client.rs**
```rust
// client 모듈을 mod를 이용하여 선언을 했기 때문에
// 이 파일 안에는 mod 선언이 필요없다
fn connect() {
}
``` 

network 모듈을 개별 파일로 추출해보자. 
**src/lib.rs**
```rust
mod client;
mod network;
```
**src/newtwork.rs** 
```rust
fn connect() {
}

mod server {
    fn connect() {
    }
}
```

server 모듈을 분리해보자. 
구조를 다음과 같이 변경한다. 
```shell
├── src
│   ├── client.rs
│   ├── lib.rs
│   └── network
│       ├── mod.rs
│       └── server.rs
```


**src/lib.rs**
```rust
mod client; 
mod network;
```

**src/client.rs**
```rust
fn connect() {
}
```
**src/network/mod.rs**   
```rust
mod server; 

fn connect() {
}
```
src/network/server.rs
```rust
fn connect() {
}
```



## 모듈 파일 시스템의 규칙
파일에 관한 모듈의 규칙을 정리해봅시다:

* 만일 foo라는 이름의 모듈이 서브모듈을 가지고 있지 않다면, foo.rs라는 이름의 파일 내에 foo에 대한 선언을 집어넣어야 합니다.
* 만일 foo가 서브모듈을 가지고 있다면, foo/mod.rs라는 이름의 파일에 foo에 대한 선언을 집어넣어야 합니다.


## 가시성 
바이너리 크레이트를 만든다.  src/main.rs 파일을 만들고 다음과 같이 작성한다. 

**src/main.rs**
```rust
extern crate comm;

fn main() {
    comm::client::connect();
}
```
comm 라이브러리 크레이트를 가져오기 위해 extern crate 명령어를 사용한다. 
아래 명령을 실행하여 comm 크레이트를 만든 것을 기억하자. 
```shell
cargo build comm --lib
```
Cargo.toml에 보면 다음과 같이 되어 있다. 
```shell
[package]
name = "comm"
version = "0.1.0"
edition = "2021"
```

> comm 라이브러리 밖의 크레이트가 안을 들여다 보는 시점에서, 우리가 만들어왔던 모든 모듈들은 comm라는 이름을 갖는 모듈 내에 있습니다. 크레이트의 최상위 모듈을 루트 모듈 (root module) 이라 부릅니다.

빌드하면 다음과 같은 오류가 있다. 
```shell
 communicator::client::connect(); 
              ^^^^^^
              private module
```              

###  공개로 만들기 
모듈을 공개로 만들기 위해서는 pub 키워드를 사용한다.
```rust
pub mod client; 
pub mod network;
```
함수를 공개로 만들기 위해서는 pub 키워드를 사용한다.
```rust
pub fn connect() {
}
```

## use 사용 
comm::client::connect();와 같이 connect() 함수를 사용한다. 
**src/main.rs**
```rust
extern crate comm;

fn main() {
  comm::client::connect();
}
```
use 키워드를 사용하여 다음과 같이 사용할 수 있다. 
**src/main.rs**
```rust
extern crate comm;

use comm::client;

fn main() {
  // 간단히 호출 
  client::connect();
}
```

### *를 이용한 모두(glob) 가져오기

'*' 문법을 이용하여 간단히 가져올 수 있다. 
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

### super를 사용하여 부모 모듈에 접근하기
```rust
super::client::connect();
```





















