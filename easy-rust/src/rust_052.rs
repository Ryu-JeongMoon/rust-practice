use std::collections::HashMap;

fn print_city_hash_map() {
  let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
  let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

  let mut city_hashmap = HashMap::new();

  for city in canadian_cities {
    city_hashmap.insert(city, "Canada");
  }
  for city in german_cities {
    city_hashmap.insert(city, "Germany");
  }

  println!("{:?}", city_hashmap["Bielefeld"]);
  println!("{:?}", city_hashmap.get("Bielefeld"));
  println!("{:?}", city_hashmap.get("Bielefeldd"));
}

fn print_book_hash_map() {
  let mut book_hashmap = HashMap::new();

  book_hashmap.insert(1, "L'Allemagne Moderne");
  book_hashmap.insert(1, "Le Petit Prince");
  book_hashmap.insert(1, "shadow of your smile");
  book_hashmap.insert(1, "Eye of the World");

  // hashmap 에 소유권을 주고 싶지 않으니 & 레퍼런스 형태로 넘기고 값을 가져온당
  println!("{:?}", book_hashmap.get(&1));
}

pub fn print_hash_map() {
  print_city_hash_map();
  print_book_hash_map();
}

/*
hash_map[key] => 쌩으로 가져오는데 키가 없는 경우 panic
안전제일 타입깐깐 언어에서는 터지지 않게 하는 것이 중요하다
그렇기 땜시 값이 있든 없든, wrapper 로 받아오는 메서드를 사용하는 것이 베스트 프랙티스인가보다
 */