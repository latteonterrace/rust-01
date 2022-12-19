
fn ref_func(str: &str) -> &str {  // &str 타입의 참조를 받음 
    // 빌림 참조 반환 ?  
    str   
}

// fn use_ref_func(str: &str) {
// }

fn main() {
    // String type 
    let str = String::from("Hello");
    // 참조를 넘김 
    let return_value = ref_func(&str);
    println!("{}", return_value);
    println!("{}", str);
}
