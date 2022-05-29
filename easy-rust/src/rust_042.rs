#[derive(Debug)]
struct Panda {
  name: String,
  real_name: String,
  height: u8,
  happiness: bool,
}

#[derive(Debug)]
struct SimplePanda {
  name: String,
  height: u8,
}

// dto <-> entity 변환 맹키로 사용하는 fn, destructuring 이용해 필요한 변수만 뽑고 나머지는 .. 처리로 무시
impl SimplePanda {
  fn from_panda(input: Panda) -> Self {
    let Panda { name, height, .. } = input;
    Self { name, height }
  }
}

pub fn print_destructuring() {
  let grizzly = Panda {
    name: "grizzly".to_string(),
    real_name: "김철수".to_string(),
    height: 255,
    happiness: true,
  };

  // 변수명 다시 다 써줘야 해서 매우 귀찮군
  let Panda { name, real_name, height, happiness } = &grizzly;
  println!(
    "his name is known as {}, but his real name is {}. he is {} cm tall, and is he happy? {}",
    name, real_name, height, happiness
  );

  let simple_panda = SimplePanda::from_panda(grizzly);
  println!("simple panda type is {:?}", simple_panda)
}