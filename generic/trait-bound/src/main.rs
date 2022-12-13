extern crate traitbound;

// 함수에 generic을 사용하기 위해서는 Trait을 구현한 타입을 사용해야 한다.
pub fn notify<T: traitbound::Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

struct NewsArticle {
    headline: String,
    location: String,
}

// trait 구현 
impl traitbound::Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{},{}", self.headline, self.location)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
    };
    // generic 함수 호출
    notify(article);
}

