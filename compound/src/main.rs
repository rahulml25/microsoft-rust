fn main() {
  let array: [u32; 3] = [1u32, 2, 3];
  let first_element = array[0];
  // let elemrnt = array[100];

  let len = "Some text".len();
  // [1][len];

  // let array: [u32; 3] = [1u32, 2, true];

  let tuple: (u32, u8, bool) = (1u32, 1, true);
  let first_element = tuple.0;
  let element = tuple.100;
}
