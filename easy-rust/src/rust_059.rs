use std::fmt::{Debug, Formatter};

#[derive(Debug)]
struct MyStruct {
  number: usize,
}

fn print_as_debug<T>(input: T) where T: Debug {
  println!("{input:?}")
}

pub fn print_annotation() {
  print_as_debug("dfsakfkd");
}

/*
#[derive(Debug)]
위 표현은 내부적으로 아래와 같은 코드를 만든다, 자주 쓰니께 표현식을 미리 만들어둔 것임둥
impl TraitName for TypeName 형태로 작성하면 됨둥
impl Debug for MyStruct {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

rust에서는 저런 형태를 애노테이션이라 부른다
대표적으로 Copy, Clone, Debug는 많이 사용되기에 구현이 전부 되어있지만
Add 같은 놈은 Struct의 어떤 속성으로 더할지를 알 수 가 없으니 직접 trait를 만들어줘야 한다
lombok 같이 사용할 수가 있고만
 */