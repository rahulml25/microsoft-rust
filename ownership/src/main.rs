fn main() {
  // let name = String::from("rahul");
  // // print_name(name);
  // // print_name(name);

  // // let username = name;
  // // drop(name);

  let name = String::from("rahul");

  print_name(name.clone());
  print_name(name.clone());

  let username = name.clone();
  drop(name);

  print_name(username);
}

fn print_name(name: String) {
  println!("The name of the user is: {name}")
}
