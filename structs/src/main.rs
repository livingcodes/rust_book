fn main() {
  let mut user1 = build_user(String::from("tom"), String::from("tom@bot.com"));
  user1.sign_in_count += 1;
  println!("hello {} {}x", &user1.username, user1.sign_in_count);
}

fn build_user(email:String, username:String) -> User {
  return User {
    active: true,
    username,
    email,
    sign_in_count: 1
  }
}

struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64
}