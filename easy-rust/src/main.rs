extern crate core;

mod rust_037;
mod rust_038;
mod rust_039;
mod rust_040;

fn main() {
  println!("===================================");
  // rust_037::print_not_rust_style_loop();
  // rust_038::print_rust_style_loop();
  // rust_039::print_impl();
  rust_040::print_more_impl();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */