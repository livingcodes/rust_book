fn main() {
  let y = {
    let x = 3;
    x + 1
  };

  println!("y:{y}");

  println!("five():{}", five());
}

fn five() -> i32 {
  5
}