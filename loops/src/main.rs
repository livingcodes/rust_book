fn main() {
  let mut i = 0;
  let result = loop {
    i = i + 1;
    println!("i:{i}");

    if i == 10 {
      break i * 2;
    }
  };

  println!("result:{result}");
}
