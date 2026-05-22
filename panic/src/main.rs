use std::io;
fn main() {
  // ans
  let ans: u32 = 3;
  loop {
    println!("Guess number between 1 and 10");
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Expect to read line");

    // verify guess is a number
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Parse error: {}", guess);
        continue;
      }
    };
    

    if guess == ans {
      println!("You win!");
      break;
    }
    else {
      println!("Try again!");
    }
  }
}

// fn main() {
//   let v = vec![1, 2, 4, 8];
//   println!("{}", v[7]); // index out of bounds
// }

// fn main_panic() {
//   println!("hello world");
//   panic!("Test panic");
//   println!("goodbye");
// }