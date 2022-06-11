use std::fmt::Debug;

trait Prints: Debug {
  // &self 를 넘겨야 var.debug_print() 형태로 쓸 수 있다
  // 넘기지 않으면 Prints.debug_print() 형태로 써야함 -> static method ?!
  fn debug_print(&self) {
    println!("i am {:?}", self);
  }
}

// impl<T> trait-name for type-name, type-name means T in generic type
impl<T: Debug> Prints for T {}

#[derive(Debug)]
struct Panda;

pub fn print_blanket_trait() {
  let my_panda = Panda;
  my_panda.debug_print();
}

