fn print_if_let() {
  let my_vec = vec![2, 3, 4];

  for index in 0..=10 {
    if let Some(number) = my_vec.get(index) {
      println!("the number is {}", number);
    }
  }
}

fn print_while_let() {
  let weather_vec = vec![
    vec!["Berlin", "cloudy", "5", "-7", "78"],
    vec!["Athens", "sunny", "not humid", "20", "10", "50"],
  ];

  for mut city in weather_vec {
    println!("For the city of {}:", city[0]);

    while let Some(information) = city.pop() {
      if let Ok(number) = information.parse::<i32>() {
        println!("  the number is {}", number);
      }
    }
  }
}

pub fn print_if_let_and_while_let() {
  print_if_let();
  print_while_let();
}

/*
match 어쩌구 {} statement 에서는 관심이 없는 놈이라도 언더바(_)를 이용해 매칭 시켜야 한다
이보다 간단하게 사용할 수 있는 것이 if let 이고 요놈은 == 비교가 아닌 pattern matching 이기 땜시
<if let 비교할 값 = 조건> 형태로 사용, 코틀린에 <조건.let{}> 형태와 유사하다리

<T>() => 물고기 같이 생겼다고 turbo fish 라 함둥
 */
