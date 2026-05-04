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
}
