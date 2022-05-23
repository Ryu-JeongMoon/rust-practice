mod rust_022;
mod rust_023;
mod rust_024;

fn main() {
  println!("===================================");
  // rust_022::print_copy_and_clone();
  // rust_023::print_uninitialized_variables();
  // rust_023::print_loop();
  rust_024::print_collections();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */