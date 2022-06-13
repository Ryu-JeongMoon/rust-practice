extern crate core;

mod rust_073;
mod rust_074;
mod rust_075;
mod rust_076;

fn main() {
  println!("===================================");
  // rust_073::print_zero_cost_abstraction();
  // rust_074::print_map_and_foreach();
  // rust_075::print_zip();
  rust_076::print_char_method();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */