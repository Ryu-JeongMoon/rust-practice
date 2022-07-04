use std::rc::Rc;

fn takes_a_string(input: Rc<String>) {
  println!("{input}");
}

fn also_takes_a_string(input: Rc<String>) {
  println!("{input}");
}

pub fn print_reference_counting() {
  let my_string = Rc::new("Hello Rust".to_string());

  takes_a_string(Rc::clone(&my_string));
  also_takes_a_string(Rc::clone(&my_string));

  takes_a_string(my_string.clone());
  also_takes_a_string(my_string.clone());
}

/*
Rc::clone(&my_string), associated function syntax ?!
일반적으로 인자가 없는 거의 모든 메서드에서
syntax sugar를 사용해 반복되는 작업을 줄일 수 있지만 사실 위처럼 &self 넘기고 있었듬

다만 Rc를 사용할 때는 명시적으로 보여줄 수 있다는 점에서
Rc::clone(&my_string) 처럼 fully qualified syntax를 사용하는 것이 낫다고 함둥
 */