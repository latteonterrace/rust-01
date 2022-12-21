fn arr1() {
    // i32 타입의 요소 3개를 갖는 배열을 선언한다.
    // [0; 3]은 0으로 초기화된 3개의 요소를 갖는 배열을 의미한다.
    let mut array: [i32; 3] = [0; 3];
    println!("{:?}", array);

    array[1] = 1; // 두 번째 요소에 1을 할당한다.
    array[2] = 2; // 세 번째 요소에 2를 할당한다.

    assert_eq!([1, 2], &array[1..]); // array[1..]은 array[1]부터 끝까지의 슬라이스를 의미한다.

    // This loop prints: 0 1 2
    for x in array {
        print!("{x} ");
    }
}

fn arr2() {
    let mut array: [i32; 3] = [0; 3];
    // &array는 array의 레퍼런스를 의미한다.
    for x in &array {
        print!("{x} ");
    }
}

fn arr3() {
    let bytes: [u8; 3] = [1, 0, 2];
    assert_eq!(1, u16::from_le_bytes(<[u8; 2]>::try_from(&bytes[0..2]).unwrap()) );
    assert_eq!(512, u16::from_le_bytes(bytes[1..3].try_into().unwrap()));
}

fn main() {
    // arr1();
    // arr2();
    arr3();
}
