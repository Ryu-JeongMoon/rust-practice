extern crate core;

mod rust_093;
mod rust_094;
mod rust_095;
mod rust_096;

fn main() {
  println!("===================================");
  // rust_093::print_mutability_and_cells();
  // rust_094::print_refcell();
  // rust_095::print_refcell();
  rust_096::print_interior_mutability();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */