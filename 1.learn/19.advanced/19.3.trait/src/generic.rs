// // 튜플 구조체를 하나 만들자 
struct Container(i32, i32);

// // 제네릭 유형 A 및 B의 사용을 허용
trait Contains<A,B> {
    fn contains(&self, _: &A, _: &B) -> bool; // Explicitly requires `A` and `B`.
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    // True if the numbers stored are equal.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // Grab the first number.
    fn first(&self) -> i32 { self.0 }

    // Grab the last number.
    fn last(&self) -> i32 { self.1 }
}

// Contains가 제네릭이기 때문에 fn difference()에 대한 모든 제네릭 유형을 명시적으로 지정해야 합니다
// 실제로 우리는 A와 B가 입력 C에 의해 결정된다는 것을 표현하는 방법을 원합니다
// 다음 섹션에서 볼 수 있듯이 연관 유형은 정확히 해당 기능을 제공합니다
fn difference<A, B, C>(container: &C) -> i32 where C: Contains<A, B> {
    container.last() - container.first()
}



fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}