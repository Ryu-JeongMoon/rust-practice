use std::rc::Rc;

#[derive(Debug)]
struct City {
  name: String,
  population: u32,
  history: Rc<String>,
}

#[derive(Debug)]
struct CityData {
  names: Vec<String>,
  histories: Vec<Rc<String>>,
}

pub fn print_rc() {
  let calgary = City {
    name: "Calgary".to_string(),
    population: 1_336_000,
    history: Rc::new("Calgary was founded in blah blah blah".to_string()),
  };

  let canada_cities = CityData {
    names: vec![calgary.name],
    histories: vec![Rc::clone(&calgary.history)],
  };

  // println!("{calgary:?}");
  println!("{canada_cities:?}");
  println!("Data has {} weak owners", Rc::weak_count(&calgary.history));
  println!("Data has {} strong owners", Rc::strong_count(&calgary.history));
}

/*
28 |   println!("{calgary:?}");
   |              ^^^^^^^ value borrowed here after partial move
calgary.name을 canada_cities 할당한 후 출력하려 했기 땜시 partial move 한 후 호출했다는 에러 뿜는다

clone()을 쓰는게 간편할 수도 있지만 Rc<_> 사용할 수도 있둥
Rc::strong_count(&data) -> data를 몇명이 소유하는지 확인하는 메서드

데이터의 연관 관계에 따라 strong_count만 존재한다면 데이터가 drop 될 수 없는 상황이 발생할 수 있다
요럴 때 weak_count를 사용하면 count에는 포함되지만
다른 strong_count가 drop 될 때 요놈 역시 따라서 drop된다
 */