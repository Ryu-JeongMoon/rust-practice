extern crate core;

mod rust_081;
mod rust_082;
mod rust_083;
mod rust_084;

fn main() {
  println!("===================================");
  // rust_081::print_any_all();
  // rust_082::print_reverse_vec();
  // rust_083::print_find_position_cycle();
  rust_084::print_skip_fold();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */