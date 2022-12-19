#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

// 해시맵을 생성하고 사용하는 방법을 보여준다. 
fn create_hash_map() {
    // 해시맵 생성
    let mut scores = HashMap::new();

    // 해시맵에 값 추가
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // 해시맵에서 값 가져오기
    let score = scores.get(&team_name);
    println!("score: {:?}", score);
}

// String 타입을 정의하고 그것을 해쉬맵에 추가하면 
// 소유권이 해쉬맵에 이동된다
fn use_hashmap_borrow(){ 

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    
    let mut map = HashMap::new();
    // String과 같이 소유된 값들에 대해서는, 값들이 이동되어 해쉬맵이 그 값들에 대한 소유자가 될 것이다
    map.insert(field_name, field_value);
    // field_name과 field_value은 이 지점부터 유효하지 않습니다.
    // 이들을 이용하는 시도를 해보고 어떤 컴파일러 에러가 나오는지 보세요!
}


// 해시맵에서 값을 가져오는 방법을 보여준다.
fn use_get_method() {

    let mut scores = HashMap::new();

    // 해시맵에 값 추가
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    // 값 가져오기 
    let score = scores.get(&team_name);
    // Some(10)가 반환된다. 
    println!("score: {:?}", score);
}


// for 루프를 이용하여 해시맵에서 값을 가져오는 방법을 보여준다.
fn use_get_with_for() {

    let mut scores = HashMap::new();

    // 해시맵에 값 추가
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 벡터에서 했던 방법과 유사한 식으로 for 루프를 이용하여 해쉬맵에서도 각각의 키/값 쌍에 
    // 대한 반복작업을 할 수 있다.
    for (key, value) in &scores {
        println!("key-value={}: {}", key, value);
    }
}

// 해시맵에 값이 이미 존재하는 경우, 기존의 값이 덮어쓰여진다.
fn overwite_value() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    // 기존의 값이 덮어쓰여진다.
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}

// 키가 없는 경우에만 값을 추가하고 싶을 때는 or_insert 메소드를 사용한다.
fn insert_without_key() {
    let mut scores = HashMap::new();

    // 해당되는 키가 없는 경우에만 값을 추가한다.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    // 결과를 확인해 보면 추가되지 않는다. 
    scores.entry(String::from("Blue")).or_insert(100);
    println!("{:?}", scores);
}



// 값 변경하기 
fn change_value() {
    let mut map = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 2);
    map.insert("orange", 1);
    
    // unwrap()을 사용하여 값을 가져온다.
    let mut orange_count = map.get_mut("orange").unwrap();
    // 포인터를 사용하여 여참조하여 값을 변경한다.
    *orange_count += 1;
    println!("orange_count count: {:?}", orange_count);
}


fn create_hash_map_from_array() {
    let solar_distance = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);


    println!("{:?}", solar_distance);
}


fn main() {
    // create_hash_map();
    // use_hashmap_borrow();
    // use_get_method();
    // use_get_with_for();
    // overwite_value();
    // insert_without_key();
    // change_value();
    create_hash_map_from_array();
}
