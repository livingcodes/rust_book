use std::io;

fn main() {
  let a = [1, 2, 3, 4, 5];

  println!("Please enter an array index.");

  let mut index = String::new();

  io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

  let index: usize = index
    .trim()
    .parse()
    .expect("Index entered was not a number");

  let element = a[index];

  println!("The value of the element at index {index} is: {element}");
}

fn main2() {
  //shadow();
  //nums();
  // tuple
  let usr = ("fatcat", 23,'😸');
  println!("{} {}: score:{}", usr.2, usr.0, usr.1);
  // array
  let a = [1,2,3,4,5];
  println!("a[2]:{}", a[2]);
}

// inner shadowing in inner scope
fn shadow() {
  let x = 5;
  let x = x + 1;
  {
    let x = x * 2;
    println!("inner x:{x}");
  }
  println!("x:{x}");
}

fn nums() {
  let a : i32= 1_001;  // 1001
  let b = 0xff;        //  255
  let c = 0o77;        //   63
  let d = 0b1111_0000; //  240
  let e = b'A' as i32; //   65
  println!("byte:{e}");// 1624
  let x = a + b + c + d + e;
  println!("ans:{x}");

  let quotient = 56.7 / 32.2;
  println!("quotient:{quotient}");
}