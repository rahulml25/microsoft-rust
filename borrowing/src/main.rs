fn main() {
  // let name = String::from("Rahul");
  // print_name(&name);
  // print_name(&name);

  // let username = &name;
  // drop(&name);
  // print_name(&username)

  // let name = &String::from("Rahul");
  // print_name(name);
  // print_name(name);

  // let username = &name;
  // drop(name);
  // print_name(username)

  // let mut my_vec = vec![1, 2, 3];
  // println!("{:?}", my_vec);

  // add_to_vec(&mut my_vec);
  // println!("{:?}", my_vec);

  // // add_to_vec(&mut my_vec, &mut my_vec);
  // // println!("{:?}", my_vec);

  let say = String::from("Cat");
  let say2 = &say;

  println!("{}", say);
  // drop(say);
  // println!("{}", say2);
}

// fn print_name(name: &String) {
//   println!("The name of the user is: {name}")
// }

fn add_to_vec(a_vec: &mut Vec<i32>) {
  a_vec.push(4);
}

// fn add_to_vec(a_vec: &mut Vec<i32>, b_vec: &mut Vec<i32>) {
//   a_vec.push(4);
// }
