// & immutable reference, shared reference
// &mut mutable reference, unique reference

pub fn print_mut_reference() {
  let mut my_number = 9;
  let num_ref = &mut my_number;

  // 오류
  // num_ref = 15;

  // 예쓰
  *num_ref = 15;

  println!("my number is {}", num_ref);
  println!("my number is {}", my_number);
}

/*
왜 unique reference 인가?
mutable reference 란 값을 변경시킬 수 있는 reference 임을 생각해보면
프로그램의 안전성을 지키기 위해서는 값을 바꿀 수 있더라도
값이 너덜너덜 해지도록 아무데서나 바꿀 수 있게 하는 것은 위험한 행동이다

자바로 생각해보면 setter 를 열어 아무데서나 변경 가능하게 만들어두면
유지보수성이 극악으로 치닫고 동시성 프로그래밍에서 사용하는 경우엔 race condition, deadlock 등이 뜰 수 있당
rust 는 매우 타입 깐깐 언어니까 이를 언어 차원에서 막아버린다
즉 값을 고칠 수 있게 만들어도 여러 개는 허용하지 않음둥

8 |   num_ref = 15;
  |             ^^ expected `&mut {integer}`, found integer
  |
help: consider dereferencing here to assign to the mutable borrowed piece of memory
reference 를 통해 값을 바꿀 때에는 반드시 dereferencing 갈긴 후에 바꿔야 한다
참조 값을 바꾸려는 시도기 때문?


dereference 는 무엇인고?
메모리 주소를 바라보고 있는 녀석을 값을 보게 한 후 바꿔쳐야 한다
*variable 형태로 갈길 수 있다
 */