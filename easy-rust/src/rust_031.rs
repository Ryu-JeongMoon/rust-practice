fn match_colours(rgb: (u32, u32, u32)) {
  match rgb {
    (r, _, _) if r < 10 => println!("not much red"),
    (_, g, _) if g < 10 => println!("not much green"),
    (_, _, b) if b < 10 => println!("not much blue"),
    _ => println!("every colour has at least 10")
  }
}

fn match_number(input: i32) {
  match input {
    number @ 0..=10 => println!("interesting... {}", number),
    n @ 11..=20 => println!("interesting too... {}", n),
    _ => println!("got ya anything else")
  }
}

pub fn print_complicated_match_statements() {
  let first_colours = (200, 0, 0);
  let second_colours = (50, 50, 50);
  let third_colours = (200, 50, 0);

  match_colours(first_colours);
  match_colours(second_colours);
  match_colours(third_colours);

  let my_number = 155;
  let other_number = if my_number < 256 { "yaho".to_string() } else { "hooray".to_string() };
  println!("it's actually not a number = {}", other_number);

  let number_input = 15;
  match_number(number_input);
}

/*
요래 보니까 match 안의 if가 쓰임이 많을 것 같기는 하구먼
match 사용 시 조건에 신경써줘야 한다
early-return 때리기 때문인디 걸리는 값 있으면 바로 리턴시키고 뒤도 안 돌아본다

match, if 동일하게 조건에 따라 다른 타입을 반환할 수 없다
타입 깐깐 언어라서 컴파일러가 컴파일 하는 순간 타입을 알 수 있어야 하는데 쟤네는 돌려봐야 알 수 있기 땜시일까?
어쨋든 incompatible type 뜨니까 그럴 순 없둥

ternary operator가 없는 대신 if-else로 표현할 수 있다
if-else 가 statement 로 인식 되기 때문이고 요즘 언어 답구먼

match 안에서 범위로 매칭시킬 때는 변수를 쓸 수가 없잖아?!
<변수 @ 범위> 형태로 매칭시켜서 변수를 출력해주면 된다리
 */