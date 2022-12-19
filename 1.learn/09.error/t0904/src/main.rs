// use std::fs::File;
// use std::io::ErrorKind;
// use std::io::Read;


// // Result<T,E> 타입의 값을 반환하는데 
// // T는 String, E는 io::Error 타입이다.
// // 성공하면 호출한 코드는 String을 담은 값을 받을 것이고 , 
// // 실패하면 io::Error 인스턴스를 담은 Err 값을 받을 것이다.
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     // 여기도 성공 실패 처리 
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     // 여기도 성공 실패 처리 
//     // 우리는 모든 성공 혹은 에러 정보를 위로 전파하여 호출하는 코드가 적절하게 처리를 하도록 합니다.
//     // 러스트에서 에러를 전파하는 패턴은 너무 흔하여 러스트에서는 이를 더 쉽게 해주는 물음표 연산자 ?를 제공합니다.
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }



// fn read_username_from_file() -> Result<String, io::Error> {
//     // Result 값 뒤의 ?는 Result 값을 다루기 위해 정의했던 match 표현식과 거의 같은 
//     // 방식으로 동작하게끔 정의되어 있습니다. 만일 Result의 값이 Ok라면, Ok 내의 값이 이 표현식으로부터 
//     // 얻어지고 프로그램이 계속됩니다. 만일 값이 Err라면, 우리가 return 키워드를 사용하여 에러 값을 호출하는 
//     // 코드에게 전파하는 것과 같이 전체 함수로부터 Err 내의 값이 반환될 것입니다.
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }


// // ?는 Result를 반환하는 함수에서만 사용될 수 있습니다
// fn read_username_from_file() -> Result<String, io::Error> {
//     // 새로운 String을 만들어 s에 넣는 부분을 함수의 시작 부분으로 옮겼습니다.
//     let mut s = String::new();

//     //? 뒤에 바로 메소드 호출을 연결하는 식으로 (chaining) 이 코드를 더 줄일 수도 있습니다:
//     // f 변수를 만드는 대신, File::open("hello.txt")?의 결과 바로 뒤에 read_to_string의 호출을 연결시켰습니다.
//     // 호출의 끝에는 여전히 ?가 남아있고, File::open과 read_to_string이 모두 에러를 반환하지 않고 성공할 때 s 안의 사용자 이름을 담은 Ok를 여전히 반환합니다. 
//     File::open("hello.txt")?.read_to_string(&mut s)?;

//     Ok(s)
// }






// fn select_and_update() -> Result<String, MyError> {
//     let user = select_user();
//     let user = match user {
//         Ok(user) => user,
//         Err(e) => return Err(e),
//     };
//     match update_view_count(&user) {
//         // _ 패턴은 어떠한 값과도 매칭 될 것이다 
//         // _는 그전에 명시하지 않은 모든 가능한 경우에 대해 매칭 
//         Ok(_) => Ok(user),
//         Err(e) => Err(e),
//     }
// }



// fn compare(s1: &str, s2: &str) -> bool {
//     if s1 == s2 {
//         true
//     } else {
//         false
//     }
// }


// 에러 코드를 enum으로 정의 
#[derive(Debug)]
enum MyError {
    NotFoundError,
    PermissionDeniedError,
    UpdateError, 
}


// Result<String, MyError>는 String을 반환하거나 MyError를 반환합니다.
fn select_user() -> Result<String, MyError> {
    //Err(MyError::NotFoundError)
    Ok(String::from("Tom"))
}

// fn update_view_count(user: &String) -> Result<String, MyError> {
//     let temp_string = String::from("Tom");
//     if &temp_string == user {
//         Ok(String::from("Tom Updated"))
//     }else {
//         Err(MyError::UpdateError)
//     }
// }

// fn select_and_update() -> Result<String, MyError> {
//     match select_user() {
//         Ok(_) => Ok(String::from("Tom")), 
//         Err(e) => Err(e),
//     }
// }

// fn select_and_update() -> Result<String, MyError> {
//     let user = select_user()?;
//     match user {
//         Ok(u) => u,
//         Err(e) => Err(e),
//     }
// }


enum IpAddrKind {
    V4(u32),
    V6(String),
}


fn main() {
    let home = IpAddrKind::V6(String::from("IV6"));
    match home { 
        IpAddrKind::V4(_) => println!("V4"),
        IpAddrKind::V6(_) => println!("V6"),
    }


    // let user = select_user();
    // match user {
    //     Ok(user) => println!("user: {}", user),
    //     Err(e) => println!("Error: {:?}", e),
    // }
}

// fn main() {
//     let name = String::from("Tom");
//     let res = match update_view_count(&name) {
//         // ok의 모든 값에 대응 
//         // _ 패턴은 어떠한 값과도 매칭 될 것이다 
//         // _는 그전에 명시하지 않은 모든 가능한 경우에 대해 매칭         
//         Ok(_) => &name,
//         Err(e) => {
//             println!("Error: {:?}", e);
//             return;
//         }
//     };

    
//     println!("res: {}", res);
//     println!("name: {}", name);
// }





// lect_all() -> Result<String, MyError> {
//     let user = select_user()?;
//     // match user {
//     //     Ok(user) => Ok(user),
//     //     Err(MyError::NotFoundError) => Err(MyError::NotFoundError),
//     //     Err(MyError::PermissionDeniedError) => Err(MyError::PermissionDeniedError),
//     // }
//     Ok(user)
// }



// fn main() {


//     // let s1 = String::from("hello");
//     // let s2 = String::from("hello");
//     // let ret = compare(&s1, &s2);
//     // println!("ret: {}", ret);

//     // // eq 
//     // println!("eq {}", s1.eq(&s2));



//     // // let user = select_all();
//     // let user = select_user();
//     // // match로 에러를 처리
//     // // match user {
//     // //     Ok(user) => println!("User: {}", user),
//     // //     Err(MyError::NotFoundError) => println!("User not found"),
//     // //     Err(MyError::PermissionDeniedError) => println!("Permission denied"),
//     // // }
//     // let user = match user {
//     //     Ok(user) => user,
//     //     Err(error) => {
//     //         panic!("Error: {:?}", error);  // 에러를 출력하고 프로그램을 종료
//     //     }
//     // };
// }





// fn main() {
//     // let f = File::open("hello.txt");
//     // let f = match f {
//     //     Ok(file) => file,  // 성공할 경우 파일 핸들을 담고 있는 OK 인스턴스 
//     //     // 실패할 경우 더 많은 에러 정보를 가지고 있는 Err 인스턴스
//     //     Err(error) => panic!("Problem opening the file: {:?}", error), // 패닉을 호출하여 프로그램 종료
//     // };

//     // // 실패 이유에 따라 다른 행동을 취하는 방법
//     // // File::open이 반환하는 값의 타입은 io::Error인데 표준라이브러리에서 제공하는 구조체 
//     // let f = File::open("hello.txt");
//     // let f = match f {
//     //     Ok(file) => file,
//     //     // Error 구조체는 kind 메소드 제공 이를 호출하여 ErrorKind 값을 얻을 수 있음
//     //     // 사용하고자 하는 variant는ㄴ ErrorKind::NotFound이다. 
//     //     // if error.kind() == ErrorKind::NotFound는 매치 가드(match guard) 라고 부른다. 
//     //     // 줄기의 코드가 실행되기 위해서는 이 조건문(if error.kind() == ErrorKind::NotFound )이 참이어야 한다.
//     //     // 패턴에는 ref가 필요하며, error가 가드 조건문으로 소유권 이동이 되지 않고 참조만 된다. 
//     //     // &대신 ref이 사용된다
//     //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
//     //         // File::create 또한 실패할 수 있기 때문에, 안쪽에 match 구문을 바깥쪽과 마찬가지로 추가할 필요가 있습니다.
//     //         match File::create("hello.txt") {
//     //             Ok(fc) => fc,
//     //             Err(e) => panic!("Problem creating the file: {:?}", e),
//     //         }
//     //     },
//     //     Err(error) => panic!("Problem opening the file: {:?}", error),
//     // };

//     // unwrap 
//     // Result 값이 Ok variant라면, unwrap은 Ok 내의 값을 반환할 것입니다. 
//     // 만일 Result가 Err variant라면, unwrap은 우리를 위해 panic! 매크로를 호출할 것입니다.
//     // let f = File::open("hello.txt").unwrap();

//     // expect 
//     // expect는 unwrap과 비슷하지만, panic! 매크로가 호출될 때 전달되는 에러 메시지를 지정할 수 있습니다.
//     // let f = File::open("hello.txt").expect("Failed to open hello.txt");


// }

