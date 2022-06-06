extern crate core;

mod rust_053;
mod rust_054;
mod rust_055;
mod rust_056;

fn main() {
  println!("===================================");
  // rust_053::print_hashmap();
  // rust_054::print_collections();
  // rust_055::print_binary_heap();
  rust_056::print_vec_deque();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */