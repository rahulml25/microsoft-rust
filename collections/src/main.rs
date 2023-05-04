fn main() {
  let mut students = vec![Student {
    name: "Ryan".to_string(),
  }];
  students.push(Student {
    name: "Lisa".to_string(),
  });

  assert!(
    &students[0]
      == &Student {
        name: "Ryan".to_string(),
      }
  );

  assert!(
    students.get(0)
      == Some(&Student {
        name: "Ryan".to_string()
      })
  );

  assert!(students.get(1000) == None);

  for student in students.iter() {
    println!("Student name: {}", student.name);
  }

  use std::collections::HashMap;

  let mut enrollment = HashMap::new();
  enrollment.insert("biology".to_string(), students);

  let biology_students = enrollment.get("biology");
  let students = enrollment.remove("biology");
}

#[derive(PartialEq, Eq)]
struct Student {
  name: String,
}
