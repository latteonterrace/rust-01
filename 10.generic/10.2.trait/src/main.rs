#![allow(unused)]

// Trait 정의 
pub trait Barkable {
    fn bark(&self, str: String) -> String;
}

// Dog 구조체 
struct Dog { 
    name: String,
}

// impl 뒤에 구현하려고 하는 트레잇 이름을 적고, for 뒤에 구현하려는 타입을 적음
impl Barkable for Dog {
    // 메소드 구현 
    fn bark(&self, conts: String) -> String {
        format!("{} is barking", conts)
    }
}

// Dog 인스턴스에서 해당 메소드들을 호출할 수 있다. 
fn use_barkable() {
    let conts = "명멍".to_string();
    let dog = Dog { name: "Pomeranian".to_string() };
    println!("{}", dog.bark(conts));
}


// 함수에서 generic type을 사용하는 경우
fn animal_bark<T: Barkable>(anmial: T, conts: String) -> String {
    anmial.bark(conts)
}

// animal_bark 함수 호출 
fn use_animal_bark() {
    let dog = Dog { name: "Pomeranian".to_string() };
    let result = animal_bark(dog, "명멍".to_string());
    println!("{}", result);
}


// 여러 개의 트레잇 바운드 

// Trait 정의 
pub trait Feedable {
    fn feed(&self, str: String);
}


// impl 뒤에 구현하려고 하는 트레잇 이름을 적고, for 뒤에 구현하려는 타입을 적음
impl Feedable for Dog {
    // 메소드 구현 
    fn feed(&self, food: String) {
        println!("the dog had {}", food);
    }
}

// 두 개의 트레잇을 구현한 타입을 genecir type으로 받는다. 
fn animal_bark_and_feed<T: Barkable + Feedable>(animal: T, conts: String, food: String) -> String {
    animal.feed(food);
    animal.bark(conts)
}


// 두 개의 트레잇을 구현한 타입을 genecir type으로 받는다. 
fn animal_bark_and_feed_where<T>(animal: T, conts: String, food: String) -> String
    where T: Barkable + Feedable {
    animal.feed(food);
    animal.bark(conts)
}

// 두 개의 트렛이를 구현하는 함수 호출
fn use_animal_bark_and_feed() {
    let dog = Dog { name: "Pomeranian".to_string() };
    let result = animal_bark_and_feed(dog, "명멍".to_string(), "밥".to_string());
    println!("{}", result);
}


// 두 개의 트렛이를 구현하는 함수 호출
fn use_animal_bark_and_feed_where() {
    let dog = Dog { name: "Pomeranian".to_string() };
    let result = animal_bark_and_feed_where(dog, "명멍".to_string(), "밥".to_string());
    println!("{}", result);
}

fn main() {
    //use_barkable();
    // use_animal_bark();
    // use_animal_bark_and_feed();
    use_animal_bark_and_feed_where();
    
}

