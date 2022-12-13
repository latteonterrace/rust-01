
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

