use std::cell::Cell;

trait SuperCoolTrait {
  fn cool_function(&self);
}

struct User {
  id: u32,
  times_used: Cell<u32>
}

impl SuperCoolTrait for User {
  fn cool_function(&self) {
    println!("now using cool_function!");
    let current_times_used = self.times_used.get();
    self.times_used.set(current_times_used + 1);
  }
}

pub fn print_interior_mutability() {
  let user = User {
    id: 32498234,
    times_used: Cell::new(0)
  };

  for _ in 0..=2000 {
    user.cool_function();
  }

  println!("{}", user.times_used.get());
}

/*
Why interior mutability?
Rc, reference counter -> 소유권자가 몇명인지 센다?!
Arc, atomic reference counter -> thread-safe

사용하는 struct의 값에 직접 때려박는 코드를 방지하고
특히 external code 같은 경우 바꿀 방법이 없슴둥
 */