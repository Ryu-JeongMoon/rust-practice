use std::fmt::Debug;

#[derive(Debug)]
struct Monster {
  health: i32,
}

#[derive(Debug)]
struct Wizard {
  health: i32,
}

#[derive(Debug)]
struct Ranger {
  health: i32,
}

trait Magic {}

trait FightClose: Debug {
  // Now a type needs Debug to use FightClose
  fn attack_with_sword(&self, opponent: &mut Monster) {
    opponent.health -= 10;
    println!(
      "You attack with your sword. Your opponent now has {} health left. You are now at: {:?}", // We can now print self with {:?} because we have Debug
      opponent.health, &self
    );

    if opponent.health < 0 {
      println!("You got the {:?}", opponent);
    }
  }
  fn attack_with_hand(&self, opponent: &mut Monster) {
    opponent.health -= 2;
    println!(
      "You attack with your hand. Your opponent now has {} health left.  You are now at: {:?}",
      opponent.health, &self
    );

    if opponent.health < 0 {
      println!("You got the {:?}", opponent);
    }
  }
}

impl FightClose for Wizard {}

impl FightClose for Ranger {}

trait FightFromDistance: Debug {
  // We could also do trait FightFromDistance: FightClose because FightClose needs Debug
  fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
    if distance < 10 {
      opponent.health -= 10;
      println!(
        "You attack with your bow. Your opponent now has {} health left.  You are now at: {:?}",
        opponent.health, self
      );

      if opponent.health < 0 {
        println!("You got the {:?}", opponent);
      }
    }
  }
  fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
    if distance < 3 {
      opponent.health -= 4;
    }
    println!(
      "You attack with your rock. Your opponent now has {} health left.  You are now at: {:?}",
      opponent.health, self
    );

    if opponent.health < 0 {
      println!("You got the {:?}", opponent);
    }
  }
}

impl Magic for Wizard {}

impl FightFromDistance for Ranger {}

fn fireball<T>(character: &T, opponent: &mut Monster, distance: u32)
  where T: Magic + Debug {
  if distance < 15 {
    opponent.health -= 20;
    println!("you raise your hands in the air, and cast fire ball! puts your hands up!");
  }
}

pub fn print_trait_as_bounds() {
  let radagast = Wizard { health: 60 };
  let aragorn = Ranger { health: 80 };

  let mut uruk_hai = Monster { health: 40 };

  radagast.attack_with_sword(&mut uruk_hai);
  aragorn.attack_with_bow(&mut uruk_hai, 8);
  aragorn.attack_with_bow(&mut uruk_hai, 8);
  aragorn.attack_with_bow(&mut uruk_hai, 8);
  aragorn.attack_with_bow(&mut uruk_hai, 8);
  fireball(&radagast, &mut uruk_hai, 11);
}