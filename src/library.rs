pub mod library {
    pub struct Books {
        name: String,
        author_name: String,
        is_available: bool,
        borrowed_by: Option<String>
    }

    impl Books {
        pub fn new(name: String, author_name: String) -> Self {
            Books { name, author_name, is_available: true, borrowed_by: None}
        }

        pub fn name(&self) -> &String {
            &self.name
        }

        pub fn is_available(&self) -> bool {
            self.is_available
        }

        pub fn borrow(&mut self, borrowed_by: String) {
            self.borrowed_by = Some(borrowed_by);
            self.is_available = false;
        }

        pub fn return_book(&mut self) {
            self.is_available = true;
        }
    }

    pub struct Library {
        books: Vec<Books>
    }

    impl Library {
        pub fn new() -> Self {
            Library { books: Vec::new() }
        }

        pub fn add_book(&mut self, book: Books) {
            self.books.push(book);
        }

        pub fn borrow_book(&mut self, name: &str, borrowed_by: &str) {
            for book in self.books.iter_mut() {
                if book.name() == name && book.is_available() {
                    book.borrow(borrowed_by.to_string());
                    println!("{} is borrowed!", book.name())
                }
            }
        }

        pub fn return_book(&mut self, name: &String) {
            for book in self.books.iter_mut() {
                if book.name() == name && !book.is_available() {
                    book.return_book();
                    println!("{} is returned!", book.name())
                }
            }
        }

        pub fn list_checkout_books(&self) {
            println!("List of checked out books:");
            self.books.iter().filter(|book| !book.is_available)
            .for_each(|book| println!("Book :: {} | Author :: {}", book.name, book.author_name));
        }
    
        pub fn list_available_books(&self) {
            println!("List of available books:");
           self.books.iter().filter(|book| book.is_available).for_each(|book| println!("Book :: {} | Author :: {}", book.name, book.author_name));
        }
    }
}