extern crate core;

mod rust_077;
mod rust_078;
mod rust_079;
mod rust_080;

fn main() {
  println!("===================================");
  // rust_077::print_filter();
  // rust_078::print_filter_map();
  // rust_079::print_map();
  rust_080::print_option();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */