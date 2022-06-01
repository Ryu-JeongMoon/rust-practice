use std::collections::{BTreeMap, HashMap};

struct City {
  name: String,
  // population: HashMap<u32, u32>,
  population: BTreeMap<u32, u32>,
}

pub fn print_map() {
  let mut tallinn = City {
    name: "Tallinn".to_string(),
    population: BTreeMap::new(),
  };

  let mut option = tallinn.population.insert(1372, 0);
  println!("does inserted ? => {:?}", option); // None

  option = tallinn.population.insert(1372, 3_250);
  println!("does inserted ? => {:?}", option); // Some(0)

  tallinn.population.insert(1851, 24_000);
  tallinn.population.insert(2020, 437_619);

  for (year, population) in tallinn.population {
    println!("In the year {}, the population of Tallinn was {}", year, population);
  }
}

/*
rust HashMap은 insert 제공하는데 자바의 putIfAbsent 를 생각해보면 된다리
기존에 값이 없었다면 삽입한 후, None 반환하고
기존에 값이 있었다면 삽입한 후, 기존 값을 반환한다

HashMap -> 재빨라지기 위해 ordering 포기했다
순서가 중요하다면 성능을 좀 깎아먹더라도 BTreeMap 사용하면 된다
제공하는 메서드들은 거의 유사하고 성능 차이뿐임둥
 */