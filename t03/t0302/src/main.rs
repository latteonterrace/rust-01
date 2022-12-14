fn main() {
    // 기본타입과 사칙연산
    let sum = 1 + 2;
    println!("{}", sum);

    let difference = 10.1 - 0.2;
    println!("{}", difference);

    let multiplication = 10 * 20;
    println!("{}", multiplication);

    let division = 50.5 / 30.5;
    println!("{}", division);

    let remainder = 53 % 5;
    println!("{}", remainder);

    // boolean 타입
    let t = true;
    let f: bool = false; // with explicit type annotation

    // 문자 타입
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // 튜플 타입
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // 개별 값을 밖으로 빼내오기
    println!("The value of y is: {}", y);

    // 순서대로 0,1,2의 인덱스
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // 배열
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
