extern crate core;

mod rust_097;
mod rust_098;
mod rust_099;
mod rust_100;

fn main() {
  println!("===================================");
  // rust_097::print_reference_counting();
  // rust_098::print_rc();
  // rust_099::print_rc_and_refcell();
  rust_100::print_rust_1_59();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */