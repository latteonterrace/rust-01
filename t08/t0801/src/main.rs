fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: Option<&i32> = v.get(2);
    if let None = third {
        println!("Got nothing");
    } else {
        println!("Got something {}", third).unwrap();
    }

    // 
}
