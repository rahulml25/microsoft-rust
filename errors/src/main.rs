use std::fs::File;

fn main() {
  // let v = vec![0, 1, 2, 3];
  // println!("{}", v[6]);

  // let fruits = vec!["banana", "apple", "coconut"];

  // let first = fruits.get(0);
  // println!("{:?}", first);

  // let third = fruits.get(2);
  // println!("{:?}", third);

  // let non_existant = fruits.get(99);
  // println!("{:?}", non_existant);

  let f = File::open("data/hello.txt")
    // .unwrap();
    .expect("Failed to open hello.txt");

  // let f = match f {
  //   Ok(file) => file,
  //   Err(error) => panic!("Can't open the file: {:?}", error),
  // };
}
