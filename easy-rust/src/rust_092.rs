use std::fmt;
use std::fmt::Formatter;

use crate::rust_092::client::InternetClient;

mod client {
  pub struct InternetClient {
    pub(crate) client_id: u32,
  }
}

struct Customer<'a> {
  money: u32,
  name: &'a str,
  client: &'a InternetClient,
}

impl fmt::Debug for Customer<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("Customer")
      .field("money", &self.money)
      .field("name", &self.name)
      .field("client", &self.client.client_id)
      .finish()
  }
}

pub fn print_dbg() {
  let client = client::InternetClient {
    client_id: 1
  };

  let my_customer = Customer {
    money: 5432542,
    name: "panda",
    client: &client,
  };

  println!("{my_customer:?}");
}

/*
external code 사용할 때 debug 구현 안 해놨다면?!
그 구조체를 갖고 있는 놈의 debug_format 을 직접 만들면 된다
 */