extern crate core;

mod rust_039;
mod rust_040;
mod rust_041;
mod rust_042;
mod rust_043;
mod rust_044;

fn main() {
  println!("===================================");
  // rust_040::print_more_impl();
  // rust_041::print_enum_impl_blocks();
  // rust_042::print_destructuring();
  // rust_043::print_dot_operator();
  rust_044::print_generics();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */