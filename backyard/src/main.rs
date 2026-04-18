// declare mod
pub mod garden;
use crate::garden::Flower;
fn main() {
  let daisy = Flower {
    name: String::from("Daisy"),
    color: Color::Red
  };
  println!("hello {}", &daisy.name);
}

pub enum Color {
  Red,
  Green,
  Blue
}