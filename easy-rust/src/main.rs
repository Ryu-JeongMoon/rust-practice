mod rust_025;
mod rust_026;
mod rust_027;
mod rust_028;

fn main() {
  println!("===================================");
  // rust_022::print_copy_and_clone();
  // rust_023::print_uninitialized_variables();
  // rust_023::print_loop();
  // rust_024::print_collections();
  // rust_025::print_slices();
  // rust_026::print_vec();
  // rust_027::print_from_into();
  rust_028::print_tuples();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */