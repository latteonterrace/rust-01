# 모듈 (3)

아래와 같이 communicator를 라이브러리 크레이트(crate)로 만들었습니다. 

```shell
cargo new communicator --lib
```

그런다음, 아래와 같이 모듈을 정의했었습니다.


```shell
├── src
│   ├── client.rs
│   ├── lib.rs
│   └── network
│       ├── mod.rs
│       └── server.rs
```

이 라이브러리 크레이트는 다른 프로젝트에서 사용할 것이지만, 같은 프로젝트에서 사용하도록 바이너리 크레이트를 만들어서 테스트해보겠습니다. 

아래의 코드를 담은 src/main.rs 파일을 만듦으로서 같은 디렉토리에 라이브러리 크레이트와 마찬가지ㅏ로 바이너트 크레이트를 만들겠습니다. 

**src/main.rs**

```rust
extern crate communicator;
fn main() {
    communicator::client::connect();
}
```

그러면, 다음과 같이 main.rs가 lib.rs와 같은 디렉토리에 있는 것을 확인할 수 있습니다. 

```shell
├── src
│   ├── client.rs
│   ├── lib.rs  # 라이브러리 크레이트
│   ├── main.rs # 바이너리 크레이트
│   └── network
│       ├── mod.rs
│       └── server.rs
```


communicator 라이브러리 크레이트를 가져오기 위해 extern crate 명령어를 사용합니

* 우리의 패키지는 이제 두 개의 크레이트를 담고 있습니다
* 카고는 src/main.rs를 바이너리 크레이트의 루트 파일로 취급하는데, 
*  바이너리 크레이트는 src/lib.rs가 루트 파일인 이미 있던 라이브러리 크레이트는 별개입니다.


우리가 만들어왔던 모든 모듈들은 communicator라는 이름을 갖는 모듈 내에 있습니다. 크레이트의 최상위 모듈을 루트 모듈 (root module) 이라 부릅니다.


이 시점에서 cargo build를 실행하면 경고들 이후에 에러를 표시할 것입니다.  client 모듈이 비공개(private)이기 때문입니다 


**러스트의 모든 코드의 기본 상태는 비공개입니다**: 즉, 다른 사람은 이 코드를 사용할 수 없습니다. 만일 여러분의 프로그램 내에서 비공개 함수를 이용하지 않는다면, 여러분의 프로그램이 그 함수를 이용할 수 있는 유일한 곳이기 때문에, 러스트는 그 함수가 사용된 적이 없다며 경고해줄 것입니다.



## 함수를 공개로 만들기 


러스트에게 어떤 것을 공개하도록 말하기 위해서는, 공개하길 원하는 아이템의 선언 시작 부분에 pub 키워드를 추가합니다. 

src/lib.rs을 수정하여 client 모듈을 공개로 만드세요:

src/lib.rs
```rust
pub mod client;
mod network;
```

pub 키워드는 mod 바로 전에 위치합니다 다시 빌드하면 connect 함수가 private라는 오류가 표시됩니다.  src/client.rs를 수정해서 공개로 만들어야 합니다. 

src/client.rs
```rust
pub fn connect() {
}
```
다 시 빌드하면 경고가 여전히 나오는데 다음과 같이 수정합니다.

src/lib.rs
```rust
pub mod client;
pub mod network;
```



## 비공개 규칙
종합해보면, 아이템 가시성에 관한 규칙은 다음과 같습니다:

* 만일 어떤 아이템이 공개라면, 이는 부모 모듈의 어디에서건 접근 가능합니다.
* 만일 어떤 아이템이 비공개라면, 같은 파일 내에 있는 부모 모듈 및 이 부모의 자식 모듈에서만 접근 가능합니다.





