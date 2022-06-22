extern crate core;

mod rust_085;
mod rust_086;
mod rust_087;
mod rust_088;

fn main() {
  println!("===================================");
  // rust_085::print_folds();
  // rust_086::print_chunks_windows();
  // rust_087::print_match_indices_peekable();
  rust_088::print_debug_macro();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */