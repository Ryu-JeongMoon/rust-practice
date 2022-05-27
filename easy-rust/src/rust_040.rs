// 다른 파일 불러올 때, pub 설정 되어있어야 하고 <use crate::어쩌구> 형태로 불러옴둥
use crate::rust_039::Animal;

pub fn print_more_impl() {
  let mut my_animal = Animal::new_dog(155);
  my_animal.print();
  my_animal.change_to_dog();
  my_animal.change_to_cat();

  println!("my animal = {:?}", my_animal);
}

/*
&self 에는 암묵적으로 자기가 쏙 들어가서 명시적으로 넘겨주지 않아도 된당
impl 은 여러번 중복으로 만들어도 되지만 일반적으로 하나의 덩어리로 만든다
그래야 유지보수가 됭게
 */