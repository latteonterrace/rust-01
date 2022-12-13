# panic! 복구 불가능한 에러

가끔씩 여러분의 코드에서 나쁜 일이 일어나고, 이에 대해 여러분이 할 수 있는 것이 없을 수도 있습니다. 이러한 경우를 위하여 러스트는 panic! 매크로를 가지고 있습니다. 이 매크로가 실행되면, 여러분의 프로그램은 실패 메세지를 출력하고, 스택을 되감고 청소하고, 그 후 종료됩니다. 




> **panic!에 응하여 스택을 되감거나 그만두기**
> 기본적으로, panic!이 발생하면, 프로그램은 되감기(unwinding) 를 시작하는데, 이는 러스트가 패닉을 마주친 각 함수로부터 스택을 거꾸로 훑어가면서 데이터를 제거한다는 뜻이지만, 이 훑어가기 및 제거는 일이 많습니다. 다른 대안으로는 즉시 그만두기(abort) 가 있는데, 이는 데이터 제거 없이 프로그램을 끝내는 것입니다. 프로그램이 사용하고 있던 메모리는 운영체제에 의해 청소될 필요가 있을 것입니다. 여러분의 프로젝트 내에서 결과 바이너리가 가능한 작아지기를 원한다면, 여러분의 Cargo.toml 내에서 적합한 [profile] 섹션에 panic = 'abort'를 추가함으로써 되감기를 그만두기로 바꿀 수 있습니다. 예를 들면, 여러분이 릴리즈 모드 내에서는 패닉 상에서 그만두기를 쓰고 싶다면, 다음을 추가하세요:
> 
> [profile.release]
panic = 'abort'



단순한 프로그램 내에서 panic! 호출을 시도해 봅시다:


```rust
fn main() {
    panic!("crash and burn");
}
```


이 프로그램을 실행하면, 다음과 같은 것을 보게 될 것입니다:

```shell
PS G:\github\latteon\rust-01\error\panic> cargo run
   Compiling panic v0.1.0 (G:\github\latteon\rust-01\error\panic)
    Finished dev [unoptimized + debuginfo] target(s) in 1.02s
     Running `target\debug\panic.exe`
thread 'main' panicked at 'crash and burn', src\main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\panic.exe` (exit code: 101)
```
panic!의 호출이 마지막 세 줄의 에러 메세지를 야기합니다. 첫 번째 줄은 우리의 패닉 메세지와 소스 코드에서 패닉이 발생한 지점을 보여줍니다: src/main.rs:2는 src/main.rs 파일의 두 번째 줄을 가리킵니다.


문제를 일으킨 코드 부분을 발견하기 위해서 panic! 호출이 발생된 함수에 대한 백트레이스(backtrace)를 사용할 수 있습니다. 백트레이스가 무엇인가에 대해서는 뒤에 더 자세히 다를 것입니다.


## panic! 백트레이스 사용하기


다른 예를 통해서, 우리 코드가 직접 매크로를 호출하는 대신 우리 코드의 버그 때문에 panic! 호출이 라이브러리로부터 발생될 때는 어떻게 되는지 살펴봅시다. Listing 9-1은 벡터 내의 요소를 인덱스로 접근 시도하는 코드입니다:

Filename: src/main.

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

여기서 우리는 벡터의 100번째 요소(0부터 시작하여 100번째)에 접근하기를 시도하고 있지만, 벡터는 오직 3개의 요소만 가지고 있습니다. 이러한 상황이면 러스트는 패닉을 일으킬 것입니다. []를 사용하는 것은 어떤 요소를 반환하기를 가정하지만, 유효하지 않은 인덱스를 넘기게 되면 러스트가 반환할 올바른 요소는 없습니다.


```shell
PS G:\github\latteon\rust-01\error\panic> cargo run
   Compiling panic v0.1.0 (G:\github\latteon\rust-01\error\panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.52s
     Running `target\debug\panic.exe`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src\main.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\panic.exe` (exit code: 101)
```

RUST_BACKTRACE 환경 변수를 설정하여 에러의 원인이 된 것이 무엇인지 정확하게 백트레이스할 수 있다고 말해주고 있습니다. 백트레이스 (backtrace) 란 어떤 지점에 도달하기까지 호출해온 모든 함수의 리스트를 말합니다. 

러스트의 백트레이스는 다른 언어들에서와 마찬가지로 동작합니다: 백트레이스를 읽는 요령은 위에서부터 시작하여 여러분이 작성한 파일이 보일 때까지 읽는 것입니다. 그곳이 바로 문제를 일으킨 지점입니다. 여러분의 파일을 언급한 줄보다 위에 있는 줄들은 여러분의 코드가 호출한 코드입니다; 밑의 코드는 여러분의 코드를 호출한 코드입니다. 이 줄들은 핵심(core) 러스트 코드, 표준 라이브러리, 혹은 여러분이 이용하고 있는 크레이트를 포함하고 있을지도 모릅니다. 백트레이스를 얻어내는 시도를 해봅시다: Listing 9-2는 여러분이 보게 될 것과 유사한 출력을 보여줍니다:


```shell
$ RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 100', /stable-dist-rustc/build/src/libcollections/vec.rs:1392
stack backtrace:
   1:     0x560ed90ec04c - std::sys::imp::backtrace::tracing::imp::write::hf33ae72d0baa11ed
                        at /stable-dist-rustc/build/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x560ed90ee03e - std::panicking::default_hook::{{closure}}::h59672b733cc6a455
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:351
   3:     0x560ed90edc44 - std::panicking::default_hook::h1670459d2f3f8843
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:367
   4:     0x560ed90ee41b - std::panicking::rust_panic_with_hook::hcf0ddb069e7abcd7
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:555
   5:     0x560ed90ee2b4 - std::panicking::begin_panic::hd6eb68e27bdf6140
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:517
   6:     0x560ed90ee1d9 - std::panicking::begin_panic_fmt::abcd5965948b877f8
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:501
   7:     0x560ed90ee167 - rust_begin_unwind
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:477
   8:     0x560ed911401d - core::panicking::panic_fmt::hc0f6d7b2c300cdd9
                        at /stable-dist-rustc/build/src/libcore/panicking.rs:69
   9:     0x560ed9113fc8 - core::panicking::panic_bounds_check::h02a4af86d01b3e96
                        at /stable-dist-rustc/build/src/libcore/panicking.rs:56
  10:     0x560ed90e71c5 - <collections::vec::Vec<T> as core::ops::Index<usize>>::index::h98abcd4e2a74c41
                        at /stable-dist-rustc/build/src/libcollections/vec.rs:1392
  11:     0x560ed90e727a - panic::main::h5d6b77c20526bc35
                        at /home/you/projects/panic/src/main.rs:4
  12:     0x560ed90f5d6a - __rust_maybe_catch_panic
                        at /stable-dist-rustc/build/src/libpanic_unwind/lib.rs:98
  13:     0x560ed90ee926 - std::rt::lang_start::hd7c880a37a646e81
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:436
                        at /stable-dist-rustc/build/src/libstd/panic.rs:361
                        at /stable-dist-rustc/build/src/libstd/rt.rs:57
  14:     0x560ed90e7302 - main
  15:     0x7f0d53f16400 - __libc_start_main
  16:     0x560ed90e6659 - _start
  17:                0x0 - <unknown>
```

windows에서는 다음과 같이 합니다. 
cmd를 사용할 떄:
```shell
set RUST_BACKTRACE=1
set RUST_BACKTRACE=full
```
powershell을 사용할 때:
```shell
$env:RUST_BACKTRACE=1
$env:RUST_BACKTRACE="full"
```


