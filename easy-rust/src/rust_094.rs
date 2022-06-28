use std::cell::{Cell, RefCell};

#[derive(Debug)]
struct User {
  id: u32,
  year_registered: u32,
  username: String,
  active: RefCell<bool>,
}

pub fn print_refcell() {
  let my_cell = Cell::new(String::from("I am a String"));
  println!("{}", my_cell.take());

  let user_1 = User {
    id: 1,
    year_registered: 2022,
    username: "panda".to_string(),
    active: RefCell::new(true)
  };
  println!("{user_1:?}");

  // first_reference의 값을 바꾸기 위해 mut로 선언해주어야 함
  let mut first_reference = user_1.active.borrow_mut();
  println!("{user_1:?}");

  // dereference로 값을 바꿨더라도 아직 borrowed 상태라서 안 보임
  *first_reference = false;
  println!("{user_1:?}");

  // 명시적으로 borrowed 상태를 바꾸려면 drop() 사용해야 함
  drop(first_reference);
  println!("{user_1:?}");
}

/*
RefCell -> runtime checked borrowing rules ?!

system programming을 위해서 안전성이 와따여야 한다
rust는 잘못된 코드를 만나면 panic에 빠지고 집어던지고 죽어버린다
죽기 전에 unwind the stack으로 데이터를 다시 감아서 OS에 반납한다

let first_reference = user_1.active.borrow_mut();
let second_reference = user_1.active.borrow_mut();
thread 'main' panicked at 'already borrowed: BorrowMutError'
borrow_mut() 두번 때리면 컴파일은 잘 되지만 BorrowMutError를 던져뿐다
 */