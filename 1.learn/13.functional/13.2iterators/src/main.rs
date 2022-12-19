fn iter_array() {
    let numbers = [2, 1, 17, 99, 34, 56];

    // iterator
    let numbers_iterator = numbers.iter();

    for number in numbers_iterator {
        println!("{}", number);
    }
}


fn iter_next() {
    let colors = vec!["Red", "Yellow", "Green"];
    
    // iterator
    let mut colors_iterator = colors.iter();
    println!("colors iterator = {:?}", colors_iterator);
    
    // fetch values from iterator one by one using next() method
    println!("{:?}", colors_iterator.next());
    println!("{:?}", colors_iterator.next());
    println!("{:?}", colors_iterator.next());
    println!("{:?}", colors_iterator.next());
}



fn main() {
    // iter_array();
    iter_next();
}
