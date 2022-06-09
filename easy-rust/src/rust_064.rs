use std::fmt::Display;

fn print_vec<T: Display>(input: &Vec<T>) {
  for item in input {
    print!("{item} ")
  }
  println!();
}

#[derive(Debug)]
struct City {
  name: String,
  population: u32,
}

#[derive(Debug)]
struct Country {
  cities: Vec<City>,
}

impl City {
  fn new(name: &str, population: u32) -> Self {
    Self {
      name: name.to_string(),
      population,
    }
  }
}

impl From<Vec<City>> for Country {
  fn from(cities: Vec<City>) -> Self {
    Self { cities }
  }
}

impl Country {
  fn print_cities(&self) {
    for city in &self.cities {
      println!("{} has a population of {}", city.name, city.population);
    }
  }
}

pub fn print_implementing_from() {
  let my_vec = vec![7, 8, 9];
  print_vec(&my_vec);

  let other_vec = Vec::from([4, 5, 6]);
  print_vec(&other_vec);

  // byte로 변환시킨다
  let str_vec = Vec::from("What kind of this vec?");
  print_vec(&str_vec);

  let helsinki = City::new("Helsinki", 794_349);
  let turku = City::new("Turku", 595_431);
  let finland_cities = vec![helsinki, turku];

  // let finland = Country::from(finland_cities);
  let finland: Country = finland_cities.into();
  finland.print_cities();
}

/*
impl From 때리면 into() trait 자동 생성된다, 아래 두 코드는 같은 기능을 함둥
메서드 체이닝을 걸고 마지막에 어떤 타입으로 받고 싶다는 것을 명시하기 위해 into()를 쓴다고 하는데
아직 모르겄다 우히히

let finland = Country::from(finland_cities);
let finland: Country = finland_cities.into();
 */