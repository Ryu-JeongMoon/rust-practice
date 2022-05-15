pub fn print_type_inference() {
  let my_number = 9.6;
  let other_number = 9;
  let another_number: f32 = 134.3;

  println!("convert to i32 => {}", my_number as i32 + other_number);
  println!("convert to f64 => {}", my_number + other_number as f64);
  println!("convert to f32 => {}", my_number as f32 + another_number);
  println!("convert to f64 => {}", my_number + another_number as f64);
}

/*
float as 캐스팅 때리면 내림으로 동작

f64 -> double 인듯
크기 2배 차이나더라도 정확도 고려해서 f64 쓸 것
f64도 충분히 빠르다
 */