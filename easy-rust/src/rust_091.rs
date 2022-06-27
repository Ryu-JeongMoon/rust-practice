struct Adventurer<'a> {
  name: &'a str,
  hit_points: u32
}

impl Adventurer<'static> {
  fn take_damage(&mut self) {
    self.hit_points -= 20;
    println!("{} has {} hit points left!!", self.name, self.hit_points);
  }
}

struct Warrior<'a> {
  name: &'a str,
  hit_points: u32
}

impl Warrior<'_> {
  fn take_damage(&mut self) {
    self.hit_points -= 30;
    println!("{} has {} hit points left!!", self.name, self.hit_points);
  }
}

pub fn print_static_lifetime() {
  let mut me = Adventurer {
    name: "panda",
    hit_points: 9999
  };

  me.take_damage();

  let mut keyboard_warrior = Warrior {
    name: "bear",
    hit_points: 439434
  };
  keyboard_warrior.take_damage();
}

/*
struct Book {
  name: &'static str,
}
1 | struct Book<'static> {
  |             ^^^^^^^ 'static is a reserved lifetime name

Adventurer struct 선언할 때 name을 static으로 박아버리면 impl 할 때도 static 쓰라함둥
6 | impl Adventurer {
  |      ^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
  = note: assuming a `'static` lifetime...

'a는 generics로 쓸 때 쓰는 거고 impl에서 lifetime 가르쳐줄 때는 <'_> 쓴다리
옛날에는 impl <'a> Warrior<'a> 요렇게 써서 먼저 어떤 lifetime이 있는지 쓰고 그 다음에 그놈을 따라간다고 알려줬는데
번거롭고 반복적이기 땜시 <'_>로 퉁침
18 | impl Warrior<'a> {
   |     -        ^^ undeclared lifetime
   |     |
   |     help: consider introducing lifetime `'a` here: `<'a>`
 */