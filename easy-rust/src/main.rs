mod rust_001;
mod rust_002;
mod rust_003;
mod rust_004;

fn main() {
  rust_001::print_hello_world();
  rust_002::comments();
  rust_003::print_integer_types();
  rust_004::print_chars();
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */