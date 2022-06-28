use std::cell::RefCell;

pub fn print_refcell() {
  let my_cell = RefCell::new(String::from("I am a String"));
  println!("{my_cell:?}");

  let blocking_reference = my_cell.borrow_mut();

  match my_cell.try_borrow_mut() {
    Ok(mut r) => *r = String::from("I am not a String"),
    Err(e) => println!("we got an error {e}")
  }
  println!("{my_cell:?}");

}

/*
try_borrow_mut()를 통해 orElseThrow 구현할 수 있둥
 */