fn give_age() -> u32 {
  ((1 << 31) - 1) & 1
}

pub fn print_println() {
  println!("age => {}", give_age());
}