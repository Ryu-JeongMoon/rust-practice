pub fn print_integer_types() {
  let u8: u8 = 1 << 8 - 1;
  let u16: u16 = 1 << 16 - 1;
  let u32: u32 = 1 << 32 - 1;
  let u64: u64 = 1 << 64 - 1;
  let u128: u128 = 1 << 128 - 1;
  println!("{}, {}, {}, {}, {}", u8, u16, u32, u64, u128);

  let first_number : u8 = 55;
  let second_number: u16 = 14;
  println!("{}", first_number as u16 + second_number);
}

/*
i8, i16, i32, i64, i128
u8, u16, u32, u64, u128
byte, short, long 요딴 거 안 쓰고 i + 사이즈 붙여서 사용한다
byte, short 인식이 안 됨둥

rust 매우 엄격해서 같은 숫자 타입을 쓰더라도 사이즈 다르면 터트려버린다
as 로 simple type casting 해줘야 함
 */