use std::fmt::{Debug, Display};

trait Prints: Debug {
  fn debug_print(&self) {
    println!("i am {:?}", self);
  }
}

impl<T: Debug> Prints for T {}

#[derive(Debug)]
struct Person;

#[derive(Debug)]
struct Building;

pub fn print_blanket_trait() {
  let my_person = Person;
  let my_building = Building;
  let my_string = "hehe";

  my_person.debug_print();
  my_building.debug_print();
  my_string.debug_print();
}

/*
blanket trait 왜 쓰는고?
이미 정의된 타입에 메서드를 추가할 수 있기 때문
 */