mod rust_001;
mod rust_002;
mod rust_003;
mod rust_004;
mod rust_005;
mod rust_006;
mod rust_007;
mod rust_008;
mod rust_009;
mod rust_010;
mod rust_011;
mod rust_012;
mod rust_013;
mod rust_014;
mod rust_015;
mod rust_016;
mod rust_017;
mod rust_018;
mod rust_019;
mod rust_020;

fn main() {
  println!("===================================");
  // rust_001::print_hello_world();
  // rust_002::comments();
  // rust_003::print_integer_types();
  // rust_004::print_chars();
  // rust_005::print_length_and_chars_count();
  // rust_006::print_type_inference();
  // rust_007::print_println();
  // rust_008::print_println2();
  // rust_009::print_tuple();
  // rust_010::print_number();
  // rust_011::print_shadowing_in_scopes();
  // rust_012::print_reference();
  // rust_013::print_with_tab();
  // rust_014::print_strings();
  // rust_015::print_string_type();
  // rust_016::print_const();
  // rust_017::print_ownership();
  // rust_018::print_mut_reference();
  // rust_019::print_unique_reference();
  rust_020::print_references_in_functions();
  println!("===================================");
}

/*
외부 함수 땡겨올 때 pub 설정되어 있어야하고 mod + 파일명
사용할 때는 파일명::함수명();
 */