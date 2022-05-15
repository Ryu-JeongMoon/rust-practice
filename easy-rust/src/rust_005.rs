pub fn print_length_and_chars_count() {
  let normal_text = "yahoo";
  let panda_face = "🐼🐼🐼🐼🐼";
  println!("yahoo => {}", normal_text);
  println!("panda face => {}\n", panda_face);

  println!("rust length means bytes");
  println!("panda face length => {}", panda_face.len());
  println!("normal text length => {}", normal_text.len());
}