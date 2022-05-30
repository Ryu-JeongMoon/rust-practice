use crate::rust_046::take_safe_fifth;

fn print_with_pattern_matching(input: &Vec<i32>) {
  match take_safe_fifth(input.clone()) {
    Some(number) => println!("i got a number = {}", number),
    None => println!("i got nothing")
  }
}

fn print_with_if_else(input: &Vec<i32>) {
  let value = take_safe_fifth(input.clone());
  if value.is_some() {
    println!("yahoo ~! = {}", value.unwrap())
  } else {
    println!("yahoo ~! = {:?}", value)
  }
}

// thread 'main' panicked at 'yahoo~~~~~~~~~~~~~~~~~~', src/rust_047.rs:22:26
fn print_with_expect(input: &Vec<i32>) {
  let value = take_safe_fifth(input.clone());
  let real_value = value.expect("yahoo~~~~~~~~~~~~~~~~~~");
  println!("yahoo ~! = {}", real_value);
}

pub fn print_unwrap_option() {
  let new_vec = vec![1, 3, 5];

  print_with_pattern_matching(&new_vec);
  print_with_if_else(&new_vec);
  print_with_expect(&new_vec);
}

/*
```
let new_vec = vec![1, 3, 5];
let option = take_safe_fifth(new_vec.clone());
let real_value = option.unwrap();
println!("value = {}", real_value);
```
Option.unwrap() -> Optional.get()
None에다가 언랩 갈기면 바로 패닉 빠져부림

Option을 어떻게 뱃겨버릴까
1. pattern matching
2. if-else
3. expect 예외 메시지 담아서 터트리기
 */