fn print_raw_closure() {
  let my_number = 11;
  let anonymous_function = || println!("i am a function");
  let closure = || println!("{my_number}");
}

fn print_vec_closure() {
  let my_vec = vec![8, 9, 10];

  let value = my_vec.get(3).unwrap_or_else(|| {
    if my_vec.get(0).is_some() {
      &my_vec[0]
    } else {
      &0
    }
  });

  println!("{value}");
}

pub fn print_zero_cost_abstraction() {
  print_raw_closure();
  print_vec_closure();
}

/*
같은 스코프 내에 선언된 변수를 사용하느냐 안 하느냐로 익명함수와 클로저로 나뉜다
러스트에서 || (pipes)를 사용하는 대부분의 경우는 클로저고,
중요하진 않지만 미묘한 차이는 다음과 같다
anonymous_function 자바의 () -> 같은 기능으로 || 어쩌구저쩌구로 구현한다
closure || 어쩌구 + 스코프 내 변수를 캡처링해서 구현한다

zero cost abstraction
.iter().map().filter().inspect().collect() 같이 작성하더라도 기존에 풀어 쓴 형태랑 성능 차이가 없다?!
그래서 zero cost 라 하고 로우 레벨은 러스트가 알아서 해주니까 하이 레벨로 작성하라는 것임둥
단 compile time 을 더 잡아 먹는다고 함둥

unwrap_or_else() == orElseGet()
값이 없는 경우에 반환해야 할 값을 클로저가 길게 작성해도 되게 해준당
 */