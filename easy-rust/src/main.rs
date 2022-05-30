extern crate core;

mod rust_045;
mod rust_046;
mod rust_047;
mod rust_048;

fn main() {
  println!("===================================");
  // rust_045::print_more_generics();
  // rust_046::print_option_and_result();
  // rust_047::print_unwrap_option();
  rust_048::print_result();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */