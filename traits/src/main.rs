// pub struct Person {
//   name: String,
// }

// pub struct Cat {
//   Name: String,
// }

// pub struct Rabbit {
//   Name: String,
// }

// pub trait Eat {
//   fn eat_dinner(&self) {
//     // default behaviour
//     println!("I eat from a dish")
//   }
// }

// impl Eat for Person {
//   fn eat_dinner(&self) {
//     println!("I eat from plate")
//   }
// }

// impl Eat for Cat {
//   fn eat_dinner(&self) {
//     println!("I eat from a cat bowl")
//   }
// }

// impl Eat for Rabbit {}

// fn main() {
//   let person = Person {
//     name: "Nell".to_string(),
//   };
//   person.eat_dinner();

//   let cat = Cat {
//     Name: "Zane".to_string(),
//   };
//   cat.eat_dinner();

//   let rabbit = Rabbit {
//     Name: "Zane".to_string(),
//   };
//   rabbit.eat_dinner();
// }

struct Film {
  title: String,
  director: String,
  studio: String,
}

struct Book {
  title: String,
  author: String,
  publisher: String,
}

trait Catalog {
  fn describe(&self) {
    println!("We need more information about this type of media")
  }
}

impl Catalog for Film {
  fn describe(&self) {
    println!(
      "{} was directed by {} through {} studios",
      self.title, self.director, self.studio
    )
  }
}

impl Catalog for Book {
  fn describe(&self) {
    println!(
      "{} was written by {} and published by {}",
      self.title, self.author, self.publisher
    )
  }
}

struct Album {
  title: String,
  artist: String,
  label: String,
}

impl Catalog for Album {}

fn main() {
  let capt_marvel = Film {
    title: String::from("Captain Marvel"),
    director: String::from("Anna Boden and Ryan Fleck"),
    studio: String::from("Marvel"),
  };
  capt_marvel.describe();

  let elantris = Book {
    title: String::from("Elantris"),
    author: String::from("Brandon Sanderson"),
    publisher: String::from("Tor Books"),
  };
  elantris.describe();

  let let_it_be = Album {
    title: String::from("Let it be"),
    artist: String::from("Beatles"),
    label: String::from("Apple"),
  };
  let_it_be.describe();
}
