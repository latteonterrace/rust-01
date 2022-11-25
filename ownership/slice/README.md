# 슬라이스(Slice)


소유권을 갖지 않는 또다른 데이터 타입은 슬라이스입니다. 슬라이스는 여러분이 컬렉션(collection) 전체가 아닌 컬렉션의 연속된 일련의 요소들을 참조할 수 있게 합니다.


우리는 대괄호 내에 [starting_index..ending_index]를 특정한 범위를 이용하여 슬라이스를 만들 수 있는데, 여기서 starting_index는 슬라이스에 포함되는 첫번째 위치이고 ending_index는 슬라이스에 포함될 마지막 위치보다 1을 더한 값입니다

```rust
fn main() {

    let s = String::from("hello world");
    
    // 슬라이스는 소유권을 가지지 않음 
    //우리는 대괄호 내에 [starting_index..ending_index]를 특정한 범위를 이용하여 슬라이스를 만들 수  있는데, 여기서 
    // starting_index는 슬라이스에 포함되는 첫번째 위치이고 
    // ending_index는 슬라이스에 포함될 마지막 위치보다 1을 더한 값입니다
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);


    //  String의 마지막 바이트까지 포함한다면, 여러분은 끝의 숫자를 생략할 수 있습니다. 
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    println!("{}", slice);
    let slice = &s[3..];
    println!("{}", slice);

    // 시작 인덱스가 0인 경우, 0을 생략할 수 있습니다
    let hello = &s[..5];
    println!("{}", hello);

    // 전체 스트링의 슬라이스를 만들기 위해 양쪽 값을 모두 생략할 수 있습니다. 따라서 아래 두 줄의 표현은 동일합니다:
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len]; // 아래와 동일
    let slice = &s[..];

}//:
```

