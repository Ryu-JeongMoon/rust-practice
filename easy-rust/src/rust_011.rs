pub fn print_immutable() {
  let my_variable = 10;

  // 에러 뿡
  // my_variable = 9;
}

pub fn print_shadowing() {
  let my_variable = 10;
  let my_variable = "yahoo";
  println!("{}", my_variable);
}

pub fn print_shadowing_in_scopes() {
  let my_variable = 9;

  {
    println!("{}", my_variable);
    let my_variable = "new string";
    println!("{}", my_variable);
  }

  println!("{}", my_variable);
}

/*
- immutability
Cannot assign twice to immutable variable [E0384]
let -> const in js (immutable by default)
let mut -> let or var in js
rust 는 안전성을 중요하게 생각하는 언어라 기본적으로 재할당할 수 없게 만들어놨다고 한다

shadowing
코드 블록은 지역 스코프를 만들어내 그 안에서 동일 변수명으로 선언하더라도 스코프 벗어나면 원래 값이 읽힌다
겹치는 변수명 발생 시 가장 마지막에 선언한 값이 들어간다
요건 좀 이상헌디 같은 이름으로 변수 선언 시 터트려야 맞지 않남
 */