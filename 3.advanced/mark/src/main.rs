use comrak::{markdown_to_html, ComrakOptions};

fn assert_markdown() {
    assert_eq!(markdown_to_html("Hello, **世界**!", &ComrakOptions::default()),
    "<p>Hello, <strong>世界</strong>!</p>\n");

    let res = markdown_to_html("Hello, **世界**!", &ComrakOptions::default()); 
    println!("{}", res);
}


fn main() {
    assert_markdown();
}