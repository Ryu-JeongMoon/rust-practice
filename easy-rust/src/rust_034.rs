// rust enum 에서는 대문자 관례가 없나보다리
enum ThingsInTheSky {
  Sun,
  Stars,
  Other
}

fn create_sky_state(time: u8) -> ThingsInTheSky {
  match time {
    6..=17 => ThingsInTheSky::Sun,
    18..=24 | 0..=5 => ThingsInTheSky::Stars,
    _ => ThingsInTheSky::Other
  }
}

fn check_sky_state(sky: &ThingsInTheSky) {
  match sky {
    ThingsInTheSky::Sun => println!("i can see the blue sky and the sun!!"),
    ThingsInTheSky::Stars => println!("i can see the glittery stars~~"),
    ThingsInTheSky::Other => println!("i can't see anything")
  }
}

pub fn print_enums() {
  // 명료하게 작성?
  // let current_time = 16;
  // let things_in_the_sky = create_sky_state(current_time);
  // check_sky_state(&things_in_the_sky);

  // rust style
  check_sky_state(&create_sky_state(16));
}