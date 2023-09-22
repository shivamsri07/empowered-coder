mod library;
mod person;

use library::library::{Books, Library};
use person::Person;

fn main() {
    let mut library = Library::new();
    let person = Person::new(String::from("Shivam"), 24);
    person.display();

    let book = Books::new(String::from("The Lord of the Rings"), String::from("J. R. R. Tolkien"));
    let book2: Books = Books::new("Harry Potter and the Chambers of Secret".to_string(), "J. K Rowling".to_string());
    library.add_book(book);
    library.add_book(book2);

    library.list_available_books();
    library.borrow_book(&"The Lord of the Rings", &person.name);

    library.list_checkout_books();
    library.return_book(&String::from("The Lord of the Rings"));

    library.list_available_books()
}