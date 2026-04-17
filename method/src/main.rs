fn main() {
  let rect = Rect {
    width: 30,
    height: 50
  };
  println!("rect area:{}", rect.area());

  // associated function
  println!("square area:{}", Rect::square(30).area());

  route(Ip::V4);

  let coin = Coin::Nickel;
  println!("coin: {coin:?}");
  let c = cents(coin);
  println!("cents: {c}");

  let result = plus_one(Some(5));
  println!("result: {result:?}");

  let roll = 7;
  match roll {
    3 => println!("three"),
    7 => println!("seven"),
    _ => println!("other")
  }

  let state = State::OK;
  println!("{:?} existed in 1900: {}", state, state.existed_in(1920));
}

#[derive(Debug)]
enum State {
  AL, AK, AZ, AR, CA, OK, TX, NY
}

impl State {
  fn existed_in(&self, year:u32) -> bool {
    match self {
      State::AL => year >= 1819,
      State::AK => year >= 1959,
      State::AZ => year >= 1912,
      State::AR => year >= 1836,
      State::CA => year >= 1850,
      State::OK => year >= 1907,
      State::TX => year >= 1845,
      State::NY => year >= 1788
    }
  }
}

fn plus_one(x:Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1)
  }
}

#[derive(Debug)]
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter
}

fn cents(coin:Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25
  }
}

enum Ip { V4, V6 }

fn route(ip:Ip) {
  let num = match ip {
    Ip::V4 => 4,
    Ip::V6 => 6
  };
  println!("v{}", num);
}

struct Rect {
  width: u32,
  height: u32
}

impl Rect {
  fn square(len:u32) -> Rect {
    Rect {
      width:len,
      height:len
    }
  }

  fn area(&self) -> u32 {
    self.width * self.height
  }
}
