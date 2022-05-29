struct Container {
  value: u8,
}

impl Container {
  fn check_value(&self, input: u8) {
    println!("Are they equal? [{}]", self.value == input);
  }
}

pub fn print_dot_operator() {
  // let number = 32u8;
  // let ref_number = &number;
  // println!("is the same? {}", number == ref_number);

  let container = Container { value: 33u8 };
  container.check_value(33);
}

/*
let number = 32u8;
let ref_number = &number;
println!("is the same? {}", number == ref_number);
u8 <-> &u8 기본적으로 타입이 다른 것은 비교 자체가 불가

33을 그냥 넘기면 i32로 되어야 허는디 parameter 자리에 타입 명시를 했기 땜시롱 자동으로 u8 인식해버림?!
그것보다 중요한 것은 &self로 하여 self.value 때렸으면 &container.value가 되었을터인디 값이 비교가 되었다?!
요것은 모순 같지만 rust의 매직이다
설계 철학을 따라 비교 불가하게 만든다면 클라이언트 측에서 *, &, 등등
strange syntax 쑈가 열리고 명확성을 잃게 된다

자동으로 dereferencing 해준다는 것은 어느 정도 성능을 깎아 먹게 되는 건데 그럼에도 편의성을 위해 도입했다
&& 중첩 때릴 일이 많지도 않으니 depth가 깊이 내려가진 않을 것을 예상하고 만든 듯함
어쨋던 사용하는 입장에서 편해지긴함둥
 */