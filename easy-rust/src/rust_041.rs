use rust_039::Animal;
use rust_039::AnimalType;

use crate::rust_039;

impl AnimalType {
  fn check_type(&self) {
    use AnimalType::*;
    match self {
      Cat => println!("animal type is cat"),
      Dog => println!("animal type is dog")
    }
  }
}

#[derive(Debug)]
enum PandaType {
  Violent(String),
  Gentle(String),
}

impl PandaType {
  fn check_characteristic(&self) {
    use PandaType::*;
    match self {
      Violent(name) => println!("violent panda's name is {}", name),
      Gentle(name) => println!("gentle panda's name is {}", name),
    }
  }
}

pub fn print_enum_impl_blocks() {
  use AnimalType::*;

  let my_cat = Animal::new_cat(10);
  let my_dog = Animal::new_dog(25);

  my_cat.animal_type.check_type();
  my_dog.animal_type.check_type();

  let gentle_panda = PandaType::Gentle("joshua".to_string());
  let violent_panda = PandaType::Violent("bloch".to_string());

  gentle_panda.check_characteristic();
  violent_panda.check_characteristic();
}

/*
다른 파일을 불러올 때 main.rs에 mod 설정 되어 있어야 인식함둥..
struct or enum 변수는 기본 설정이 private으로 되어있나봄둥, 직접 pub으로 열어줘야 인식함
 */