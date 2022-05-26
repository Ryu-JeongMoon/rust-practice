// 아무 위치에서나 use 써서 prefix 줄일 수도 있당
use Mood::*;

#[derive(Debug)]
enum Mood {
  Happy,
  Sleepy,
  NotBad,
  Angry,
}

// match -> expression, 불필요한 변수 할당 없이 바로 리턴 가능!!
fn match_mood(mood: &Mood) -> i32 {
  match mood {
    Happy => 10,
    Sleepy => 6,
    NotBad => 7,
    Angry => 2
  }
}

enum ProgrammingLanguage {
  Go,
  Rust,
  Java,
  Kotlin,
}

// 콤퓨타는 enum을 숫자로 인식한당 -> ordinal, 0부터 시작
fn print_enum_in_ordinal() {
  use ProgrammingLanguage::*;
  let four_languages = vec![Go, Rust, Java, Kotlin];
  for language in four_languages {
    println!("{}", language as u8);
  }
}

pub fn print_enums() {
  let my_mood = Sleepy;
  let my_mood_level = match_mood(&my_mood);
  println!("my mood is = {:#?}", &my_mood);
  println!("out of 1 to 10, my mood level is = {}", &my_mood_level);

  print_enum_in_ordinal();
}