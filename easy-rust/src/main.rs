extern crate core;

mod rust_057;
mod rust_058;
mod rust_059;
mod rust_060;

fn main() {
  println!("===================================");
  // rust_057::print_question_mark_operator();
  // rust_058::print_formatting_str();
  // rust_059::print_trait();
  rust_060::print_trait();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */