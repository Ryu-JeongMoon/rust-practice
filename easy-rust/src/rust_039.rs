#[derive(Debug)]
pub struct Animal {
  pub age: u8,
  pub animal_type: AnimalType,
}

#[derive(Debug)]
pub enum AnimalType {
  Cat,
  Dog,
}

// 특정 타입에 적용할 수 있는 빵숀 만들고 싶을 때 impl 사용한다
// 어쨋던 Self 자리에는 Animal이 들어가는 자리인데 rust에서는 Self를 선호한다
// Animal이 바뀌어도 Self로 해놓은 건 바꾸지 않아도 되기 때문임둥
impl Animal {
  pub(crate) fn new_dog(age: u8) -> Self {
    Self {
      age,
      animal_type: AnimalType::Dog,
    }
  }

  pub(crate) fn new_cat(age: u8) -> Self {
    Self {
      age,
      animal_type: AnimalType::Cat,
    }
  }

  pub(crate) fn print(&self) {
    println!("i'm a {:?}", self);
  }

  pub(crate) fn change_to_dog(&mut self) {
    self.animal_type = AnimalType::Dog;
    println!("now i'm a {:?}", self);
  }

  pub(crate) fn change_to_cat(&mut self) {
    self.animal_type = AnimalType::Cat;
    println!("now i'm a {:?}", self);
  }
}

pub fn print_impl() {
  let my_dog = Animal::new_dog(55);
  println!("i made a {:?}, his age is {}", my_dog.animal_type, my_dog.age);
}

/*
<Animal::어쩌구 메서드> 형태로 사용하는 것은 associated function 이라 한다
인스턴스에다 연산 갈기는 것이 아니다
static method 마냥 그 타입으로 호출할 수 있둥
 */