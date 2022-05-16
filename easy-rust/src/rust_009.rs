fn empty_tuple() {}

fn return_integer() -> i32 {
  8
}

pub fn print_tuple() {
  let my_number = empty_tuple();
  println!("{:?}", my_number);

  let x = return_integer();
  println!("{}", x);
}

/*
빵숀 안에 ; 있으면 empty tuple 준다, 자바의 void 개념
따라서 main 빵숀도 main() -> () 이거였듬

함수에서 값 반환시키고 싶으면 반환 타입 명시해야하고 return 쓰던가 말던가
마지막 라인에 세미 콜론 빼고 반환할 값을 넣어준다

`fn()` doesn't implement `Display` (required by {})
빵숀은 프린트할 수 없다
{:?} 특수 문법으로 debug print 라 한다
 */