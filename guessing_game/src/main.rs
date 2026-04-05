use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
  let random_num = rand::thread_rng()
    .gen_range(1..=100);
  println!("Guess the, number (0 - 100):");

  loop {
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Guess must be a number; guess again:");
        continue;
      }
    };

    match guess.cmp(&random_num) {
      Ordering::Less => println!("Too small; guess again:"),
      Ordering::Greater => println!("Too big; guess again:"),
      Ordering::Equal => {
          println!("You win! Game over.");
          break;
      }
    }
  }
}