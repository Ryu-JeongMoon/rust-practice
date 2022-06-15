fn print_filtered_months() {
  let months = vec![
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December"];

  let filtered_months = months.into_iter()
    .filter(|month| month.len() < 5 && month.contains("u"))
    .collect::<Vec<&str>>();

  println!("{:?}", filtered_months);
}

pub struct Company {
  pub name: String,
  ceo: Option<String>,
}

impl Company {
  pub(crate) fn new(name: &str, ceo: &str) -> Self {
    let ceo = match ceo {
      "" => None,
      ceo => Some(ceo.to_string())
    };

    Self {
      name: name.to_string(),
      ceo,
    }
  }

  pub(crate) fn get_ceo(&self) -> Option<String> {
    self.ceo.clone()
  }
}

fn print_company() {
  let company_vec = vec![
    Company::new("Umbrella Corporation", "Unknown"),
    Company::new("Ovintiv", "Doug Suttles"),
    Company::new("The Red-Headed League", ""),
    Company::new("Stark Enterprise", ""),
    Company::new("Apple", "Tim Cook"),
    Company::new("Samsung", "Lee"),
  ];

  let all_the_ceos = company_vec
    .into_iter()
    .filter_map(|company| company.get_ceo()) // Some, None에 따라 필터링 된다
    .collect::<Vec<String>>();

  println!("{all_the_ceos:?}");
}

pub fn print_filter() {
  // print_filtered_months();
  print_company();
}

/*
요런 것도 줄여버리네잉
filter() + map() => filter_map()
 */