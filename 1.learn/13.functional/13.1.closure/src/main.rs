fn run_closure() {
    // 클로저 정의
    // closure that prints a text
    let print_text = || println!("Hello, World!");
    print_text(); // closure 호출
}

fn run_closure2() {
    // 클로저 정의
    // define closure to add 1 to an integer
    let add_one = |x: i32| x + 1;
    // call the closure with value 2
    let added = add_one(2);
    println!("Added value is {}", added);
}

fn run_multi_line_closure() {
    // define a multi-line closure
    let squared_sum = |x: i32, y: i32| {
        // find the sum of two parameters
        let mut sum: i32 = x + y;

        // find the squared value of the sum
        let mut result: i32 = sum * sum;

        return result;
    };

    // call the closure
    let result = squared_sum(5, 3);

    println!("Result = {}", result);
}

fn run_capture_env() {
    let num = 100;

    // A closure that captures the num variable
    let print_num = || println!("Number = {}", num);

    print_num();
}

fn run_env_not_modified() {
    let word = String::from("Hello");

    // immutable closure
    let print_str = || {
        // 내부에서 수정되지 않는다.
        println!("word = {}", word);
    };

    // immutable borrow is possible outside the closure
    println!("length of word = {}", word.len());

    print_str();

}


fn run_env_modified() {
    let mut word = String::from("Hello");
    
    // mutable closure
    let mut print_str = || {
        // value of word is changed here
        // 여기서 수정된다.
        word.push_str(" World!");
        println!("word = {}", word);
    };
     
     // cannot immutable borrow because the variable is borrowed as mutable inside the closure
     // println!("length of word = {}", word.len());
    
    print_str();

    // can immutable borrow because the closure has been already used
    println!("length of word = {}", word.len());
}


fn run_env_moved() {
    let word = String::from("Hello");

    // immutable closure
    let print_str = || {
        // word variable is moved to a new variable
        // 여기서 변수가 이동된다.
        let new_word = word;
        println!("word = {}", new_word);
    };

    print_str();

    // cannot immutable borrow because word variable has moved inside closure
    // println!("length of word = {}", word.len());
}

fn main() {
    run_closure();
    run_closure2();
    run_multi_line_closure();
    run_capture_env();
} //;
