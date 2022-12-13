
// communicator 라이브러리 크레이트를 가져오기 위해 extern crate 명령어를 사용
extern crate communicator;

fn main() {
    // cargo build 명령을 실행하면 오류 발생 
    // client가 비공개 이기 때문이다
    // 러스트의 모든 코드의 기본 상태는 비공개입니다
    // 공개하려면 pub 키워드를 사용해야 합니다
    communicator::client::connect();
}


// 카고는 src/main.rs를 바이너리 크레이트의 루트 파일로 취급하는데, 
// 이 바이너리 크레이트는 src/lib.rs가 루트 파일인 이미 있던 라이브러리 크레이트는 
// 별개입니다


// communicator 라이브러리 밖의 크레이트가 안을 들여다 보는 시점에서, 우리가 만들어왔던 
// 모든 모듈들은 communicator라는 이름을 갖는 모듈 내에 있습니다. 
// 크레이트의 최상위 모듈을 루트 모듈 (root module) 이라 부릅니다.


