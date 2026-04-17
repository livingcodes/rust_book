fn main() {
  let rect = Rect {
    width: 30,
    height: 50
  };
  println!("area:{}", rect.area());
}

struct Rect {
  width: u32,
  height: u32
}

impl Rect {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}