extern crate core;

mod rust_049;
mod rust_050;
mod rust_051;
mod rust_052;

fn main() {
  println!("===================================");
  // rust_049::print_result();
  // rust_050::print_if_let_and_while_let();
  // rust_051::print_map();
  rust_052::print_hash_map();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */