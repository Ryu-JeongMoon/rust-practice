extern crate core;

mod rust_089;
mod rust_090;
mod rust_091;
mod rust_092;

fn main() {
  println!("===================================");
  // rust_089::print_inspect();
  // rust_090::print_lifetime();
  // rust_091::print_static_lifetime();
  rust_092::print_dbg();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */