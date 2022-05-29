use std::fmt::Display;

fn give_thing<T: Display>(input: T) -> T {
  println!("yahoo = {}", input);
  input
}

pub fn print_generics() {
  let thing = give_thing(String::from("panda"));
  let other_thing = give_thing(341431409134093104i64);
  println!("the thing is {} and other is {}", thing, other_thing);
}

/*
1 | fn give_thing(input: T) -> T {
  |              -       ^ not found in this scope
  |              |
  |              help: you might be missing a type parameter: `<T>`
type parameter 뺴먹으면 요래 나옴, 친절하구만

concrete & generic
상속? 구현?은 <T: 어쩌구> 형태로 나타냄
std lib도 안 넣어주는 rust,,
 */