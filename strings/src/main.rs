fn main() {
  let text = "Hello\nworld\n!";
  let upper = text.to_uppercase();
  let stripped = upper.strip_prefix("HELLO\n").unwrap();

  // // println!("{}", first_line(String::from(text)));
  // println!("{}", first_line(text));
  println!("{}", first_line(stripped));
}

// fn first_line(string: String) -> String {
//   string.lines().next().unwrap().to_owned()
// }

fn first_line(string: &str) -> &str {
  string.lines().next().unwrap()
}
