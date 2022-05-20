const HIGH_SCORE: i32 = 99;

#[warn(non_upper_case_globals)]
const low_score: i32 = 16;

static mut MIDDLE_SCORE: i32 = 55;

pub fn print_const() {
  println!("high score is {}", HIGH_SCORE);
  println!("low score is {}", low_score);

  unsafe { MIDDLE_SCORE = 60 };
  unsafe { println!("middle score is {}", MIDDLE_SCORE); }
}

/*
const high_score: i32 = 16;
위와 같이 썼을 때 컴파일 되지만 경고로 const should be upper case name 뜸, 관례긴 한데 강한 관례?!
그냥 소문자로 쓰고 싶고 컴파일러한테 나대지 말라고 하려면
#[warn(non_upper_case_globals)], attribute라 하는 요놈을 위에다 붙이면 된다

const, static 쓸 때는 컴파일러가 타입 추론 안 해준다, 반드시 타입을 명시해야 함
static mut 선언 시, 언어 철학을 위배하는 것과 다름 없다
어쩔 수 없이 불변성을 헤치고 써야하는 경우를 위해 초기에 unsafe 키워드를 만들어 두긴 했지만
안전성을 중요시하는 언어 철학도 있고 대세가 점점 불변성으로 가니 unsafe 쓰면 미친놈임
local scope 에서 값 복사해서 해결하면 됨
 */