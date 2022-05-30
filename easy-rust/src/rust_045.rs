use std::fmt::Display;

fn compare_and_print<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
  println!("{}! Is {} greater than {}? => {}", statement, num_1, num_2, num_1 > num_2);
}

// where -> extends 하는 것들이 많아지면 따로 빼내어 가독성 향상
fn compare_and_print2<T, U>(statement: T, num_1: U, num_2: U)
  where T: Display, U: Display + PartialOrd {
  println!("{}! Is {} smaller than {}? => {}", statement, num_1, num_2, num_1 < num_2);
}

pub fn print_more_generics() {
  compare_and_print("Listen up", 535, 5959);
  compare_and_print2("Listen man", 535, 5959);
}