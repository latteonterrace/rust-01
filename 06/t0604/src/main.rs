
fn select_one(val: i32) -> Option<i32> {
    // 조회를 했다고 가정하고
    if val == 0 {
        // 조회한 데이터가 없다면
        None // None 을 반환
    } else {
        Some(val) // 조회한 데이터를 반환
    }
}

fn main() {
    // match를 이용한 옵션 값 비교
    let x = Some(10); // 옵션으로 만들어야 함
    match x {
        Some(10) => println!("Got 10"),
        None => println!("Got nothing"),
        // 위에 해당 사항이 없을 때
        _ => println!("Got something else"),
    }

    // match 바인딩 사용 
    let y = select_one(10);
    match y {
        Some(0) => println!("Got 0"),
        // 옵션값이 i에 바인딩
        Some(i) => println!("Got some other number: {}", i),
        None => println!("Got nothing"),
    }
    
    // 변수에 값 설정 
    let k = select_one(10);
    let mut l = 0;
    match y {
        Some(0) => l = 0,
        Some(i) => l = i,
        None => l = 0,
    }
    println!("l = {}", l);


    // if 사용 
    let comp = Some(10);  // 옵션으로 만들어야 함
    if let Some(10) = comp {  // 옵션끼리 비교 
        println!("Got 10");
    } else {
        println!("Got something else");
    }

  
    // None 체크 (null)
    let x = select_one(1);
    if let None = x {
        println!("Got nothing");
    } else {
        println!("Got something");
        println!("x = {}", x.unwrap());
    }

    // match 의 Some(i)와 같이 바인딩 사용 
    if let Some(val) = x {
        println!("Got something");
        println!("Some(val) = {}", val);
    } else {
        println!("Got nothing");
    }

    // unwrapping 
    // match 사용 
    let x = Some(10);
    let y:u32 = match x {
        Some(10) => 10,
        None => 0,
        _ => 0,
    };
    println!("y = {}", y);
    // 또는 
    let x = Some(10);
    let y:u32 = match x {
        Some(i) => i,
        None => 0,
    };
    println!("y = {}", y);
    

    // unwrap 사용
    let x = Some(10);
    let y:u32 = x.unwrap();
    println!("unwrap  = {}", y);

    // unwrapor() 사용 
    let x = Some(10);
    let y:u32 = x.unwrap_or(0);
    println!("unwrapor  = {}", y);


    // unwrapor_else() 사용
    let x = Some(10);
    let y:u32 = x.unwrap_or_else(|| 0);
    println!("unwrapor_else  = {}", y);

    let x = Some(10);
    let y:u32 = x.unwrap_or_else(|| {
        println!("x is None");
        0
    });
    println!("unwrapor_else  = {}", y);


    // expect() 사용
    let x = Some(10);
    // let x = None;
    // 이 경우 y가 None일 때 패닉이 발생하고 지정한 메시지가 출력된다.
    let y:u32 = x.expect("x is None");
    println!("expect  = {}", y);
  
}




