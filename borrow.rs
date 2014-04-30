struct Book {
  author: ~str,
  title: ~str,
  year: uint,
}

// & means a mutable reference, meaning the fields of the struct can be
// modified
fn borrow_book(book: &Book) {
  println!("I borrowed {} {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book) {
  book.year = 2014;
}

fn main() {
  let geb = Book {
    author: ~"Douglas Hofstadter",
    title: ~"Godel, Escher, Bach",
    year: 1979,
  };

  // borrow geb, geb can still be used afterwards
  borrow_book(&geb);

  // geb can be borrowed again and again, and again, and again ...
  borrow_book(&geb);

  // a mutable book
  let mut mutable_geb = Book {
    author: ~"Douglas Hofstadter",
    title: ~"Godel, Escher, Bach",
    year: 1979,
  };

  // Error: can't borrow immutable object as mutable
  //new_edition(&mut geb);

  // OK: borrow a mutable object as mutable
  borrow_book(&mut mutable_geb);

  // OK: mutable objects can be immediately borrowed
  borrow_book(&mutable_geb);

  // when an object is mutable borrowed, the original can't be used until
  // its mutable reference goes out of scope
  if true {
    let borrwed_geb = &mut mutable_geb;

    // Error: mutable_geb has been mutably borrowed
    //preintln!("Can no longer access {}", mutable_geb.title);

    println!("The mutable reference of {} is available",
             borrwed_geb.title);

    // now borrwed_geb goes out of scope
  }

  println!("Once again, I can access {}", mutable_geb.title);

  // immutable borrows place no restrictions on the original owner
  if true {
    let borrowed_geb = &mutable_geb;
    println!("The original is still accessible: {}",
             mutable_geb.title);
    println!("and so is the original reference: {}",
             borrowed_geb.title);
  }

  // ref can be used to take references when destructuring
  // here geb_title and geb_author are references to ~str, i.e. &~str
  // geb_year is a copy of year field
  let Book {
    author: ref geb_author,
    title: ref geb_title,
    year: geb_year,
  } = geb;
}
