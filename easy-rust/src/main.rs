extern crate core;

mod rust_061;
mod rust_062;
mod rust_063;
mod rust_064;

fn main() {
  println!("===================================");
  // rust_061::print_trait();
  // rust_062::print_another_trait();
  // rust_063::print_trait_as_bounds();
  rust_064::print_implementing_from();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */