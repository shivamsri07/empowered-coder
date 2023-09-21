mod library;
mod person;

use library::library::{Books, Library};
use person::person::{Person, PersonImpl};

fn main() {
    let mut library = Library::new();
    let _person = PersonImpl::new(String::from("Shivam"), 24);
    let book = Books::new(String::from("The Lord of the Rings"), String::from("J. R. R. Tolkien"));
    library.add_book(book);
    library.borrow_book(&String::from("The Lord of the Rings"));
    library.return_book(&String::from("The Lord of the Rings"));
}