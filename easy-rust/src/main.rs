extern crate core;

mod rust_065;
mod rust_066;
mod rust_067;
mod rust_068;

fn main() {
  println!("===================================");
  // rust_065::print_blanket_trait();
  // rust_066::print_blanket_trait();
  // rust_067::print_as_ref();
  rust_068::print_method_chaining();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */