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

  // push char
  let mut lol = String::from("lo");
  lol.push('l');
  println!("{lol}");

  lol.push_str(" mr");
  println!("{lol}");

  let s1 = String::from("hello");
  let s2 = String::from("world");
  //let s3 = s1 + " " + s2; // err
  let s3 = s1 + " " + &s2;
  println!("{s3}");

  let a = String::from("tic");
  let b = String::from("tac");
  let c = String::from("toe");
  let d = format!("{a}-{b}-{c}");
  println!("{d}");
}
