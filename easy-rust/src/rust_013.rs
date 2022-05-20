fn r#return() -> u8 {
  15
}

// 경로 같은 것 표현할 때 raw text 이용하면 됨
fn print_raw_text() {
  println!(r#"c:\this_drive\new_drive"#)
}

pub fn print_with_tab() {
  print_raw_text();

  print!("\t Start with a tab\nand move to a new line\n");

  println!("
Inside quotes
you can write over
many lines
and it will print just fine.");

  let yahoo = r#return();
  println!("{}", yahoo)
}

/*
rust의 ""는 코틀린 & 자바의 """ """ 같이 동작한다
단 라인 시작을 따로 설정할 수는 없고 맨 첨부터 붙여야 한당
변수나 함수에 예약어를 사용하고 싶다면 r# 붙이고 쓰면 됨둥

이거 왜 컴파일 에러 나냐, `u8` doesn't implement `Display` (required by {}) ?!
let yahoo = r#return();
println!("{}", yahoo)
 */