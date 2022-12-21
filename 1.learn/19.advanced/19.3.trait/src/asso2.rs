
struct Container(i32, i32);

trait Contains {
    type A; 
    type B; 
    fn first(&self) -> Self::A;
    fn last(&self) -> Self::B;
}

impl Contains for Container {
    type A = i32;
    type B = i32;
    fn first(&self) -> Self::A { self.0 }
    fn last(&self) -> Self::B { self.1 }
}

fn main() {
    let x = Container(1, 2);
    println!("{}", x.first());
    println!("{}", x.last());
}