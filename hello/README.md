# 01. 시작하기 


## Hello World!

[Hello World](https://rinthel.github.io/rust-lang-book-ko/ch01-02-hello-world.html)

main.rs 파일을 생성합니다. 
```rust
fn main() {
    println!("Hello, world!");
}
```

빌드하고 실행합니다. 
```rust
> rustc main.rs
> .\main.exe
Hello, world!
```


* .rs 소스 파일
* .pdb 디버깅 정보를 담고 있는 파일
* .exe 실행파일 



## Hello Cargo 
cargo를 이용하여 프로젝트를 생성합니다. 

[Hello Cargo](https://rinthel.github.io/rust-lang-book-ko/ch02-00-guessing-game-tutorial.html) 

```shell
$ cargo new rust-01 --bin
$ cd rust-01
```

* --bin 바이너리용 프로젝트 

Cargo.toml 파일을 열어 라이브러리를 추가합니다. 

```toml
[package]
name = "guest"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

다음 명령을 사용하여 빌드하고 실행해 봅니다. 
```shell 
cargo run
```

target/ 디렉터리에 빌드된 바이너리가 생성됩니다. 







