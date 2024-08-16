use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
struct Redacted<T: Display + Debug>(T);

impl<T> From<T> for Redacted<T>
where
  T: Display + Debug,
{
  fn from(value: T) -> Self {
    Self(value)
  }
}

impl<T> Display for Redacted<T>
where
  T: Display + Debug,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "<Redacted>")
  }
}

#[derive(Debug)]
#[bon::builder]
struct User {
  id: String,
  name: String,
  email: Redacted<String>,
  subscribe_to_newsletter: bool,
}

impl User {
  fn subscribe_to_newsletter(&mut self, value: bool) {
    self.subscribe_to_newsletter = value;
    // your custom logger here
    println!(
      "User with email: {} has changed preferences to {}\n",
      self.email, self.subscribe_to_newsletter
    );
  }
}

fn main() {
  let mut u1 = User::builder()
    .id("user_1")
    .name("John Doe")
    .email("john@example.com".to_string().into())
    .subscribe_to_newsletter(false)
    .build();

  println!("Actual value to debug: {:?}\n", u1.email);
  u1.subscribe_to_newsletter(true);
  u1.subscribe_to_newsletter(false);
}
