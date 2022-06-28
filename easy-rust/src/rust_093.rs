use std::cell::Cell;

#[derive(Debug)]
struct PhoneModel {
  company_name: String,
  model_name: String,
  screen_size: f32,
  memory: usize,
  date_issued: u32,
  on_sale: Cell<bool>,
}

// on_sale을 바꾸고 싶은데 super_phone을 mut로 하고 싶지 않다
pub fn print_mutability_and_cells() {
  let iphone_15 = PhoneModel {
    company_name: "Apple".to_string(),
    model_name: "iPhone15".to_string(),
    screen_size: 7.5,
    memory: 4_000_000,
    date_issued: 2022,
    on_sale: Cell::new(true),
  };
  println!("{iphone_15:?}");

  iphone_15.on_sale.set(false);
  println!("{iphone_15:?}");
}

/*
interior mutability
changing on the inside

& -> immutable reference, shared reference
&mut -> mutable reference, unique reference
요렇게만 하면 유연하게 바꿀 수 없기 때문에 Cell(small room)을 제공

thread-1, thread-2의 race condition rust는 어떻게 막는가?
Cell이라는 작은 방을 주고 이 놈 안의 값은 & (reference) 쓸 수 없음둥
얘는 set을 사용해 명시적으로 바꿔줘야한다
 */