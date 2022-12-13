extern crate traituse;

use traituse::*; 

fn main() {
  let dog = traituse::Dog {
    Pomeranian: String::from("Pomeranian"),
    Poodle: String::from("Poodle"),
    Dashund: String::from("Dashund"),
  };
  dog.common_bark();
  dog.custom_bark();
}