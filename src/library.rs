pub mod library {
    pub struct Books {
        name: String,
        author_name: String,
        is_available: bool
    }

    impl Books {
        pub fn new(name: String, author_name: String) -> Self {
            Books { name, author_name, is_available: true }
        }

        pub fn name(&self) -> &String {
            &self.name
        }

        pub fn is_available(&self) -> bool {
            self.is_available
        }

        pub fn borrow(&mut self) {
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

        pub fn borrow_book(&mut self, name: &String) {
            for book in self.books.iter_mut() {
                if book.name() == name && book.is_available() {
                    book.borrow();
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
    }
}