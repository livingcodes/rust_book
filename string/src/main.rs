fn main() {
  // string literal
  let hello = "hello";
  // 
  let world = String::from("world");

  let msg = world + " " + hello;

  // string concatenation requires an owned `String` on the left
  //let msg = hello + " " + world;

  println!("{msg}");

  // push_str
  let mut name = String::from("John");
  name.push_str(" Smith");
  println!("{name}");

  // Strings are UTF-8 encoded, so they can contain non-ASCII characters
  let translation = "Здра".to_string();
  println!("{translation}");
}
