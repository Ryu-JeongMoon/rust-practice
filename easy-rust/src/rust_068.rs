fn print_traditional_method() {
  let mut my_vec = Vec::new();
  let mut counter = 1;

  while counter < 11 {
    my_vec.push(counter);
    counter += 1;
  }

  // my_vec 을 debug-print 안 하면 터지넹, 허참!
  println!("{my_vec:?}");
}

fn print_fluent_method() {
  // turbo-fish 필수임둥
  let my_vec = (1..=65355).collect::<Vec<i32>>();

  let another_vec = my_vec
    .into_iter()
    .skip(535)
    .take(63)
    .collect::<Vec<_>>();
  println!("{another_vec:?}");
}

pub fn print_method_chaining() {

  // print_traditional_method();
  print_fluent_method();
}

/*
into_iter() -> stream() 동일
 */