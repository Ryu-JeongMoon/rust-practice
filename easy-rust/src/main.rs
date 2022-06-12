extern crate core;

mod rust_069;
mod rust_070;
mod rust_071;
mod rust_072;

fn main() {
  println!("===================================");
  // rust_069::print_iterator();
  // rust_070::print_iterators_assert_eq();
  // rust_071::print_impl_iterator();
  rust_072::print_closure();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */