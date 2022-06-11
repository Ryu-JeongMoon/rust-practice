use std::fmt::Display;

fn print_it<T: Display + AsRef<str>>(input: T) {
  println!("{input}");
}

pub fn print_as_ref() {
  print_it("yahoo!");
  // print_it(19500625);
}

/*
요래 선언해뿌면 모든 타입에 대해 적용된다
fn print_it<T: Display>(input: T) { 어쩌구 저쩌구 }

난 String에 대해서만 하고 싶은디?!
AsRef<str>로 타입 한정을 해줄 수 있따, 자바의 <T extends CharSequence> 같은 것이로구만
fn print_it<T: Display + AsRef<str>>(input: T) { 어쩌구 저쩌구 }

AsRef<str> 했는데도 냅다 갈겨버리면 메서드 콜이 제한되어있다고 알려줌둥
9 |   print_it(19500625);
  |   -------- ^^^^^^^^ the trait `AsRef<str>` is not implemented for `{integer}`
  |   |
  |   required by a bound introduced by this call
 */