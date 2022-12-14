
// 시그니처 내의 모든 참조자들이 동일한 라이프타임 'a를 가지고 있어야 함을 특정한 longest 함수 정의
// 어떤 라이프타임 'a에 대하여, 이 함수는 두 개의 파라미터를 갖는다
// 두 개 모두 라이프타임 'a 만큼 살아 있는 스트링 슬라이스 
// 적어도 라이프타임 'a 만큼 살아 있는 스트링 슬라이스를 반환
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
      x
  } else {
      y
  }
}

fn main() {
  let string1 = String::from("long string is long");
  let result;
  {
      let string2 = String::from("xyz");
      // 서로 다른 구체적인 라이프타임을 가진 참조자들을 넘겨도 longest 함수는 컴파일 됨
      result = longest(string1.as_str(), string2.as_str());
      // string2는 여기까지 유효 
  }
  // result는 유효하지만 string2가 유효하지 않음, 
  // 이런 에러가 남
  //    result = longest(string1.as_str(), string2.as_str());
  //|                                          ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
  println!("The longest string is {}", result); 
  // string1은 여기까지 유효 
}
