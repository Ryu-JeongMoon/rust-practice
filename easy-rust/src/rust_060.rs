struct Animal {
  name: String
}

trait Canine {
  fn bark(&self) {
    println!("woof! woof!");
  }

  fn run(&self) {
    println!("i'm running!");
  }
}

impl Canine for Animal {
  fn bark(&self) {
    println!("멍멍! 멍!")
  }
}

pub fn print_trait() {
  let my_dog = Animal {
    name: "bbosil".to_string()
  };

  my_dog.bark();
  my_dog.run();
}

/*
trait란 무엇인가?
interface 로 이해하면 쉽다, 설명에서는 초능력이라고 한 것으로 보아 완전 일치하는 개념은 아닌듯 하다
특정한 속성을 추가한다는 개념으로 쓰나봄둥

인터페이스와 기능이 유사하여 default method도 가능하고,
method 선언이 목적이며 구현은 알아서 해라 라는 것도 비슷함둥
 */