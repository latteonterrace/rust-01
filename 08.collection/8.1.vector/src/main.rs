#![allow(dead_code)]
#![allow(unused_variables)]

// 요소 가져오기 방법
fn use_vec1() {
    let v = vec![1, 2, 3, 4, 5];
    // 참조자를 사용하여 벡터의 첫 번째 요소를 가져온다.
    let first = &v[0];
    println!("The first element is: {}", first);
    let second = v.get(1);
    // Option을 반환
    println!("The second element is: {:?}", second); // The second element is: Some(2)
    println!("The second element is: {}", second.unwrap()); // The second element is: 2
}

// 가변일 경우 추가 가능
fn use_vec2() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6); // 요소 추가
    v.push(7); // 요소 추가
    println!("The vector is: {:?}", v); // The vector is: [1, 2, 3, 4, 5, 6, 7]
}

// 잘못된 참조
// 프로그램에 유효한 참조가 있는 경우 차용 검사기는 소유권 및 차용 규칙(4장에서 다룸)을 시행하여 이 참조 및 벡터 내용에 대한 다른 참조가 유효한지 확인합니다.
// 동일한 범위에서 가변 및 불변 참조를 가질 수 없다는 규칙을 상기하십시오
// 그러나 오류가 발생하지 않는다. 왜지?
fn use_vec3() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6); // 요소 추가
}

// vector 값 반복
fn use_vec4() {
    // for벡터의 각 요소에 대한 불변 참조를 얻고 이를 출력하는 방법을 보여줍니다
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}

// 모든 요소를 ​​변경하기 위해 가변 벡터의 각 요소에 대한 가변 참조를 반복할 수도 있습니다. for목록 8-9 의 루프는 50각 요소에 추가됩니다.
fn use_vec5() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}


// if let 사용
fn use_vec6() {
    
    let v = vec![1, 2, 3, 4, 5];
    // get(index)를 사용하여 벡터의 요소를 가져온다.
    let third: Option<&i32> = v.get(2);
    if let None = third {
        println!("Got nothing");
    } else {
        println!("Got something {}", third.unwrap());
    }
}


// 불변 vector를 인자로 받을 때 시그니처 
fn disp_vec(v: &Vec<i32>) {
    for i in v {
        println!("{}", i);
    }
}

//  불변 vector를 인자로 전달할
fn disp_vec_main() {
    let v = vec![100, 32, 57];
    disp_vec(&v);
}

// 가변 벡터 인자
fn mod_vec(v: &mut Vec<i32>) {
    for i in v {
        *i += 50;
    }
}

// 가변 벡터 인자
fn mod_vec2(v: &mut [i32]) {
    for i in v {
        *i += 100;
    }
}

// 가변 벡터 인자 함수 사용 메인 함수 
fn mod_vec_main() {
    let mut v = vec![100, 32, 57];
    mod_vec2(&mut v);
    disp_vec(&v);
}


fn main() {
    // use_vec1();
    // use_vec2();
    // use_vec3();
    // use_vec4();
    // use_vec5();
    // use_vec6();
    // disp_vec_main();
    mod_vec_main();

}
