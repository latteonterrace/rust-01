
// String Type을 사용하는 Result Type
fn return_result() -> Result<String, String> {
    //Ok(String::from("Ok!"))
    Err(String::from("Error!"))
}


// Result를 사용한 에러 처리 
fn use_return_result() {
    let r = return_result();
    match r {
        Ok(s) => println!("Ok: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}

// int 타입을 사용하는 Result Type
fn return_result_using_i32() -> Result<i32, String> {
    Ok(100)
    // Err(String::from("Error!"))
}

// Result를 사용한 에러 처리
fn use_return_result_using_i32() {
    let r = return_result_using_i32();
    match r {
        Ok(i) => println!("Ok: {}", i),
        Err(e) => println!("Error: {}", e),
    }
}


// 에러 코드를 enum으로 정의 
#[derive(Debug)]
enum MyError {
    NotFoundError,
    PermissionDeniedError,
    UpdateError, 
}

// Result<String, MyError>는 String을 반환하거나 MyError를 반환합니다.
fn select_user() -> Result<String, MyError> {
    Err(MyError::NotFoundError)
    // Ok(String::from("Tom"))
}

fn use_select_user() {
    let user = select_user();
    match user {
        Ok(user) => println!("user: {}", user),
        Err(e) => println!("Error: {:?}", e),
    }
}


// 서로 다른 에러에 대해 매칭하기 
fn use_select_user2(){
    let user = select_user();
    match user {
        Ok(user) => println!("user: {}", user),
        Err(MyError::NotFoundError) => println!("Not Found Error"),
        Err(MyError::PermissionDeniedError) => println!("Permission Denied Error"),
        _ => println!("Other Error"),
    }
}


// 결과값이 OK인지 확인하기
fn use_is_ok(){ 
    let user = select_user();
    if user.is_ok() {
        println!("Ok");
    }else {
        println!("Error");
    }

}


// 결과가 에러인지 확인하기 
fn use_is_err() {
    let user = select_user();
    if user.is_err() {
        println!("Error");
    }else {
        println!("Ok");
    }
}

// 만일 Result 값이 Ok variant라면, unwrap은 Ok 내의 값을 반환할 것이다.
// 만일 Result가 Err variant라면, unwrap은 우리를 위해 panic! 매크로를 호출할 것이다.
fn use_unwrap() {
    let user = select_user().unwrap();
}


// 만일 Result 값이 Ok variant라면, unwrap은 Ok 내의 값을 반환할 것이다.
// 만일 Result가 Err variant라면, unwrap은 우리를 위해 panic! 매크로를 호출할 것이다.

fn use_expect() {
    // 에러 메시지를 선택할 수 있다. 
    let user = select_user().expect("Error!!!!!!! Error");
}




fn main() {
    // use_return_result();
    // use_return_result_using_i32();
    // use_select_user();
    // use_Select_user2();
    // use_is_ok();
    // use_is_err();
    // use_unwrap();
    use_expect();
}
