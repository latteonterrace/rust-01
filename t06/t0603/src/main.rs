fn match_basic_string() {
    // 문자열로 패턴 매칭
    let name = "John";
    match name {
        "Hong" => println!("Hello, Hong!"),
        "Kim" => println!("Hello, Kim!"),
        _ => println!("Hello, Stranger!"), // _ 는 모든 값
    }
}

fn match_basic_string2() {
    let name = "John";
    // 괄호 사용 , 여러줄로 표현
    match name {
        "John" => {
            println!("4");
            println!("green");
            println!("papaya");
        }
        "Annie" => {
            println!("3");
            println!("blue");
            println!("blueberry");
        }
        "Michael" => {
            println!("2");
            println!("yellow");
            println!("guava");
        }
        "Gabrielle" => {
            println!("1");
            println!("purple");
            println!("apple");
        }
        _ => {
            println!("0");
            println!("orange");

            println!("orange");
        }
    }
}

fn match_basic_string3() {
    let name = "John";

    // 값을 반환
    let result = match name {
        "John" => "papaya",
        "Annie" => "blueberry",
        "Michael" => "guava",
        "Gabrielle" => "apple",
        _ => "orange",
    };

    println!("{}", result);
}

#[derive(Debug)]
enum Vehicle {
    Car,
    MotorCycle,
    Bicycle,
}

fn match_basic_enum1() {
    // Using enums in Rust
    // 문자열, 숫자 및 기타 데이터 유형과 패턴 일치를 수행할 수 있는 것처럼 열거형 Variant도 일치시킬 수 있습니다.
    let vehicle = Vehicle::Car;
    match vehicle {
        Vehicle::Car => println!("I have four tires"),
        Vehicle::MotorCycle => println!("I have two tires and run on gas"),
        Vehicle::Bicycle => println!("I have two tires and run on your effort"),
    }
}

enum Vehicle2 {
    Car(String),
    MotorCycle(String),
    Bicycle(String),
}

fn match_basic_enum2() {
    //Adding data to enum variants

    // enum을 변수에 할당
    let vehicle = Vehicle2::Car("red".to_string());
    // 변수를 match 문에 두고
    match vehicle {
        // 각각의 열거형의 variant의 내용을 변수로 분해
        // 패턴의 오른쪽의 코드블록에서 분해된 변수를 사용할 수 있다.
        Vehicle2::Car(color) => println!("I am {} and have four tires", color),
        Vehicle2::MotorCycle(color) => println!("I am {} and have two tires and run on gas", color),
        Vehicle2::Bicycle(color) => {
            println!("I am {} and have two tires and run on your effort", color)
        }
    }
}

// 문자와 숫자를 값으로 갖는 열거형
enum Info {
    Name(String),
    Age(u8),
}

fn match_basic_enum3() {
    // 문자와 숫자를 값으로 갖는 enum 사용
    let info = Info::Name("John".to_string());
    // 아래 코드 실행할려면 여기에 주석을 달아야 함
    // match info {
    //     Info::Name(name) => println!("My name is {}", name),
    //     Info::Age(age) => println!("I am {} years old", age),
    // }

    let res = match info {
        Info::Name(name) => name,
        Info::Age(age) => age.to_string(), // name과 타입이 달라서 맞추어야 함
    };
    println!("{}", res);
}

// 열거형의 variant를 반환하는 함수
fn divide(numerator: i32, denominator: i32) -> Result<i32, String> {
    if denominator == 0 {
        return Err("Cannot divide by zero".to_string());
    } else {
        return Ok(numerator / denominator);
    }
}

fn match_basic_result1() {
    // Result and Option enums
    // Result enum
    match divide(103, 2) {
        Ok(solution) => println!("The answer is {}", solution),
        Err(error) => println!("Error: {}", error),
    }

    let res = match divide(103, 2) {
        Ok(solution) => solution.to_string(),
        Err(error) => error,
    };
    println!("{}", res);

    let number = divide(103, 2);
    if number.is_err() {
        // 에러인가?
        println!("Error: {}", number.unwrap_err()); // 에러 메시지를 가져온다.
    } else if number.is_ok() {
        println!("The answer is {}", number.unwrap()); // 정상적인 값이면 값을 가져온다.
    }
}

fn divide2(numerator: i32, denominator: i32) -> Option<i32> {
    if denominator == 0 {
        return None;
    } else {
        return Some(numerator / denominator);
    }
}

fn match_basic_result2() {
    match divide2(103, 0) {
        Some(solution) => println!("The answer is {}", solution),
        None => println!("Your numerator was about to be divided by zero :)"),
    }

    let number = divide2(103, 0);

    if number.is_some() {
        println!("number is: {}", number.unwrap());
    } else {
        println!("Is the result none: {}", number.is_none());
        println!("Result: {}", number.unwrap_or(0));
    }
}

// match
fn match_head() {
    let number = 19;
    match number {
        // 단일값에 매치된다. 1에 매치
        1 => println!("One!"),
        // 여러개의 값에 매치된다. 2, 3, 5, 7, 11에 매치
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 13부터 19까지 매치된다. 포함된 범위(inclusive range)에 매치된다.
        13..=19 => println!("A teen"), // exclude range 패턴은 아직 지원하지 않음
        // 13..19 => println!("A teen"), // exclude range 패턴은 아직 지원하지 않음
        // 매치되지 않는 모든 경우에 매치된다.
        _ => println!("Ain't special"),
    }
}

// match - boolean
fn match_boolean() {
    let boolean = true;
    // match는 표현식이다.
    let binary = match boolean {
        // 매치의 각 암들은 가능한 값들을 모두 커버한다.
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}

// destructring - tuple
fn match_tuple() {
    // 튜플은 매치에서 다음과 같이 분해될 수 있다.
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);
    // 매치는 튜플을 분해하는데 사용될 수 있다.
    match triple {
        // 두번째와 세번재 요소를 분해한다.
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        // 처음은 중요하고 나머지는 중요하지 않다
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        // 마지막이 2이고 나머지는 중요하지 않다.
        (.., 2) => println!("last is `2` and the rest doesn't matter"),
        // 처음이 3, 마지막은 4, 나머지는 중요하지 않다
        (3, .., 4) => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        // `..` can be used to ignore the rest of the tuple
        // `..`는 튜플의 나머지를 무시하는데 사용될 수 있다.
        _ => println!("It doesn't matter what they are"),
        // `_`는 값을 변수에 바인딩하지 않는다는 것을 의미한다.
    }
}

// array and slices
fn match_array_slices() {
    // 배열의 값을 바꾸거나 슬라이스로 만들어보자.
    let array = [9, -2, 6];

    match array {
        // 두번째와 세번째 요소를 각각의 변수(respective variables)에 바인딩한다.
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // _ 가 있는 단일값들은 무시할 수 있다.
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // 일부를 바인딩하고 나머지는 무시할 수 있다.
        // 첫번째 값 -1, 두번째는 바인딩, 나머지는 무시
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // 아래 코드는 컴파일되지 않는다.
        // [-1, second] => ...

        // Or store them in another array/slice (the type depends on
        // that of the value that is being matched against)
        // 또는 다른 배열/슬라이스에 저장할 수 있다.
        // (타입은 매치되는 값에 따라 달라진다.)
        // @ 는 해당 값이 패턴과 매치되는지 확인하는 동시에 해당 값을 갖는 변수를 생성할 수 있도록 해준다.
        // https://rinthel.github.io/rust-lang-book-ko/ch18-03-pattern-syntax.html
        //   let array = [3, -2, 6]; 으로 정의하면
        // 결과가 이렇게 나온다. ->  array[0] = 3, array[1] = -2 and the other elements were [6]
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        // 이 패턴들을 결합하면, 예를 들어, 첫번째와 마지막 값을 바인딩하고 나머지는 하나의 배열에 저장할 수 있다.
        [first, middle @ .., last] => println!(
            "=> array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}

// `allow` required to silence warnings because only
// one variant is used.
#[allow(dead_code)]
enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

// destructuring enum
fn match_enum() {
    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");
    // match 를 사용하여 enum 을 destructuring 할 수 있다.
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
        // Don't need another arm because all variants have been examined
    }
} //:

fn match_pointer_ref() {
    // Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned.
    let reference = &4;

    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}

// 참조에 대한 이해
fn get_ref() {
    // https://blog.thoughtram.io/references-in-rust/

    // 1. 불변 변수는 값의 재 할당이 불가능하다.
    let i = 0;
    println!("i = {}", i); // i = 0
                           // 불변 참조 변수 i에 두 번 할당할 수 없다.
                           // 오류 발생한다.
                           // i = 2; // error: cannot assign twice to immutable variable `i`

    // 2. 변수의 값을 재할당하려면 가변 변수를 사용해야 한다.
    // 불변(immutable) 변수를 가변(mutable) 변수로 정의해야 수정할 수 있다.
    let mut i = 0;
    i = 2; // ok

    // 3. 가변 참조 변수의 값을 변경하려면 *를 통해 역참조(dereference) 해야 한다.
    // i는 변경할 수 있다
    let mut i: i32 = 2; // i32 타입의 변수 i 를 선언하고 2로 초기화한다.
                        // j는 i의 가변 참조, 다음과 같이 정의한다.
    let j: &mut i32 = &mut i; // j 는 i 의 참조이다.
                              // 가변 참조 변수의 값을 변경하려면 *를 통해 역참조해야 한다.(dereference)
    *j = 3; // j 를 통해 i 의 값을 변경한다.
            // i 값이 변경이 되었는지 확인한다.
    println!("i = {}", i); // i = 3
                           // 가변 참조에 빌려주고 나면 가변 변수 변경 불가하다
                           // i가 가변 변수이기 때문에 변경이 가능할 것 같지만
                           // j에 빌려주었기 때문에 변경이 불가하다
                           // 아래는 오류가 발생한다고 하는데 정상적으로 아래 코드들이 실행 되었다.
    i = 4;
    println!("i = {}", i); // i = 4
}

fn is_ten(val: &i32) -> bool {
    *val == 10
}

fn deref() {
    let x = 10;
    let r = &x;
    let rr = &r; // `rr` is a `&&x`

    if is_ten(rr) {
        println!("Same!");
    }
}

struct Foo(String);

fn ref_test1() {
    let foo = Foo(String::from("Hello, World"));
    match foo {
        Foo(a) => println!("{}", a),
        _ => panic!("Unreachable pattern"),
    }

    //println!("{}", foo.0);
}

// 튜플을 분해하는 것은 간단하다. 튜플의 각 필드에 새 변수 이름을 할당하기만 하면 된다.
fn destruction_tuple_struct_tuple() {
    let (first, second) = (1, 2);
    println!("first = {}, second = {}", first, second);
    // first = 1, second = 2
}

fn destruction_tuple_struct() {
    // 튜플 구조체를 분해하는 것은 튜플을 분해하는 것과 동일하지만 구조체 이름이 추가된다.
    struct TupleStruct(&'static str, i32);
    let my_tuple_struct = TupleStruct("foo", 123);
    let TupleStruct(foo, num) = my_tuple_struct;
    println!("foo = {}, num = {}", foo, num); // foo = foo, num = 123
}

// 구조체를 분해하는 것은 튜플 구조체를 분해하는 것과 비슷하다.
fn destruction_struct() {
    struct ArabianNights {
        name: String,
        stories: usize,
    }
    let teller = ArabianNights {
        name: "Scheherazade".into(),
        stories: 1001,
    };
    {
        let ArabianNights { name, stories } = teller;
        println!("{} told {} stories", name, stories);
        // Scheherazade told 1001 stories
    }
}

// .. 연산자는 구조 분해 시 일부 필드를 무시하는 데 사용할 수 있다
fn destruction_struct_partial() {
    struct ArabianNights {
        name: String,
        stories: usize,
    }

    let teller = ArabianNights {
        name: "Scheherazade".into(),
        stories: 1001,
    };
    let ArabianNights { name, .. } = teller;
    println!("{} survived by her wits", name);
}

fn ref1() {
    // i32 타입의 참조를 할당한다.
    // `&`는 할당된 참조이다.
    let reference = &4;

    match reference {
        // reference가  &val에 대한 매치된 패턴이라면, 아래와 같은 비교가 된다.
        // `&i32`
        // `&val`
        //'&'이 제거되면 `i32`가 `val`에 할당되어야 한다.
        &val => println!("Got a value via destructuring: {:?}", val),
        // Got a value via destructuring: 4
    }
}

fn ref2() {
    // i32 타입의 참조를 할당한다.
    // `&`는 할당된 참조이다.
    let reference = &4;

    // `&`를 피하기 위해서는 매치하기 전에 참조를 역참조해야 한다.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }
}


fn ref3() {

    // 참조로 시작하지 않는다면 어떻게 될까? `reference`는 `&`로 시작했다.
    // 오른쪽이 이미 참조였기 때문이다.
    // 이것은 오른쪽이 참조가 아니기 때문에 참조가 아니다.
    let _not_a_reference = 3;

    // 러스트는 정확히 이 목적을 위해 `ref`를 제공한다. 요소에 대한 참조가 생성되도록 할당을 수정한다.
    // this reference is assigned.
    // 이 참조는 할당된다.
    let ref _is_a_reference = 3;
        
    // 따라서 참조없이 2 개의 값을 정의하여 
    // `ref`와 `ref mut`을 통해 참조를 가져올 수 있다. 
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    // `ref` 키워드를 사용하여 참조를 만든다.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }
    // `ref mut`을 비슷하게 사용한다.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can add anything to it.
            // 참조를 얻었다. 무언가를 추가하기 전에 역참조해야 한다.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}
    

fn match_des_structs() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // 구조체의 값을 변경하여 어떻게 되는지 확인해보자.
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),
        // 구조체를 분해하고 변수의 이름을 바꿀 수 있다.
        // 순서는 중요하지 않다.
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // 또한 일부 변수를 무시할 수도 있다.
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // 이것은 오류를 발생시킨다: 패턴은 필드 `x`를 언급하지 않는다.
        //Foo { y } => println!("y = {}", y),
    }
}



// guards 
fn guards1(){ 
    enum Temperature {
        Celsius(i32),
        Fahrenheit(i32)
    }

    let temperature = Temperature::Celsius(35);
    match temperature {
        // if 조건이 부분이 guard이다.
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
    }
}


fn guards2() {
    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        // 아래 구문이 없으면 오류 발생 주석을 해제해야 함 
        _ => unreachable!("Should never happen."),
        
    }
}

fn main() {
    // match_head();
    // match_boolean();
    // match_tuple();
    // match_array_slices();
    // match_enum();
    // match_pointer_ref();
    // get_ref();
    // deref();
    // ref_test1();
    // destructring1();
    // destruction_tuple_struct();
    // destruction_struct();
    // destruction_struct_partial();

    // ref1();
    // ref2();
    // match_des_structs();
    // guards1();
    guards2();
    
}
