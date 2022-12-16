// 표준 라이브러리 use
use std::env;
use std::fs::File;
use std::io::prelude::*;



// fn main() {
//     // 인자들을 변수에 할당 
//     let args: Vec<String> = env::args().collect();
//     // 인자들을 모두 출력, 우리는 디버그 형식자인 :?으로 벡터를 출력한다. 
//     println!("{:?}", args);
//     // 다음을 실행한다. 
//     //  cargo run searchString  example.txt 
//     // args에 할당된 값들은 다음과 같이 보인다. 
//     // ["target\\debug\\greprs.exe", "searchString", "example.txt"]
//     // 변수에 대입 
//     // let query = &args[1];
//     // let filename = &args[2];
    
//     //let config = parse_config(&args);
//     let config = Config::new(&args).unwrap_or_else(|err| {
//         println!("Problem parsing arguments: {}", err);
//         std::process::exit(1);
//     });


//     // 변수 출력 
//     println!("Searching for {}", query);
//     println!("In file {}", filename);


//     let mut f = File::open(filename).expect("file not found");
//     let mut contents = String::new();
//     f.read_to_string(&mut contents).expect("Something went wrong reading the file");
//     println!("With text:\n{}", contents);
// }


// struct Config {
//     query: String,
//     filename: String,
// }

// impl Config {
//     pub fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Ok(Config { query, filename })
//     }
// }


// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Config { query, filename }
// }


