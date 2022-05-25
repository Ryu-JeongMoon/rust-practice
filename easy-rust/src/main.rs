mod rust_029;
mod rust_030;
mod rust_031;
mod rust_032;

fn main() {
  println!("===================================");
  // rust_029::print_control_flow();
  // rust_030::print_match_statements();
  // rust_031::print_complicated_match_statements();
  rust_032::print_struct();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */