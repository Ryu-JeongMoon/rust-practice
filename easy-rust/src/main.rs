extern crate core;

mod rust_033;
mod rust_034;
mod rust_035;
mod rust_036;

fn main() {
  println!("===================================");
  // rust_033::print_size_of_struct();
  // rust_034::print_enums();
  // rust_035::print_enums();
  rust_036::print_stars();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */