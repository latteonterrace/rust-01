
// First, Last라는 연관타입을 가진 트레잇
trait GetItems {
  type First; 
  type Last;
  fn get_first(&self) -> &Self::First;
  fn get_last(&self) -> &Self::Last;
}

// 트레잇 구현. ( T, U) 형의 튜플, 즉 두 개의 값을 갖는 튜플에 대해 트레잇을 구현 
impl<T, U> GetItems for (T, U) {
  type First = T;
  type Last = U;
  fn get_first(&self) -> &Self::First { &self.0 }
  fn get_last(&self) -> &Self::Last { &self.1 }
}


fn main(){ 
  // 이렇게만 선언해도 
  let x = (1, 2);
  // trait의 메소드를 사용할 수 있다. 
  println!("{}", x.get_first());
  println!("{}", x.get_last());
  // let y = (1, 2, 3);
  // println!("{}", y.get_first());
  // println!("{}", y.get_last());
}


