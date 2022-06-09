struct Monster {
  health: u32,
}

struct Wizard {}

struct Ranger {}

trait FightClose {
  // default implementation
  fn attack_with_sword(&self, opponent: &mut Monster) {
    opponent.health -= 10;
    println!("you strike with sword!, your opponent's health is {}", opponent.health);
  }

  fn attack_with_hand(&self, opponent: &mut Monster) {
    opponent.health -= 2;
    println!("you strike with hand!, your opponent's health is {}", opponent.health);
  }
}

trait FightFromDistance {
  fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
    if distance > 10 || distance < 0 {
      return;
    }

    opponent.health -= (10 - distance);
    println!("you attack with bow!, your opponent's health is {}", opponent.health);
  }

  fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
    if distance > 5 || distance < 0 {
      return;
    }

    opponent.health -= (5 - distance);
    println!("you attack with rock!, your opponent's health is {}", opponent.health);
  }
}

impl FightClose for Wizard {}

impl FightClose for Ranger {}

impl FightFromDistance for Ranger {}

pub fn print_another_trait() {
  let gandalf = Wizard {};
  let aragorn = Ranger {};

  let mut uruk_hai = Monster {
    health: 99
  };

  gandalf.attack_with_hand(&mut uruk_hai);
  gandalf.attack_with_sword(&mut uruk_hai);

  aragorn.attack_with_sword(&mut uruk_hai);
  aragorn.attack_with_rock(&mut uruk_hai, 3);
  aragorn.attack_with_bow(&mut uruk_hai, 3);
  aragorn.attack_with_bow(&mut uruk_hai, 0);
}