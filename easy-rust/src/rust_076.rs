// chars() iterator of char
// count() counts number of items in chars

pub fn print_char_method() {
  let big_string = "hello there, i am a &str";

  big_string.chars().for_each(|c| println!("{c}"));

  // chars(), char_indices() 선언하고 변수 안 쓰고 싶다면 _ 이용하면 되는데 흔치 않을 일,,
  big_string.char_indices().for_each(|(index, char)| println!("at index : {index}, the character : {char}"));
  println!("{big_string:?} has {} characters", big_string.chars().count());
}