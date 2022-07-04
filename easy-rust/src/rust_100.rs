use std::borrow::Borrow;
use std::thread::available_parallelism;

fn print_destructuring() {
  let (mut x, mut y) = (9, 10);
  println!("x = {x}, y = {y}");
}

fn print_available_parallelism() {
  let a = available_parallelism().unwrap();
  println!("we have {a} cores");
}

fn print_number_to_char() {
  let my_char = char::try_from(333u32);
  println!("{my_char:?}");
}

pub fn print_rust_1_59() {
  // print_destructuring();
  // print_available_parallelism();
  print_number_to_char();
}

