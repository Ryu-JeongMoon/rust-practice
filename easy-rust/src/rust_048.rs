fn check_even_and_odd(input: i32) -> Result<(), ()> {
  match input % 2 {
    0 => Ok(()),
    _ => Err(())
  }
}

pub fn print_result() {
  let result = check_even_and_odd(6);

  if result.is_ok() {
    println!("it's okay man, {:?}", result);
  } else {
    println!("it's not okay man!, {:?}", result);
  }
}