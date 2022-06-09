use std::fmt;
use std::fmt::{Formatter, write};

#[derive(Debug)]
struct Dog {
  name: String,
  age: u8,
}

// @ToString 같은 것이로고만
impl fmt::Display for Dog {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let name = &self.name;
    let age = self.age;
    write!(f, "My Dog's name is {name}, and he is {age} years old.")
  }
}

pub fn print_trait() {
  let my_dog = Dog {
    name: "뽀실".to_string(),
    age: 4,
  };

  println!("My dog is {my_dog:?}");
  println!("{my_dog}");
}