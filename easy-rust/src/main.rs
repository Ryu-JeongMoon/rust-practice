extern crate core;

mod rust_053;

fn main() {
  println!("===================================");
  rust_053::print_hashmap();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */