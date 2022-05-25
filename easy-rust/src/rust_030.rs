pub fn print_match_statements() {
  let sky = "cloudy";
  let temperature = "cold";

  match (sky, temperature) {
    ("cloudy", "warm") => println!("it's a goooood day"),
    ("cloudy", "cold") => println!("it's a coooool day"),
    _ => println!("i don't care what wheather is")
  }

  let children: i128 = 15534348329423493242349234234;
  let married = true;
  match (children, married) {
    (children, married) if children < 16 && married => println!("happy family"),
    (child, marry) if child > 43934034 && !marry => println!("weird man"),
    (c, m) if c > 1 && m => println!("yahoo"),
    _ => println!("i don't care")
  }
}

/*
tuple 을 이용해 다양한 matching 가능
모든 케이스를 고려할 수 없는 경우, 자바의 default 처럼 이 외의 상황에 대한 처리도 필요한데
rust 는 (_) 언더바를 이용한다, 요 놈은 아래와 같은 경우에 사용할 수 있다리
1. 값을 할당할 필요가 없는 경우
2. anything else 를 나타내는 경우

tuple로 match 돌리는 경우, 매칭시킬 때 임시 변수의 이름을 다르게 줄 수 있다
match 선언할 때 명시적으로 주고 안에서 매칭시킬 때는 간략하게 쓰는 방식이 좋을듯
match 안에서 조건 분기를 또 할 수 있는디 썩 좋아보이진 않음둥
<매칭 시킬 값 if 조건 => 어쩔시구리> 형태로 사용
 */